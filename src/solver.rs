//! Core solving logic, with modular setup.

use core::{cmp::Reverse, error, fmt, num::NonZeroU8, ops::Range};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    executor::Executor,
    model::{Arcosphere, Path, Polarity, Recipe, RecipeSet, Set},
    space_exploration::SeRecipeSet,
};

/// Error which may occur during the search for a solution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolutionError {
    /// There is no solution, as the number of arcospheres is not preserved.
    PreservationError,
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
        matches!(self, Self::PreservationError | Self::NotWithInversions)
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
        let maximum_recipes = 100;

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

impl SolverConfiguration {
    fn catalysts(&self) -> Range<usize> {
        let start = self.minimum_catalysts as usize;
        let end = self.maximum_catalysts as usize + 1;

        start..end
    }
}

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

        //  From then on, it gets a tad more complicated.

        let count = Searcher::select_count(&self.recipes, source, target)?;

        self.solve_impl(source, target, count)
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

        self.solve_impl(source, target, ONE)
    }

    fn solve_impl(
        &self,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        count: NonZeroU8,
    ) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        let catalysts = self.configuration.catalysts();
        let configuration = self.configuration.into();

        let mut last_error = None;

        for i in catalysts {
            let searchers = Searcher::generate_searchers(&self.recipes, source, target, count, i, configuration);

            let tasks: Vec<_> = searchers.into_iter().map(|searcher| move || searcher.solve()).collect();

            let mut results = Vec::new();

            for result in self.executor.execute(tasks) {
                match result {
                    Ok(paths) => results.extend(paths),
                    Err(e) if e.is_definitive() => return Err(e),
                    Err(e) if e == ResolutionError::OutsideRecipes => last_error = Some(e),
                    _ => (),
                }
            }

            let Some(shortest) = results.iter().map(|p| p.recipes.len()).min() else {
                continue;
            };

            //  Should longer paths still be made available?
            results.retain(|p| p.recipes.len() == shortest);

            //  Stable output is nice, and definitely not the most costly part anyway...
            results.sort_unstable();

            return Ok(results);
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

struct Searcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    recipes: R,
    source: Set<R::Arcosphere>,
    target: Set<R::Arcosphere>,
    count: NonZeroU8,
    catalysts: Set<R::Arcosphere>,
    configuration: SearcherConfiguration,
}

impl<R> Searcher<R>
where
    R: RecipeSet + Clone,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    //  Selects the count to use.
    fn select_count(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
    ) -> Result<NonZeroU8, ResolutionError> {
        debug_assert_ne!(source.count_negatives(), target.count_negatives());

        //  Determine the direction in which to invert.
        let polarity = if source.count_negatives() > target.count_negatives() {
            Polarity::Positive
        } else {
            Polarity::Negative
        };

        //  Determine the minimum count to use.
        //
        //  The count must be so that the inversions can be applied.
        let inversions = recipes.inversions().filter(|r| r.polarity() == polarity);

        //  Determine the number of spheres flipped by a source -> target conversion, and by a recipe application.
        let (target_flipped, recipe_flipped) = match polarity {
            Polarity::Positive => {
                let target_flipped = target.count_positives() - source.count_positives();
                let recipe_flipped = inversions
                    .map(|i| i.output().count_positives() - i.input().count_positives())
                    .min();

                (target_flipped, recipe_flipped)
            }
            Polarity::Negative => {
                let target_flipped = target.count_negatives() - source.count_negatives();
                let recipe_flipped = inversions
                    .map(|i| i.output().count_negatives() - i.input().count_negatives())
                    .min();

                (target_flipped, recipe_flipped)
            }
        };

        let recipe_flipped = recipe_flipped.ok_or(ResolutionError::NotWithInversions)?;

        if target_flipped == recipe_flipped {
            return Ok(ONE);
        }

        //  If the number of flipped arcospheres doesn't quite match, use the GCD to determine how to make it work.
        let gcd = num_integer::gcd(target_flipped, recipe_flipped);

        let count = recipe_flipped / gcd;

        debug_assert_eq!(0, target_flipped * count % recipe_flipped);

        NonZeroU8::new(count.try_into().expect("sufficiently small count")).ok_or(ResolutionError::NotWithInversions)
    }

    fn generate_searchers(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        count: NonZeroU8,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
    ) -> Vec<Searcher<R>> {
        let catalysts = Self::generate_catalysts(number_catalysts);

        catalysts
            .into_iter()
            .map(|catalysts| Searcher {
                recipes: recipes.clone(),
                source,
                target,
                count,
                catalysts,
                configuration,
            })
            .collect()
    }

    //  Generates all permutations of `number` spheres.
    //
    //  The generated number of permutations is "triangularly" quadratic:
    //
    //  -   0: 0.
    //  -   1: 8, one of each.
    //  -   2: 36, at each level 8, then 7, then 6, etc...
    //  -   ...
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

        //  Do not insert spheres with a lower index than the highest index sphere used: it only creates duplicates.
        let minimum = catalysts
            .into_iter()
            .last()
            .map(|sphere| sphere.into_index())
            .unwrap_or_default();

        let generator = (minimum..R::Arcosphere::DIMENSION).map(|i| {
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
}

impl<R> Searcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn solve(&self) -> Result<Vec<Path<R::Arcosphere>>, ResolutionError> {
        let maximum_iterations = (self.configuration.maximum_recipes as usize + 1) / 2;

        let source = self.source * self.count + self.catalysts;
        let target = self.target * self.count + self.catalysts;

        let mut forward = FxHashMap::default();
        let mut backward = FxHashMap::default();

        let mut in_forward = FxHashSet::from_iter([source]);
        let mut in_backward = FxHashSet::from_iter([target]);

        let mut out_forward = FxHashMap::default();
        let mut out_backward = FxHashMap::default();

        for _ in 0..maximum_iterations {
            if in_forward.is_empty() && in_backward.is_empty() {
                return Err(ResolutionError::OutsideCatalysts);
            }

            let searcher = searcher::ForwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut forward, &mut in_forward, &mut out_forward, &backward) {
                return Ok(self.stitch(&forward, &backward, out_forward.keys().copied()));
            }

            let searcher = searcher::BackwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut backward, &mut in_backward, &mut out_backward, &forward) {
                return Ok(self.stitch(&forward, &backward, out_backward.keys().copied()));
            }
        }

        if in_forward.is_empty() && in_backward.is_empty() {
            return Err(ResolutionError::OutsideCatalysts);
        }

        Err(ResolutionError::OutsideRecipes)
    }

    //  Returns true if a connection has been found.
    fn advance<S, OF>(
        searcher: &S,
        known: &mut FxHashMap<Set<R::Arcosphere>, S::Recipe>,
        inputs: &mut FxHashSet<Set<R::Arcosphere>>,
        outputs: &mut FxHashMap<Set<R::Arcosphere>, S::Recipe>,
        opposite_known: &FxHashMap<Set<R::Arcosphere>, OF>,
    ) -> bool
    where
        S: searcher::DirectionSearcher<Set = Set<R::Arcosphere>>,
    {
        searcher.fold(known, inputs, outputs);

        inputs.clear();
        inputs.extend(outputs.keys().copied());

        known.extend(outputs.iter().map(|(&key, &value)| (key, value)));

        outputs.keys().any(|key| opposite_known.contains_key(key))
    }

    fn stitch<C>(
        &self,
        forward: &FxHashMap<Set<R::Arcosphere>, Recipe<R::Arcosphere>>,
        backward: &FxHashMap<Set<R::Arcosphere>, Reverse<Recipe<R::Arcosphere>>>,
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

            Self::stitch_forward(
                self.source * self.count + self.catalysts,
                forward,
                candidate,
                &mut recipes,
            );
            Self::stitch_backward(
                self.target * self.count + self.catalysts,
                backward,
                candidate,
                &mut recipes,
            );

            result.push(Path {
                source: self.source,
                target: self.target,
                count: self.count,
                catalysts: self.catalysts,
                recipes,
            })
        }

        debug_assert_ne!(0, result.len());

        result
    }

    fn stitch_forward(
        _source: Set<R::Arcosphere>,
        forward: &FxHashMap<Set<R::Arcosphere>, Recipe<R::Arcosphere>>,
        candidate: Set<R::Arcosphere>,
        recipes: &mut Vec<Recipe<R::Arcosphere>>,
    ) {
        let mut step = candidate;

        while let Some(recipe) = forward.get(&step) {
            step = step - recipe.output() + recipe.input();

            recipes.push(*recipe);
        }

        debug_assert_eq!(_source, step);

        recipes.reverse();
    }

    fn stitch_backward(
        _target: Set<R::Arcosphere>,
        backward: &FxHashMap<Set<R::Arcosphere>, Reverse<Recipe<R::Arcosphere>>>,
        candidate: Set<R::Arcosphere>,
        recipes: &mut Vec<Recipe<R::Arcosphere>>,
    ) {
        let mut step = candidate;

        while let Some(Reverse(recipe)) = backward.get(&step) {
            step = step - recipe.input() + recipe.output();

            recipes.push(*recipe);
        }

        debug_assert_eq!(_target, step);
    }
}

mod searcher {
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

        //  The recipe to use.
        type Recipe: Copy + Debug;

        #[allow(dead_code)]
        fn direction(&self) -> &'static str;

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe>;

        fn extract_recipe(&self, recipe: Self::Recipe) -> (Self::Set, Self::Set);

        //  Never overriden.
        fn fold(
            &self,
            known: &FxHashMap<Self::Set, Self::Recipe>,
            inputs: &FxHashSet<Self::Set>,
            outputs: &mut FxHashMap<Self::Set, Self::Recipe>,
        ) {
            outputs.clear();

            for &input in inputs {
                for recipe in self.all_recipes() {
                    let (from, to) = self.extract_recipe(recipe);

                    if !from.is_subset_of(&input) {
                        continue;
                    }

                    let output = input - from + to;

                    if inputs.contains(&output) || outputs.contains_key(&output) || known.contains_key(&output) {
                        continue;
                    }

                    outputs.insert(output, recipe);
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

        type Recipe = Recipe<R::Arcosphere>;

        fn direction(&self) -> &'static str {
            "forward"
        }

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe> {
            let inversions = self.recipes.inversions().map(Recipe::Inversion);
            let foldings = self.recipes.foldings().map(Recipe::Folding);

            inversions.chain(foldings)
        }

        fn extract_recipe(&self, recipe: Self::Recipe) -> (Self::Set, Self::Set) {
            (recipe.input(), recipe.output())
        }
    }

    impl<R> DirectionSearcher for BackwardSearcher<'_, R>
    where
        R: RecipeSet,
        [(); R::Arcosphere::DIMENSION]: Sized,
    {
        type Set = Set<R::Arcosphere>;

        type Recipe = Reverse<Recipe<R::Arcosphere>>;

        fn direction(&self) -> &'static str {
            "backward"
        }

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe> {
            let inversions = self.recipes.inversions().map(Recipe::Inversion);
            let foldings = self.recipes.foldings().map(Recipe::Folding);

            inversions.chain(foldings).map(Reverse)
        }

        fn extract_recipe(&self, recipe: Self::Recipe) -> (Self::Set, Self::Set) {
            let Reverse(recipe) = recipe;

            (recipe.output(), recipe.input())
        }
    }
} // mod searcher

#[cfg(test)]
mod tests {
    use crate::{
        executor::DefaultExecutor,
        model::FoldingRecipe,
        space_exploration::{SePath, SeSet},
    };

    use super::*;

    #[test]
    fn size() {
        assert_eq!(186, core::mem::size_of::<Searcher<SeRecipeSet>>());
    }

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

    #[test]
    fn solve_space_dilation_data_a() {
        const TWO: NonZeroU8 = NonZeroU8::new(2).unwrap();

        let source = "LL".parse().unwrap();
        let target = "OZ".parse().unwrap();

        let catalysts_pg = "PG".parse().unwrap();
        let catalysts_xo = "XO".parse().unwrap();
        let catalysts_xt = "XT".parse().unwrap();

        let inversion = "ELPX -> GOTZ".parse().unwrap();

        let et = "ET -> PO".parse().unwrap();
        let lo = "LO -> XT".parse().unwrap();
        let lt = "LT -> EZ".parse().unwrap();
        let pg = "PG -> XO".parse().unwrap();
        let xz = "XZ -> PT".parse().unwrap();

        let expected = vec![
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_pg,
                recipes: vec![pg, lo, lt, xz, inversion, lt, et],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xo,
                recipes: vec![lo, lt, xz, inversion, lt, et, pg],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xo,
                recipes: vec![lo, lt, xz, lt, inversion, et, pg],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xt,
                recipes: vec![lt, xz, et, lo, lt, inversion, pg],
            },
        ];

        let paths = solve(source, target);

        assert_eq!(expected, paths);
    }

    fn solve(source: SeSet, target: SeSet) -> Vec<SePath> {
        SeSolver::<DefaultExecutor>::default()
            .solve(source, target)
            .expect("success")
    }

    #[test]
    fn catalysts_zero_five() {
        const EXPECTED: [usize; 6] = [0, 8, 36, 120, 330, 792];

        for (n, expected) in EXPECTED.into_iter().enumerate() {
            let catalysts = generate_catalysts(n);

            assert_eq!(expected, catalysts.len(), "{n}: {catalysts:?}");

            let deduplicated: FxHashSet<_> = catalysts.iter().copied().collect();

            assert_eq!(
                catalysts.len(),
                deduplicated.len(),
                "{n}: {catalysts:?} <> {deduplicated:?}"
            );
        }
    }

    fn generate_catalysts(n: usize) -> Vec<SeSet> {
        Searcher::<SeRecipeSet>::generate_catalysts(n)
    }
} // mod tests
