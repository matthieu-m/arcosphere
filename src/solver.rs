//! Core solving logic, with modular setup.

use core::cmp::Reverse;

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    executor::Executor,
    model::{Arcosphere, FoldingRecipe, Path, Recipe, RecipeSet, Set},
    space_exploration::SeRecipeSet,
};

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
    pub fn solve(&self, source: Set<R::Arcosphere>, target: Set<R::Arcosphere>) -> Vec<Path<R::Arcosphere>> {
        //  Special case: 0 conversion.

        if source == target {
            return vec![Path {
                source,
                target,
                catalysts: Set::new(),
                recipes: Vec::new(),
            }];
        }

        //  Special case: 1 conversion.

        for folding in self.recipes.foldings() {
            if source != folding.input() || target != folding.output() {
                continue;
            }

            return vec![Path {
                source,
                target,
                catalysts: Set::new(),
                recipes: vec![Recipe::Folding(folding)],
            }];
        }

        //  From then on, it gets a tad more complicated.

        let configuration = self.configuration.into();

        for i in self.configuration.minimum_catalysts..self.configuration.maximum_catalysts {
            let searchers = Searcher::generate_searchers(&self.recipes, source, target, i as usize, configuration);

            let tasks: Vec<_> = searchers.into_iter().map(|searcher| move || searcher.solve()).collect();

            let results: Vec<_> = self.executor.execute(tasks).into_iter().flatten().collect();

            if !results.is_empty() {
                return results;
            }
        }

        //  Didn't find anything, it may be necessary to raise the number of catalysts or the number of recipes in a
        //  path.
        Vec::new()
    }
}

//
//  Implementation
//

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
    catalysts: Set<R::Arcosphere>,
    configuration: SearcherConfiguration,
}

impl<R> Searcher<R>
where
    R: RecipeSet + Clone,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn generate_searchers(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
    ) -> Vec<Searcher<R>> {
        let mut result = Vec::new();

        if number_catalysts == 0 {
            return result;
        }

        Self::generate_searchers_rec(
            recipes,
            source,
            target,
            Set::new(),
            number_catalysts,
            configuration,
            &mut result,
        );

        result
    }

    fn generate_searchers_rec(
        recipes: &R,
        source: Set<R::Arcosphere>,
        target: Set<R::Arcosphere>,
        catalysts: Set<R::Arcosphere>,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
        output: &mut Vec<Searcher<R>>,
    ) {
        debug_assert!(number_catalysts > 0);

        let generator = (0..R::Arcosphere::DIMENSION).map(|i| {
            let mut catalysts = catalysts;

            catalysts.insert(R::Arcosphere::from_index(i));

            catalysts
        });

        if number_catalysts == 1 {
            let searchers = generator.map(|catalysts| Searcher {
                recipes: recipes.clone(),
                source,
                target,
                catalysts,
                configuration,
            });

            output.extend(searchers);
            return;
        }

        for catalysts in generator {
            Self::generate_searchers_rec(
                recipes,
                source,
                target,
                catalysts,
                number_catalysts - 1,
                configuration,
                output,
            );
        }
    }
}

impl<R> Searcher<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    fn solve(&self) -> Vec<Path<R::Arcosphere>> {
        //  FIXME: handle inversions, at least a bit.
        assert_eq!(self.source.count_negatives(), self.target.count_negatives());
        assert_eq!(self.source.count_positives(), self.target.count_positives());

        let maximum_iterations = (self.configuration.maximum_recipes as usize + 1) / 2;

        let source = self.source + self.catalysts;
        let target = self.target + self.catalysts;

        let mut forward = FxHashMap::default();
        let mut backward = FxHashMap::default();

        let mut in_forward = FxHashSet::from_iter([source]);
        let mut in_backward = FxHashSet::from_iter([target]);

        let mut out_forward = FxHashMap::default();
        let mut out_backward = FxHashMap::default();

        for _ in 0..maximum_iterations {
            if in_forward.is_empty() && in_backward.is_empty() {
                //  No new solution, no progress can be made now.
                break;
            }

            let searcher = searcher::ForwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut forward, &mut in_forward, &mut out_forward, &backward) {
                return self.stitch(&forward, &backward, out_forward.keys().copied());
            }

            let searcher = searcher::BackwardSearcher { recipes: &self.recipes };

            if Self::advance(&searcher, &mut backward, &mut in_backward, &mut out_backward, &forward) {
                return self.stitch(&forward, &backward, out_backward.keys().copied());
            }
        }

        Vec::new()
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

            Self::stitch_forward(self.source + self.catalysts, forward, candidate, &mut recipes);
            Self::stitch_backward(self.target + self.catalysts, backward, candidate, &mut recipes);

            result.push(Path {
                source: self.source,
                target: self.target,
                catalysts: self.catalysts,
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
} // mod searcher

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
                catalysts: catalysts_g,
                recipes: vec![
                    Recipe::Folding("PG -> XO".parse().unwrap()),
                    Recipe::Folding("EO -> LG".parse().unwrap()),
                ],
            },
            Path {
                source,
                target,
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
        SeSolver::<DefaultExecutor>::default().solve(source, target)
    }
} // mod tests
