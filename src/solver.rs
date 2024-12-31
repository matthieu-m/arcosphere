//! Core solving logic, with modular setup.

use core::{cmp::Reverse, error, fmt, iter, num::NonZeroU8};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    executor::Executor,
    model::{Arcosphere, FoldingRecipe, InversionRecipe, Path, Polarity, Recipe, RecipeSet, Set},
    space_exploration::SeRecipeSet,
};

/// Error which may occur during the search for a solution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolutionError {
    /// There is no solution, as the number of arcospheres is not preserved.
    PreservationError,
    /// There is no solution for the provided set of foldings.
    NotWithFoldings,
    /// There is no solution for the provided set of inversions.
    NotWithInversions,
    /// There is no solution for the given range of number of catalysts.
    OutsideCatalysts,
    /// There is no solution for the given range of number of inversions.
    OutsideInversions,
    /// There is no solution for the given range of number of recipes.
    OutsideRecipes,
}

impl ResolutionError {
    /// Returns whether the error is definitive.
    ///
    /// An error is definitive if the supplied recipes simply do not permit solving the problem, while it is not if
    /// there exists a possibility, however remote, that increasing the search space would allow finding a solution.
    pub fn is_definitive(&self) -> bool {
        matches!(
            self,
            Self::PreservationError | Self::NotWithFoldings | Self::NotWithInversions
        )
    }
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for ResolutionError {}

/// Configuration of the solver.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SolverConfiguration {
    /// The maximum number of catalysts to add.
    pub maximum_catalysts: u8,
    /// The minimum number of catalysts to add.
    pub minimum_catalysts: u8,
    /// The maximum number of recipes in the path from source to target.
    pub maximum_recipes: u8,
}

impl Default for SolverConfiguration {
    fn default() -> Self {
        //  Sufficient for all SE recipes.
        let maximum_catalysts = 8;
        let minimum_catalysts = 0;
        let maximum_recipes = 10;

        Self {
            maximum_catalysts,
            minimum_catalysts,
            maximum_recipes,
        }
    }
}

/// Solver.
#[derive(Clone, Debug, Default)]
pub struct Solver<R, E>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    recipes: R,
    executor: E,
    configuration: SolverConfiguration,
}

//
//  Configuration
//

impl<R, E> Solver<R, E>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    /// Creates a new solver based on a set of recipes.
    pub fn new(recipes: R) -> Self
    where
        E: Default,
    {
        let executor = E::default();
        let configuration = SolverConfiguration::default();

        Self {
            recipes,
            executor,
            configuration,
        }
    }

    /// Sets the configuration.
    pub fn with_configuration(mut self, configuration: SolverConfiguration) -> Self {
        self.configuration = configuration;

        self
    }

    /// Sets the executor.
    pub fn with_executor<OE>(self, executor: OE) -> Solver<R, OE> {
        let Solver {
            recipes, configuration, ..
        } = self;

        Solver {
            recipes,
            executor,
            configuration,
        }
    }
}

//
//  Space exploration short-hand.
//

/// Solver for default Space Exploration.
pub type SeSolver<E> = Solver<SeRecipeSet, E>;

impl<E> SeSolver<E> {
    /// Creates a new solver for space exploration.
    pub fn space_exploration() -> Self
    where
        E: Default,
    {
        Self::default()
    }
}

//
//  Solving!
//

impl<R, E> Solver<R, E>
where
    R: RecipeSet<Arcosphere: Send> + Clone + Send,
    E: Executor,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    /// Looks for all possible recipe paths from `source` to `target` with a minimum number of catalysts.
    ///
    /// If the solver return a set of solutions, then it is guaranted no solution exists with a smaller number of
    /// catalysts up to the given number of recipes.
    ///
    /// If the solver does not return any solution, then raising either the number of catalysts or the number of recipes
    /// may allow it to find further solutions.
    pub fn solve(
        &self,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
    ) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        //  Special case: impossible.

        if source.len() != target.len() {
            return Err(ResolutionError::PreservationError);
        }

        //  Special case: 0 conversion.

        if source == target {
            return Ok(vec![Path {
                source,
                target,
                count: ONE,
                catalysts: Set::new(),
                recipes: Vec::new(),
            }]);
        }

        //  Is an inversion required, or not?

        if source.count_negatives() == target.count_negatives() {
            self.solve_by_fold(source, target)
        } else {
            self.solve_by_inversion(source, target)
        }
    }
}

//
//  Implementation
//

const ONE: NonZeroU8 = NonZeroU8::new(1).unwrap();

impl<R, E> Solver<R, E>
where
    R: RecipeSet<Arcosphere: Send> + Clone + Send,
    E: Executor,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn solve_by_inversion(
        &self,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
    ) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        debug_assert_eq!(source.len(), target.len());
        debug_assert_ne!(source.count_negatives(), target.count_negatives());

        //  Special case: 1 conversion.

        for inversion in self.recipes.inversions() {
            if source != inversion.input() || target != inversion.output() {
                continue;
            }

            return Ok(vec![Path {
                source,
                target,
                count: ONE,
                catalysts: Set::new(),
                recipes: vec![Recipe::Inversion(inversion)],
            }]);
        }

        //  Determine the inversion to use.

        let Some((count, inversion, apps)) = InversionSearcher::select_inversion(&self.recipes, source, target) else {
            return Err(ResolutionError::NotWithInversions);
        };

        //  FIXME: may want to handle that one day... though it massively increases the search space.
        if apps.get() != 1 {
            return Err(ResolutionError::OutsideInversions);
        }

        //  Special case: inversion is first.

        if inversion.input().is_subset_of(&(source * count)) {
            let post_inversion = source * count - inversion.input() + inversion.output();

            if let Ok(mut paths) = self.solve_by_fold(post_inversion, target * count) {
                paths.iter_mut().for_each(|p| {
                    p.source = source;
                    p.target = target;
                    p.count = count;
                    p.recipes.insert(0, Recipe::Inversion(inversion));
                });

                return Ok(paths);
            }
        }

        //  Special case: inversion is last.

        if inversion.output().is_subset_of(&(target * count)) {
            let pre_inversion = target * count - inversion.output() + inversion.input();

            if let Ok(mut paths) = self.solve_by_fold(source * count, pre_inversion) {
                paths.iter_mut().for_each(|p| {
                    p.source = source;
                    p.target = target;
                    p.count = count;
                    p.recipes.push(Recipe::Inversion(inversion));
                });

                return Ok(paths);
            }
        }

        self.solve_by_middle_inversion(source, target, count, inversion)
    }

    fn solve_by_middle_inversion(
        &self,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        count: NonZeroU8,
        inversion: InversionRecipe<R::Arcosphere>,
    ) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        let configuration = self.configuration.into();

        let mut last_error = None;

        for i in self.configuration.minimum_catalysts..=self.configuration.maximum_catalysts {
            let searchers = InversionSearcher::generate_searchers(
                &self.recipes,
                source,
                target,
                count,
                inversion,
                i as usize,
                configuration,
            );

            let tasks: Vec<_> = searchers.into_iter().map(|searcher| move || searcher.solve()).collect();

            let mut results = Vec::new();

            for result in self.executor.execute(tasks) {
                match result {
                    Ok(paths) => results.extend(paths),
                    Err(e) if e.is_definitive() => last_error = Some(e),
                    Err(e) if e == ResolutionError::OutsideRecipes => last_error = Some(e),
                    _ => (),
                }
            }

            if !results.is_empty() {
                return Ok(results);
            }
        }

        //  Didn't find anything, it may be necessary to raise the number of catalysts or the number of recipes in a
        //  path.
        Err(last_error.unwrap_or(ResolutionError::OutsideCatalysts))
    }

    fn solve_by_fold(
        &self,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
    ) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        debug_assert_eq!(source.len(), target.len());
        debug_assert_eq!(source.count_negatives(), target.count_negatives());

        //  Special case: 1 conversion.

        for folding in self.recipes.foldings() {
            if source != folding.input() || target != folding.output() {
                continue;
            }

            return Ok(vec![Path {
                source,
                target,
                count: ONE,
                catalysts: Set::new(),
                recipes: vec![Recipe::Folding(folding)],
            }]);
        }

        //  From then on, it gets a tad more complicated.

        let configuration = self.configuration.into();

        let mut last_error = None;

        for i in self.configuration.minimum_catalysts..=self.configuration.maximum_catalysts {
            let searchers = FoldSearcher::generate_searchers(&self.recipes, source, target, i as usize, configuration);

            let tasks: Vec<_> = searchers.into_iter().map(|searcher| move || searcher.solve()).collect();

            let mut results = Vec::new();

            for result in self.executor.execute(tasks) {
                match result {
                    Ok(paths) => results.extend(paths),
                    Err(e) if e.is_definitive() => last_error = Some(e),
                    Err(e) if e == ResolutionError::OutsideRecipes => last_error = Some(e),
                    _ => (),
                }
            }

            if !results.is_empty() {
                return Ok(results);
            }
        }

        //  Didn't find anything, it may be necessary to raise the number of catalysts or the number of recipes in a
        //  path.
        Err(last_error.unwrap_or(ResolutionError::OutsideCatalysts))
    }
}

#[derive(Clone, Copy, Debug)]
struct SearcherConfiguration {
    maximum_recipes: u8,
}

impl From<SolverConfiguration> for SearcherConfiguration {
    fn from(value: SolverConfiguration) -> SearcherConfiguration {
        let SolverConfiguration { maximum_recipes, .. } = value;

        SearcherConfiguration { maximum_recipes }
    }
}

struct InversionSearcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    source: Set<R::Arcosphere>,
    target: Set<R::Arcosphere>,
    count: NonZeroU8,
    //  assert_eq!(pre.source_catalysts, post.target_catalysts);
    //  assert_eq!(pre.target_catalysts, post.source_catalysts);
    pre: FoldSearcher<R>,
    inversion: InversionRecipe<R::Arcosphere>,
    post: FoldSearcher<R>,
}

impl<R> InversionSearcher<R>
where
    R: RecipeSet + Clone,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    //  Selects the inversion to use, and returns the count of sources & targets (and inversions) to apply.
    //
    //  Returns None if no inversion can be found which fits.
    fn select_inversion(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
    ) -> Option<(NonZeroU8, InversionRecipe<R::Arcosphere>, NonZeroU8)> {
        //  Determine the direction in which to invert.
        let polarity = if source.count_negatives() > target.count_negatives() {
            Polarity::Positive
        } else {
            Polarity::Negative
        };

        //  Determine which inversion to use.
        //
        //  Note: for Space Exploration a single inversion exists, should multiple inversions be considered?
        let inversion = recipes.inversions().into_iter().find(|r| r.polarity() == polarity)?;

        //  Determine the number of spheres flipped by a source -> target conversion, and by a recipe application.
        let (target_flipped, recipe_flipped) = match polarity {
            Polarity::Positive => {
                let target_flipped = target.count_positives() - source.count_positives();
                let recipe_flipped = inversion.output().count_positives() - inversion.input().count_positives();

                (target_flipped, recipe_flipped)
            }
            Polarity::Negative => {
                let target_flipped = target.count_negatives() - source.count_negatives();
                let recipe_flipped = inversion.output().count_negatives() - inversion.input().count_negatives();

                (target_flipped, recipe_flipped)
            }
        };

        if target_flipped == recipe_flipped {
            let one = NonZeroU8::new(1).unwrap();

            return Some((one, inversion, one));
        }

        //  If the number of flipped arcospheres doesn't quite match, use the GCD to determine how to make it work.
        let gcd = num_integer::gcd(target_flipped, recipe_flipped);

        let count = recipe_flipped / gcd;
        let apps = target_flipped / gcd;

        let count = NonZeroU8::new(count.try_into().expect("sufficiently small count")).expect("non-zero count");
        let apps = NonZeroU8::new(apps.try_into().expect("sufficiently small apps")).expect("non-zero apps");

        Some((count, inversion, apps))
    }

    fn generate_searchers(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        count: NonZeroU8,
        inversion: InversionRecipe<R::Arcosphere>,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
    ) -> Vec<InversionSearcher<R>> {
        let catalysts = FoldSearcher::generate_catalysts(number_catalysts);

        let combinations = Self::generate_catalysts_combinations(&catalysts);

        let pre_inversions = Self::generate_candidate_pre_inversions(source, count, inversion);

        combinations
            .iter()
            .flat_map(|&(start, middle)| {
                pre_inversions.iter().map(move |&pre_inversion| {
                    let post_inversion = pre_inversion - inversion.input() + inversion.output();

                    let pre = FoldSearcher {
                        recipes: recipes.clone(),
                        source: source * count,
                        target: pre_inversion,
                        source_catalysts: start,
                        target_catalysts: middle,
                        configuration,
                    };

                    let post = FoldSearcher {
                        recipes: recipes.clone(),
                        source: post_inversion,
                        target: target * count,
                        source_catalysts: middle,
                        target_catalysts: start,
                        configuration,
                    };

                    InversionSearcher {
                        source,
                        target,
                        count,
                        pre,
                        inversion,
                        post,
                    }
                })
            })
            .collect()
    }

    #[expect(clippy::type_complexity)]
    fn generate_catalysts_combinations(
        catalysts: &[Set<R::Arcosphere>],
    ) -> Vec<(Set<R::Arcosphere>, Set<R::Arcosphere>)> {
        //  Not all combinations are valid, as the polarities of the catalysts never change.

        debug_assert!(catalysts.windows(2).all(|w| w[0].len() == w[1].len()));

        let mut result = Vec::new();

        for select in catalysts {
            for candidate in catalysts {
                if select.count_negatives() == candidate.count_negatives() {
                    result.push((*select, *candidate));
                }
            }
        }

        result
    }

    fn generate_candidate_pre_inversions(
        source: Set<R::Arcosphere>,
        count: NonZeroU8,
        inversion: InversionRecipe<R::Arcosphere>,
    ) -> Vec<Set<R::Arcosphere>> {
        //  Not all middle steps combinations are valid as:
        //
        //  -   The middle step must include the input of the inversion.
        //  -   Since only foldings are made on source * count until the middle step, the middle step polarities must
        //      match the source * count polarities.

        let seed = inversion.input();

        let negatives = Set::<R::Arcosphere>::full().negatives();
        let positives = Set::<R::Arcosphere>::full().positives();

        let remainder = source * count - seed;
        let number_negatives = remainder.count_negatives();
        let number_positives = remainder.count_positives();

        let mut result = Vec::new();

        match (number_negatives, number_positives) {
            (0, 0) => result.push(seed),
            (n, 0) => {
                Self::generate_candidate_pre_inversions_rec(negatives, seed, n, &mut result);
            }
            (0, n) => {
                Self::generate_candidate_pre_inversions_rec(positives, seed, n, &mut result);
            }
            (_, _) => {
                let mut temporaries = Vec::new();
                Self::generate_candidate_pre_inversions_rec(negatives, seed, number_negatives, &mut temporaries);

                for t in temporaries {
                    Self::generate_candidate_pre_inversions_rec(positives, t, number_positives, &mut result);
                }
            }
        }

        //  The lengths & polarities should match.
        #[cfg(debug_assertions)]
        {
            let source = source * count;
            let number_negatives = source.count_negatives();
            let number_positives = source.count_positives();

            debug_assert!(result.iter().all(|r| r.len() == source.len()));
            debug_assert!(result.iter().all(|r| r.count_negatives() == number_negatives));
            debug_assert!(result.iter().all(|r| r.count_positives() == number_positives));
        }

        result
    }

    fn generate_candidate_pre_inversions_rec(
        seeds: Set<R::Arcosphere>,
        current: Set<R::Arcosphere>,
        number: usize,
        output: &mut Vec<Set<R::Arcosphere>>,
    ) {
        debug_assert!(number > 0);

        let generator = seeds.into_iter().map(|i| {
            let mut current = current;

            current.insert(i);

            current
        });

        if number == 1 {
            output.extend(generator);
            return;
        }

        for candidate in generator {
            Self::generate_candidate_pre_inversions_rec(seeds, candidate, number - 1, output);
        }
    }
}

impl<R> InversionSearcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn solve(&self) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        //  1.  Solve the source -> inversion part, via folding.
        let pre_inversions = self.pre.solve()?;

        //  2.  Solve the inversion -> target part, via folding.
        let post_inversions = self.post.solve()?;

        //  Banzai! Any combination of pre & post is a solution, so we need to combine them, while inserting the
        //  insertion in the middle.

        let inversion = Recipe::Inversion(self.inversion);

        Ok(pre_inversions
            .iter()
            .flat_map(|pre_inversion| {
                post_inversions.iter().map(|post_inversion| {
                    let recipes: Vec<_> = pre_inversion
                        .recipes
                        .iter()
                        .copied()
                        .chain(iter::once(inversion))
                        .chain(post_inversion.recipes.iter().copied())
                        .collect();

                    Path {
                        source: self.source,
                        target: self.target,
                        count: self.count,
                        catalysts: pre_inversion.catalysts,
                        recipes,
                    }
                })
            })
            .collect())
    }
}

struct FoldSearcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    recipes: R,
    source: Set<R::Arcosphere>,
    target: Set<R::Arcosphere>,
    source_catalysts: Set<R::Arcosphere>,
    target_catalysts: Set<R::Arcosphere>,
    configuration: SearcherConfiguration,
}

impl<R> FoldSearcher<R>
where
    R: RecipeSet + Clone,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn generate_catalysts(number: usize) -> Vec<Set<R::Arcosphere>> {
        let mut result = Vec::new();

        if number == 0 {
            return result;
        }

        Self::generate_catalysts_rec(Set::new(), number, &mut result);

        result
    }

    fn generate_catalysts_rec(catalysts: Set<R::Arcosphere>, number: usize, output: &mut Vec<Set<R::Arcosphere>>) {
        debug_assert!(number > 0);

        let generator = (0..R::Arcosphere::DIMENSION).map(|i| {
            let mut catalysts = catalysts;

            catalysts.insert(R::Arcosphere::from_index(i));

            catalysts
        });

        if number == 1 {
            output.extend(generator);
            return;
        }

        for catalysts in generator {
            Self::generate_catalysts_rec(catalysts, number - 1, output);
        }
    }

    fn generate_searchers(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
    ) -> Vec<FoldSearcher<R>> {
        let catalysts = Self::generate_catalysts(number_catalysts);

        catalysts
            .into_iter()
            .map(|catalysts| FoldSearcher {
                recipes: recipes.clone(),
                source,
                target,
                source_catalysts: catalysts,
                target_catalysts: catalysts,
                configuration,
            })
            .collect()
    }
}

impl<R> FoldSearcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn solve(&self) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        assert_eq!(self.source.count_negatives(), self.target.count_negatives());
        assert_eq!(self.source.count_positives(), self.target.count_positives());

        let maximum_iterations = (self.configuration.maximum_recipes as usize + 1) / 2;

        let source = self.source + self.source_catalysts;
        let target = self.target + self.target_catalysts;

        let mut forward = FxHashMap::default();
        let mut backward = FxHashMap::default();

        let mut in_forward = FxHashSet::from_iter([source]);
        let mut in_backward = FxHashSet::from_iter([target]);

        let mut out_forward = FxHashMap::default();
        let mut out_backward = FxHashMap::default();

        for _ in 0..maximum_iterations {
            if in_forward.is_empty() && in_backward.is_empty() {
                return Err(ResolutionError::NotWithFoldings);
            }

            let searcher = fold_searcher::ForwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut forward, &mut in_forward, &mut out_forward, &backward) {
                return Ok(self.stitch(&forward, &backward, out_forward.keys().copied()));
            }

            let searcher = fold_searcher::BackwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut backward, &mut in_backward, &mut out_backward, &forward) {
                return Ok(self.stitch(&forward, &backward, out_backward.keys().copied()));
            }
        }

        if in_forward.is_empty() && in_backward.is_empty() {
            return Err(ResolutionError::NotWithFoldings);
        }

        Err(ResolutionError::OutsideRecipes)
    }

    //  Returns true if a connection has been found.
    fn advance<S, OF>(
        searcher: &S,
        known: &mut FxHashMap<Set<R::Arcosphere>, S::Folding>,
        inputs: &mut FxHashSet<Set<R::Arcosphere>>,
        outputs: &mut FxHashMap<Set<R::Arcosphere>, S::Folding>,
        opposite_known: &FxHashMap<Set<R::Arcosphere>, OF>,
    ) -> bool
    where
        S: fold_searcher::DirectionSearcher<Set = Set<R::Arcosphere>>,
    {
        searcher.fold(known, inputs, outputs);

        inputs.clear();
        inputs.extend(outputs.keys().copied());

        known.extend(outputs.iter().map(|(&key, &value)| (key, value)));

        outputs.keys().any(|key| opposite_known.contains_key(key))
    }

    fn stitch<C>(
        &self,
        forward: &FxHashMap<Set<R::Arcosphere>, FoldingRecipe<R::Arcosphere>>,
        backward: &FxHashMap<Set<R::Arcosphere>, Reverse<FoldingRecipe<R::Arcosphere>>>,
        candidates: C,
    ) -> Vec<Path<R::Arcosphere>>
    where
        C: IntoIterator<Item = Set<R::Arcosphere>>,
    {
        let mut result = Vec::new();

        for candidate in candidates {
            if !forward.contains_key(&candidate) || !backward.contains_key(&candidate) {
                continue;
            }

            let mut recipes = Vec::new();

            Self::stitch_forward(self.source + self.source_catalysts, forward, candidate, &mut recipes);
            Self::stitch_backward(self.target + self.target_catalysts, backward, candidate, &mut recipes);

            result.push(Path {
                source: self.source,
                target: self.target,
                count: ONE,
                catalysts: self.source_catalysts,
                recipes,
            })
        }

        debug_assert_ne!(0, result.len());

        result
    }

    fn stitch_forward(
        _source: Set<R::Arcosphere>,
        forward: &FxHashMap<Set<R::Arcosphere>, FoldingRecipe<R::Arcosphere>>,
        candidate: Set<R::Arcosphere>,
        recipes: &mut Vec<Recipe<R::Arcosphere>>,
    ) {
        let mut step = candidate;

        while let Some(folding) = forward.get(&step) {
            step = step - folding.output() + folding.input();

            recipes.push(Recipe::Folding(*folding));
        }

        debug_assert_eq!(_source, step);

        recipes.reverse();
    }

    fn stitch_backward(
        _target: Set<R::Arcosphere>,
        backward: &FxHashMap<Set<R::Arcosphere>, Reverse<FoldingRecipe<R::Arcosphere>>>,
        candidate: Set<R::Arcosphere>,
        recipes: &mut Vec<Recipe<R::Arcosphere>>,
    ) {
        let mut step = candidate;

        while let Some(Reverse(folding)) = backward.get(&step) {
            step = step - folding.input() + folding.output();

            recipes.push(Recipe::Folding(*folding));
        }

        debug_assert_eq!(_target, step);
    }
}

mod fold_searcher {
    use core::{
        cmp::Eq,
        fmt::Debug,
        hash::Hash,
        ops::{Add, Sub},
    };

    use crate::model::IsSubsetOf;

    use super::*;

    pub(super) trait DirectionSearcher {
        //  The set of arcospheres to use.
        type Set: Copy + Eq + Hash + IsSubsetOf + Add<Output = Self::Set> + Sub<Output = Self::Set>;

        //  The folding recipe to use.
        type Folding: Copy + Debug;

        #[allow(dead_code)]
        fn direction(&self) -> &'static str;

        fn all_foldings(&self) -> impl Iterator<Item = Self::Folding>;

        fn extract_folding(&self, folding: Self::Folding) -> (Self::Set, Self::Set);

        //  Never overriden.
        fn fold(
            &self,
            known: &FxHashMap<Self::Set, Self::Folding>,
            inputs: &FxHashSet<Self::Set>,
            outputs: &mut FxHashMap<Self::Set, Self::Folding>,
        ) {
            outputs.clear();

            for &input in inputs {
                for folding in self.all_foldings() {
                    let (from, to) = self.extract_folding(folding);

                    if !from.is_subset_of(&input) {
                        continue;
                    }

                    let output = input - from + to;

                    if inputs.contains(&output) || outputs.contains_key(&output) || known.contains_key(&output) {
                        continue;
                    }

                    outputs.insert(output, folding);
                }
            }
        }
    }

    pub(super) struct ForwardSearcher<'a, R> {
        pub(super) recipes: &'a R,
    }

    pub(super) struct BackwardSearcher<'a, R> {
        pub(super) recipes: &'a R,
    }

    impl<R> DirectionSearcher for ForwardSearcher<'_, R>
    where
        R: RecipeSet,
        [(); R::Arcosphere::DIMENSION]: Sized,
    {
        type Set = Set<R::Arcosphere>;

        type Folding = FoldingRecipe<R::Arcosphere>;

        fn direction(&self) -> &'static str {
            "forward"
        }

        fn all_foldings(&self) -> impl Iterator<Item = Self::Folding> {
            self.recipes.foldings().into_iter()
        }

        fn extract_folding(&self, folding: Self::Folding) -> (Self::Set, Self::Set) {
            (folding.input(), folding.output())
        }
    }

    impl<R> DirectionSearcher for BackwardSearcher<'_, R>
    where
        R: RecipeSet,
        [(); R::Arcosphere::DIMENSION]: Sized,
    {
        type Set = Set<R::Arcosphere>;

        type Folding = Reverse<FoldingRecipe<R::Arcosphere>>;

        fn direction(&self) -> &'static str {
            "backward"
        }

        fn all_foldings(&self) -> impl Iterator<Item = Self::Folding> {
            self.recipes.foldings().into_iter().map(Reverse)
        }

        fn extract_folding(&self, folding: Self::Folding) -> (Self::Set, Self::Set) {
            let Reverse(folding) = folding;

            (folding.output(), folding.input())
        }
    }
} // mod fold_searcher

#[cfg(test)]
mod tests {
    use crate::{
        executor::DefaultExecutor,
        space_exploration::{SeArcosphere, SeSet},
    };

    use super::*;

    #[test]
    fn solve_zero() {
        let set = "EL".parse().unwrap();

        let expected = vec![Path {
            source: set,
            target: set,
            count: ONE,
            catalysts: Set::new(),
            recipes: Vec::new(),
        }];

        let paths = solve(set, set);

        assert_eq!(expected, paths);
    }

    #[test]
    fn solve_one() {
        let source = "EO".parse().unwrap();
        let target = "LG".parse().unwrap();

        let expected = vec![Path {
            source,
            target,
            count: ONE,
            catalysts: Set::new(),
            recipes: vec![Recipe::Folding(FoldingRecipe::new(source, target).unwrap())],
        }];

        let paths = solve(source, target);

        assert_eq!(expected, paths);
    }

    #[test]
    fn solve_space_folding_data_a() {
        let source = "EP".parse().unwrap();
        let target = "LX".parse().unwrap();
        let catalysts_g = "G".parse().unwrap();
        let catalysts_o = "O".parse().unwrap();

        let expected = vec![
            Path {
                source,
                target,
                count: ONE,
                catalysts: catalysts_g,
                recipes: vec![
                    Recipe::Folding("PG -> XO".parse().unwrap()),
                    Recipe::Folding("EO -> LG".parse().unwrap()),
                ],
            },
            Path {
                source,
                target,
                count: ONE,
                catalysts: catalysts_o,
                recipes: vec![
                    Recipe::Folding("EO -> LG".parse().unwrap()),
                    Recipe::Folding("PG -> XO".parse().unwrap()),
                ],
            },
        ];

        let paths = solve(source, target);

        assert_eq!(expected, paths);
    }

    fn solve(source: SeSet, target: SeSet) -> Vec<Path<SeArcosphere>> {
        SeSolver::<DefaultExecutor>::default()
            .solve(source, target)
            .expect("success")
    }
} // mod tests
