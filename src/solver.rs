//! Core solving logic, with modular setup.

use core::{cmp::Reverse, error, fmt, num::NonZeroU8, ops::Range};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    executor::Executor,
    model::{Arcosphere, ArcosphereFamily, ArcosphereRecipe, ArcosphereSet, Path},
    space_exploration::SeArcosphereFamily,
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
    /// The maximum number of repetitions allowed.
    pub maximum_repetitions: u8,
    /// The maximum number of recipes in the path from source to target.
    pub maximum_recipes: u8,
}

impl Default for SolverConfiguration {
    fn default() -> Self {
        //  Sufficient for all SE recipes.
        let maximum_catalysts = 2;
        let minimum_catalysts = 0;
        let maximum_repetitions = 4;
        let maximum_recipes = 20;

        Self {
            maximum_catalysts,
            minimum_catalysts,
            maximum_repetitions,
            maximum_recipes,
        }
    }
}

/// Solver.
#[derive(Clone, Debug, Default)]
pub struct Solver<F, E>
where
    F: ArcosphereFamily,
{
    family: F,
    executor: E,
    configuration: SolverConfiguration,
}

//
//  Configuration
//

impl<F, E> Solver<F, E>
where
    F: ArcosphereFamily,
{
    /// Creates a new solver based on a family of arcospheres.
    pub fn new(family: F) -> Self
    where
        E: Default,
    {
        let executor = E::default();
        let configuration = SolverConfiguration::default();

        Self {
            family,
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
    pub fn with_executor<OE>(self, executor: OE) -> Solver<F, OE> {
        let Solver {
            family, configuration, ..
        } = self;

        Solver {
            family,
            executor,
            configuration,
        }
    }
}

//
//  Space exploration short-hand.
//

/// Solver for default Space Exploration.
pub type SeSolver<E> = Solver<SeArcosphereFamily, E>;

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

impl<F, E> Solver<F, E>
where
    F: ArcosphereFamily<Arcosphere: Send, Set: Send, Recipe: Send> + Send,
    E: Executor,
{
    /// Looks for all possible recipe paths from `source` to `target` with a minimum number of catalysts.
    ///
    /// If the solver return a set of solutions, then it is guaranted no solution exists with a smaller number of
    /// catalysts up to the given number of recipes.
    ///
    /// If the solver does not return any solution, then raising either the number of catalysts or the number of recipes
    /// may allow it to find further solutions.
    pub fn solve(&self, source: F::Set, target: F::Set) -> Result<Vec<Path<F>>, ResolutionError> {
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
                catalysts: F::Set::default(),
                recipes: Vec::new(),
            }]);
        }

        //  Special case: 1 conversion.

        for recipe in (0..F::Recipe::DIMENSION).map(F::Recipe::from_index) {
            if source != recipe.input() || target != recipe.output() {
                continue;
            }

            return Ok(vec![Path {
                source,
                target,
                count: ONE,
                catalysts: F::Set::default(),
                recipes: vec![recipe],
            }]);
        }

        //  Is an inversion required, or not?

        self.solve_impl(source, target)
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

    fn repetitions(&self) -> Range<u8> {
        let end = self.maximum_repetitions + 1;

        1..end
    }
}

impl<F, E> Solver<F, E>
where
    F: ArcosphereFamily<Arcosphere: Send, Set: Send, Recipe: Send> + Send,
    E: Executor,
{
    fn solve_impl(&self, source: F::Set, target: F::Set) -> Result<Vec<Path<F>>, ResolutionError> {
        let catalysts = self.configuration.catalysts();
        let configuration = self.configuration.into();

        let mut last_error = None;

        for i in catalysts {
            let repetitions = self.configuration.repetitions();

            for count in repetitions {
                let Some(count) = NonZeroU8::new(count) else {
                    continue;
                };

                let searchers = Searcher::generate_searchers(self.family, source, target, count, i, configuration);

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

struct Searcher<F>
where
    F: ArcosphereFamily,
{
    family: F,
    source: F::Set,
    target: F::Set,
    count: NonZeroU8,
    catalysts: F::Set,
    configuration: SearcherConfiguration,
}

impl<F> Searcher<F>
where
    F: ArcosphereFamily,
{
    fn generate_searchers(
        family: F,
        source: F::Set,
        target: F::Set,
        count: NonZeroU8,
        number_catalysts: usize,
        configuration: SearcherConfiguration,
    ) -> Vec<Searcher<F>> {
        let catalysts = Self::generate_catalysts(number_catalysts);

        catalysts
            .into_iter()
            .map(|catalysts| Searcher {
                family,
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
    fn generate_catalysts(number: usize) -> Vec<F::Set> {
        let mut result = Vec::new();

        if number == 0 {
            return result;
        }

        Self::generate_catalysts_rec(F::Set::default(), number, &mut result);

        result
    }

    fn generate_catalysts_rec(catalysts: F::Set, number: usize, output: &mut Vec<F::Set>) {
        debug_assert!(number > 0);

        //  Do not insert spheres with a lower index than the highest index sphere used: it only creates duplicates.
        let minimum = catalysts
            .into_iter()
            .last()
            .map(|sphere| sphere.into_index())
            .unwrap_or_default();

        let generator = (minimum..F::Arcosphere::DIMENSION).map(|i| {
            let mut catalysts = catalysts;

            catalysts.insert(F::Arcosphere::from_index(i));

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

impl<F> Searcher<F>
where
    F: ArcosphereFamily,
{
    fn solve(&self) -> Result<Vec<Path<F>>, ResolutionError> {
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

            let searcher = searcher::ForwardSearcher::new(self.family);

            let matched = Self::advance(
                &searcher,
                source,
                &mut forward,
                &mut in_forward,
                &mut out_forward,
                &backward,
            );

            if matched {
                return Ok(self.stitch(&forward, &backward, out_forward.keys().copied()));
            }

            let searcher = searcher::BackwardSearcher::new(self.family);

            let matched = Self::advance(
                &searcher,
                target,
                &mut backward,
                &mut in_backward,
                &mut out_backward,
                &forward,
            );

            if matched {
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
        start: F::Set,
        known: &mut FxHashMap<F::Set, S::Recipe>,
        inputs: &mut FxHashSet<F::Set>,
        outputs: &mut FxHashMap<F::Set, S::Recipe>,
        opposite_known: &FxHashMap<F::Set, OF>,
    ) -> bool
    where
        S: searcher::DirectionSearcher<Set = F::Set>,
    {
        searcher.fold(start, known, inputs, outputs);

        inputs.clear();
        inputs.extend(outputs.keys().copied());

        known.extend(outputs.iter().map(|(&key, &value)| (key, value)));

        outputs.keys().any(|key| opposite_known.contains_key(key))
    }

    fn stitch<C>(
        &self,
        forward: &FxHashMap<F::Set, F::Recipe>,
        backward: &FxHashMap<F::Set, Reverse<F::Recipe>>,
        candidates: C,
    ) -> Vec<Path<F>>
    where
        C: IntoIterator<Item = F::Set>,
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
        _source: F::Set,
        forward: &FxHashMap<F::Set, F::Recipe>,
        candidate: F::Set,
        recipes: &mut Vec<F::Recipe>,
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
        _target: F::Set,
        backward: &FxHashMap<F::Set, Reverse<F::Recipe>>,
        candidate: F::Set,
        recipes: &mut Vec<F::Recipe>,
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
    use core::marker::PhantomData;

    use crate::model::{ArcosphereRecipe, ArcosphereSet};

    use super::*;

    pub(super) trait DirectionSearcher {
        //  The set of arcospheres to use.
        type Set: ArcosphereSet;

        //  The recipe to use.
        type Recipe: Copy + fmt::Debug;

        #[allow(dead_code)]
        fn direction(&self) -> &'static str;

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe>;

        fn extract_recipe(&self, recipe: Self::Recipe) -> (Self::Set, Self::Set);

        //  Never overriden.
        fn fold(
            &self,
            start: Self::Set,
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

                    if output == start
                        || inputs.contains(&output)
                        || outputs.contains_key(&output)
                        || known.contains_key(&output)
                    {
                        continue;
                    }

                    outputs.insert(output, recipe);
                }
            }
        }
    }

    pub(super) struct ForwardSearcher<F> {
        _marker: PhantomData<fn(F) -> F>,
    }

    pub(super) struct BackwardSearcher<F> {
        _marker: PhantomData<fn(F) -> F>,
    }

    impl<F> ForwardSearcher<F> {
        pub(super) fn new(_family: F) -> Self {
            Self { _marker: PhantomData }
        }
    }

    impl<F> BackwardSearcher<F> {
        pub(super) fn new(_family: F) -> Self {
            Self { _marker: PhantomData }
        }
    }

    impl<F> DirectionSearcher for ForwardSearcher<F>
    where
        F: ArcosphereFamily,
    {
        type Set = F::Set;

        type Recipe = F::Recipe;

        fn direction(&self) -> &'static str {
            "forward"
        }

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe> {
            (0..F::Recipe::DIMENSION).map(F::Recipe::from_index)
        }

        fn extract_recipe(&self, recipe: Self::Recipe) -> (Self::Set, Self::Set) {
            (recipe.input(), recipe.output())
        }
    }

    impl<F> DirectionSearcher for BackwardSearcher<F>
    where
        F: ArcosphereFamily,
    {
        type Set = F::Set;

        type Recipe = Reverse<F::Recipe>;

        fn direction(&self) -> &'static str {
            "backward"
        }

        fn all_recipes(&self) -> impl Iterator<Item = Self::Recipe> {
            (0..F::Recipe::DIMENSION).map(|i| Reverse(F::Recipe::from_index(i)))
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
        space_exploration::{SeArcosphereFamily, SeArcosphereRecipe, SeArcosphereSet, SePath},
    };

    use super::*;

    #[test]
    fn size() {
        assert_eq!(26, core::mem::size_of::<Searcher<SeArcosphereFamily>>());
    }

    #[test]
    fn solve_zero() {
        let set = "EL".parse().unwrap();

        let expected = vec![Path {
            source: set,
            target: set,
            count: ONE,
            catalysts: SeArcosphereSet::new(),
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
            catalysts: SeArcosphereSet::new(),
            recipes: vec![SeArcosphereRecipe::EO],
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
                recipes: vec![SeArcosphereRecipe::PG, SeArcosphereRecipe::EO],
            },
            Path {
                source,
                target,
                count: ONE,
                catalysts: catalysts_o,
                recipes: vec![SeArcosphereRecipe::EO, SeArcosphereRecipe::PG],
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

        let inversion = SeArcosphereRecipe::ELPX;

        let et = SeArcosphereRecipe::ET;
        let lo = SeArcosphereRecipe::LO;
        let lt = SeArcosphereRecipe::LT;
        let pg = SeArcosphereRecipe::PG;
        let xz = SeArcosphereRecipe::XZ;

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

    #[test]
    fn solve_space_injection_data_a() {
        const TWO: NonZeroU8 = NonZeroU8::new(2).unwrap();

        let source = "ZZ".parse().unwrap();
        let target = "GT".parse().unwrap();

        let catalysts = "X".parse().unwrap();

        let xz = SeArcosphereRecipe::XZ;
        let pz = SeArcosphereRecipe::PZ;
        let et = SeArcosphereRecipe::ET;
        let pg = SeArcosphereRecipe::PG;
        let eo = SeArcosphereRecipe::EO;
        let lo = SeArcosphereRecipe::LO;

        let expected = vec![Path {
            source,
            target,
            count: TWO,
            catalysts,
            recipes: vec![xz, pz, et, pg, xz, pz, eo, lo],
        }];

        let paths = solve(source, target);

        assert_eq!(expected, paths);
    }

    #[test]
    fn solve_space_warping_data_b() {
        const TWO: NonZeroU8 = NonZeroU8::new(2).unwrap();

        let source = "GO".parse().unwrap();
        let target = "EP".parse().unwrap();

        let catalysts_lt = "LT".parse().unwrap();
        let catalysts_lx = "LX".parse().unwrap();
        let catalysts_lz = "LZ".parse().unwrap();
        let catalysts_tz = "TZ".parse().unwrap();
        let catalysts_xt = "XT".parse().unwrap();
        let catalysts_xz = "XZ".parse().unwrap();

        let inversion = SeArcosphereRecipe::GOTZ;

        let lo = SeArcosphereRecipe::LO;
        let lt = SeArcosphereRecipe::LT;
        let xg = SeArcosphereRecipe::XG;
        let xz = SeArcosphereRecipe::XZ;

        let expected = vec![
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_lt,
                recipes: vec![lo, xg, inversion, lt, xz],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_lt,
                recipes: vec![lo, xg, lt, inversion, xz],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_lx,
                recipes: vec![lo, xg, lt, xz, inversion],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_lx,
                recipes: vec![lo, xg, xz, lt, inversion],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_lz,
                recipes: vec![lo, inversion, xg, xz, lt],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xt,
                recipes: vec![xg, inversion, lo, lt, xz],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_tz,
                recipes: vec![inversion, lo, xg, xz, lt],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xz,
                recipes: vec![xg, lo, inversion, xz, lt],
            },
            Path {
                source,
                target,
                count: TWO,
                catalysts: catalysts_xz,
                recipes: vec![xg, lo, xz, inversion, lt],
            },
        ];

        let paths = solve(source, target);

        assert_eq!(expected, paths);
    }

    fn solve(source: SeArcosphereSet, target: SeArcosphereSet) -> Vec<SePath> {
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

    fn generate_catalysts(n: usize) -> Vec<SeArcosphereSet> {
        Searcher::<SeArcosphereFamily>::generate_catalysts(n)
    }
} // mod tests
