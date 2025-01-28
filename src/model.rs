//! Collection of types & traits used by the solver.
//!
//! The `space_exploration` module provides the default arcospheres & recipes normally available in SE.

use core::{array, cmp, error, fmt, hash, iter, marker::PhantomData, num::NonZeroU8, ops, str};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An arcosphere.
#[const_trait]
pub trait Arcosphere: Copy + fmt::Debug + fmt::Display + Eq + hash::Hash + Ord + PartialEq + PartialOrd {
    /// The total number of arcospheres.
    ///
    /// The existing arcospheres are expected to map to indexes `0..Self::DIMENSION`.
    const DIMENSION: usize;

    /// Returns an array of all arcospheres.
    fn all() -> [Self; Self::DIMENSION] {
        let mut array = [Self::from_index(0); Self::DIMENSION];

        let mut i = 1;

        while i < Self::DIMENSION {
            array[i] = Self::from_index(i);

            i += 1;
        }

        array
    }

    /// Creates an arcosphere from an index in `0..Self::DIMENSION`.
    ///
    /// If `index` lies outside `0..Self::DIMENSION`, the implementation may either panic or return any value.
    fn from_index(index: usize) -> Self;

    /// Returns the index of an arcosphere.
    ///
    /// The implementation should ensure that the index lies within `0..Self::DIMENSION`, and uniquely identifies the
    /// given arcosphere.
    fn into_index(self) -> usize;

    /// Returns the abbreviated name of the arcosphere, eg. 'E'.
    fn abbr(&self) -> char;

    /// Returns the full name of the arcosphere, eg. 'Epsilon'.
    fn full(&self) -> &'static str;

    /// Returns the fancy name of the arcosphere, eg. 'ε'.
    fn fancy(&self) -> &'static str {
        self.full()
    }
}

/// A set of arcospheres.
pub trait ArcosphereSet:
    Copy
    + fmt::Debug
    + Default
    + fmt::Display
    + Eq
    + hash::Hash
    + iter::IntoIterator<Item = Self::Arcosphere>
    + Ord
    + PartialEq
    + PartialOrd
    + str::FromStr
    //  Set operations
    + ops::AddAssign
    + ops::Add<Output = Self>
    + ops::SubAssign
    + ops::Sub<Output = Self>
    + ops::MulAssign<u8>
    + ops::Mul<u8, Output = Self>
    + ops::MulAssign<NonZeroU8>
    + ops::Mul<NonZeroU8, Output = Self>
{
    /// The type of arcospheres used by the set.
    type Arcosphere: Arcosphere;

    /// Returns where the set contains any arcosphere.
    fn is_empty(&self) -> bool;

    /// Returns the number of spheres in the set.
    fn len(&self) -> usize;

    /// Returns where a sphere is contained in the set.
    fn contains(&self, sphere: Self::Arcosphere) -> bool;

    /// Returns whether `self` is a subset of `other`.
    ///
    /// A set may be neither a subset nor a superset of another.
    fn is_subset_of(&self, other: &Self) -> bool;

    /// Returns whether `self` is a superset of `candidate`.
    ///
    /// A set may be neither a subset nor a superset of another.
    fn is_superset_of(&self, other: &Self) -> bool;

    /// Inserts a sphere in the set.
    fn insert(&mut self, sphere: Self::Arcosphere);

    /// Removes a sphere from the set.
    ///
    /// #   Panics
    ///
    /// If there is no such sphere in the set.
    fn remove(&mut self, sphere: Self::Arcosphere);
}

/// A recipe, transforming a set of arcospheres into another set.
pub trait ArcosphereRecipe:
    Copy + fmt::Debug + fmt::Display + Eq + hash::Hash + Ord + PartialEq + PartialOrd + str::FromStr
{
    /// The total number of arcosphere recipes.
    ///
    /// The existing arcosphere recipes are expected to map to indexes `0..Self::DIMENSION`.
    const DIMENSION: usize;

    /// The type of arcospheres used by the recipe.
    type Arcosphere: Arcosphere;
    /// The type of set of arcospheres used by the recipe.
    type Set: ArcosphereSet<Arcosphere = Self::Arcosphere>;

    /// Creates an arcosphere recipe from an index in `0..Self::DIMENSION`.
    ///
    /// If `index` lies outside `0..Self::DIMENSION`, the implementation may either panic or return any value.
    fn from_index(index: usize) -> Self;

    /// Returns the index of an arcosphere recipe.
    ///
    /// The implementation should ensure that the index lies within `0..Self::DIMENSION`, and uniquely identifies the
    /// given arcosphere recipe.
    fn into_index(self) -> usize;

    /// The input of the recipe.
    ///
    /// The number of the arcospheres in the output MUST match the number of arcospheres in the input.
    fn input(&self) -> Self::Set;

    /// The output of the recipe.
    ///
    /// The number of the arcospheres in the output MUST match the number of arcospheres in the input.
    fn output(&self) -> Self::Set;

    /// Finds the recipe.
    fn find(input: Self::Set, output: Self::Set) -> Result<Self, RecipeIdentifyError> {
        (0..Self::DIMENSION)
            .map(|i| Self::from_index(i))
            .find(|r| r.input() == input && r.output() == output)
            .ok_or(RecipeIdentifyError::UnknownRecipe)
    }

    /// Parses the recipe, for use in implementing `str::FromStr`.
    fn parse(s: &str) -> Result<Self, RecipeParseError>
    where
        Self::Set: str::FromStr<Err = SetParseError>,
        [(); Self::DIMENSION]: Sized,
    {
        let mut tokens = s.split_whitespace().peekable();

        let recipe = parse::parse_recipe::<Self, _>(&mut tokens)?;

        if tokens.peek().is_some() {
            return Err(RecipeParseError::Incomplete);
        }

        Ok(recipe)
    }

    /// Formats the recipe, for use in implementing `fmt::Display`.
    fn display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}", self.input(), self.output())
    }
}

/// A family of arcospheres.
pub trait ArcosphereFamily: Copy + fmt::Debug + Eq + hash::Hash + PartialEq {
    /// The Arcospheres used by the recipes.
    type Arcosphere: Arcosphere;
    /// The type of set of arcospheres used by the recipe.
    type Set: ArcosphereSet<Arcosphere = Self::Arcosphere>;
    /// The type of recipes.
    type Recipe: ArcosphereRecipe<Arcosphere = Self::Arcosphere, Set = Self::Set>;
}

/// An erorr which occurs when identifying a recipe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecipeIdentifyError {
    /// Unknown recipe.
    UnknownRecipe,
}

impl fmt::Display for RecipeIdentifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for RecipeIdentifyError {}

/// An error which occurs when parsing a recipe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecipeParseError {
    /// There is extraneous non-whitespace after the recipe.
    Incomplete,
    /// The input is missing.
    MissingInput,
    /// The input is invalid.
    InvalidInput(SetParseError),
    /// The arrow between input & output is missing.
    MissingArrow,
    /// The arrow between input & output is invalid.
    InvalidArrow,
    /// The output is missing.
    MissingOutput,
    /// The output is invalid.
    InvalidOutput(SetParseError),
    /// The number of arcospheres is not preserved.
    PreservationError,
    /// Unknown recipe.
    UnknownRecipe,
}

impl From<RecipeIdentifyError> for RecipeParseError {
    fn from(value: RecipeIdentifyError) -> RecipeParseError {
        match value {
            RecipeIdentifyError::UnknownRecipe => RecipeParseError::UnknownRecipe,
        }
    }
}

impl fmt::Display for RecipeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for RecipeParseError {}

/// Possible path computed by the solver.
///
/// This path converts source * count + catalysts into target * count + catalysts.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Path<F>
where
    F: ArcosphereFamily,
{
    /// Source arcospheres.
    pub source: F::Set,
    /// Target arcospheres.
    pub target: F::Set,
    /// Minimum number of source -> target transformations to perform.
    ///
    /// When inversions kick in, it may be necessary to batch the conversions for the number of polarity flips to line
    /// up. Thus, while in trivial cases count is 1, with inversions it may be greater.
    pub count: NonZeroU8,
    /// Catalysts to use for this path.
    pub catalysts: F::Set,
    /// Recipes to use, in order.
    pub recipes: Vec<F::Recipe>,
}

impl<F> Path<F>
where
    F: ArcosphereFamily,
{
    #[allow(clippy::type_complexity)]
    fn tuplify(&self) -> (F::Set, F::Set, NonZeroU8, F::Set, &[F::Recipe]) {
        (self.source, self.target, self.count, self.catalysts, &self.recipes)
    }
}

//
//  String operations
//

impl<F> fmt::Display for Path<F>
where
    F: ArcosphereFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}", self.source, self.target)?;

        if self.count.get() > 1 {
            write!(f, " x{}", self.count.get())?;
        }

        if !self.catalysts.is_empty() {
            write!(f, " + {}", self.catalysts)?;
        }

        for (i, recipe) in self.recipes.iter().enumerate() {
            let separator = if i > 0 { " | " } else { "  =>  " };

            write!(f, "{separator}{recipe}")?;
        }

        Ok(())
    }
}

impl<F> str::FromStr for Path<F>
where
    F: ArcosphereFamily<Set: str::FromStr<Err = SetParseError>>,
    [(); F::Recipe::DIMENSION]: Sized,
{
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SEPARATOR: &str = "|";

        let mut tokens = s.split_whitespace().peekable();

        let mut this =
            parse::parse_path_head::<F, _>(&mut tokens).map_err(|error| PathParseError::InvalidHead { error })?;

        loop {
            let index = this.recipes.len();

            if tokens.peek().is_some_and(|s| *s == SEPARATOR) {
                return Err(PathParseError::UnexpectedSeparator { index });
            }

            let recipe = parse::parse_recipe::<F::Recipe, _>(&mut tokens)
                .map_err(|error| PathParseError::InvalidRecipe { index, error })?;

            this.recipes.push(recipe);

            let Some(separator) = tokens.next() else {
                //  Nothing else, we're done!
                break;
            };

            if separator == SEPARATOR {
                continue;
            }

            let error = if separator.parse::<F::Set>().is_ok() {
                PathParseError::MissingSeparator { index }
            } else {
                PathParseError::InvalidSeparator { index }
            };

            return Err(error);
        }

        Ok(this)
    }
}

/// Error which may arise when parsing a path.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PathParseError {
    /// The head of the path (SOURCE -> TARGET xCOUNT + CATALYSTS) could not be parsed.
    InvalidHead {
        /// Reason for which the head is invalid.
        error: PathHeadParseError,
    },
    /// A recipe in the path is invalid.
    InvalidRecipe {
        /// Index of the invalid recipe.
        index: usize,
        /// Reason for which the recipe is invalid.
        error: RecipeParseError,
    },
    /// A separator in the path is not "|".
    InvalidSeparator {
        /// Index of the recipe after which the invalid separator occurs.
        index: usize,
    },
    /// A separator in the path is missing.
    MissingSeparator {
        /// Index of the recipe after which the missing separator should occur.
        index: usize,
    },
    /// A separator was found in the path where none was expected
    UnexpectedSeparator {
        /// Index of the recipe before which the unexpected separator occurs.
        index: usize,
    },
}

impl fmt::Display for PathParseError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for PathParseError {}

/// Error which may arise when parsing a path.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PathHeadParseError {
    /// The source is missing.
    MissingSource,
    /// The source is invalid.
    InvalidSource(SetParseError),
    /// The arrow between source & target is missing.
    MissingArrow,
    /// The arrow between source & target is invalid.
    InvalidArrow,
    /// The target is missing.
    MissingTarget,
    /// The target is invalid.
    InvalidTarget(SetParseError),
    /// The count is invalid.
    InvalidCount,
    /// The catalysts are missing (after a +).
    MissingCatalysts,
    /// The catalysts are invalid.
    InvalidCatalysts(SetParseError),
    /// The end of the path (=>) is missing.
    MissingEnd,
}

impl fmt::Display for PathHeadParseError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for PathHeadParseError {}

//
//  Identity operations
//

impl<F> cmp::PartialEq for Path<F>
where
    F: ArcosphereFamily,
{
    fn eq(&self, other: &Self) -> bool {
        self.tuplify() == other.tuplify()
    }
}

impl<F> cmp::Eq for Path<F> where F: ArcosphereFamily {}

impl<F> hash::Hash for Path<F>
where
    F: ArcosphereFamily,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.tuplify().hash(state);
    }
}

//
//  Order operations
//

impl<F> cmp::PartialOrd for Path<F>
where
    F: ArcosphereFamily,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> cmp::Ord for Path<F>
where
    F: ArcosphereFamily,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.tuplify().cmp(&other.tuplify())
    }
}

/// A stage in the path.
#[derive(Clone, Copy, Debug)]
pub struct Stage<'a, R>(pub &'a [R]);

impl<R> Stage<'_, R>
where
    R: ArcosphereRecipe,
{
    /// Returns the input of the stage, that is the combined input of all recipes in the stage.
    pub fn input(&self) -> R::Set {
        self.0.iter().fold(R::Set::default(), |acc, r| acc + r.input())
    }

    /// Returns the output of the stage, that is the combined output of all recipes in the stage.
    pub fn output(&self) -> R::Set {
        self.0.iter().fold(R::Set::default(), |acc, r| acc + r.output())
    }
}

//
//  String operations
//

impl<R> fmt::Display for Stage<'_, R>
where
    R: ArcosphereRecipe,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (i, recipe) in self.0.iter().enumerate() {
            let separator = if i > 0 { " // " } else { "" };

            write!(f, "{separator}{recipe}")?;
        }

        Ok(())
    }
}

/// Possible staged path computed by the solver.
///
/// This path converts source * count + catalysts into target * count + catalysts.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StagedPath<F>
where
    F: ArcosphereFamily,
{
    /// The path.
    pub path: Path<F>,
    /// The index of the start of each parallel stage, with an implicit 0.
    ///
    /// The recipes in the path are, generally, not _strictly_ dependent, and instead some of them can be executed in
    /// parallel at some point.
    ///
    /// Each set of recipes which can run in parallel is called a "stage" here, and the `stages` vector points to the
    /// recipe in `path.recipes` which starts a stage.
    ///
    /// Since the first recipe always starts a stage, the implicit 0 is omitted.
    pub stages: Vec<u8>,
}

impl<F> StagedPath<F>
where
    F: ArcosphereFamily,
{
    /// Creates a StagedPath, from a Path.
    ///
    /// The staged path thus created is not only guaranteed to be valid, it is also normalized by ensuring that the
    /// recipes in each stage are sorted.
    pub fn parallelize(mut path: Path<F>) -> Self {
        struct Stage<F>
        where
            F: ArcosphereFamily,
        {
            remaining: F::Set,
            recipes: Vec<F::Recipe>,
        }

        impl<F> Default for Stage<F>
        where
            F: ArcosphereFamily,
        {
            fn default() -> Self {
                Self {
                    remaining: F::Set::default(),
                    recipes: Vec::new(),
                }
            }
        }

        fn find_earliest<F>(stages: &[Stage<F>], recipe: F::Recipe) -> usize
        where
            F: ArcosphereFamily,
        {
            assert!(!stages.is_empty());

            (0..stages.len())
                .rev()
                .find(|&i| !recipe.input().is_subset_of(&stages[i].remaining))
                .map(|i| i + 1)
                .unwrap_or(0)
        }

        let remaining = path.source * path.count + path.catalysts;

        let mut stages = vec![Stage::<F> {
            remaining,
            recipes: Vec::new(),
        }];

        for &r in &path.recipes {
            let earliest = find_earliest(&stages, r);

            let stage = &mut stages[earliest];

            stage.remaining -= r.input();
            stage.recipes.push(r);

            if stage.recipes.len() == 1 {
                let remaining = stage.remaining + r.output();
                stages.push(Stage::<F> {
                    remaining,
                    recipes: Vec::new(),
                });

                continue;
            }

            for stage in stages.iter_mut().skip(earliest + 1) {
                stage.remaining -= r.input();
                stage.remaining += r.output();
            }
        }

        if stages.last().is_some_and(|stage| stage.recipes.is_empty()) {
            stages.pop();
        }

        path.recipes.clear();

        let mut compressed = Vec::new();

        for stage in &mut stages {
            stage.recipes.sort();

            if !path.recipes.is_empty() {
                compressed.push(path.recipes.len() as u8);
            }

            path.recipes.extend_from_slice(&stage.recipes);
        }

        Self {
            path,
            stages: compressed,
        }
    }

    /// Returns an iterator over the stages.
    pub fn stages(&self) -> impl Iterator<Item = Stage<'_, F::Recipe>> + use<'_, F> {
        let start = iter::once(0);
        let end = iter::once(self.path.recipes.len());

        let stages = start.chain(self.stages.iter().map(|i| *i as usize)).chain(end);

        stages.map_windows(|&[start, end]| Stage(&self.path.recipes[start..end]))
    }

    #[allow(clippy::type_complexity)]
    fn tuplify(&self) -> (&Path<F>, &[u8]) {
        (&self.path, &self.stages)
    }
}

//
//  String operations
//

impl<F> fmt::Display for StagedPath<F>
where
    F: ArcosphereFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}", self.path.source, self.path.target)?;

        if self.path.count.get() > 1 {
            write!(f, " x{}", self.path.count.get())?;
        }

        if !self.path.catalysts.is_empty() {
            write!(f, " + {}", self.path.catalysts)?;
        }

        for (i, stage) in self.stages().enumerate() {
            let separator = if i > 0 { " |  " } else { "  =>  " };

            write!(f, "{separator}{stage}")?;
        }

        Ok(())
    }
}

impl<F> str::FromStr for StagedPath<F>
where
    F: ArcosphereFamily<Set: str::FromStr<Err = SetParseError>>,
    [(); F::Recipe::DIMENSION]: Sized,
{
    type Err = StagedPathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PARALLEL_SEPARATOR: &str = "//";
        const STAGE_SEPARATOR: &str = "|";

        let mut tokens = s.split_whitespace().peekable();

        let path =
            parse::parse_path_head::<F, _>(&mut tokens).map_err(|error| StagedPathParseError::InvalidHead { error })?;

        let mut this = StagedPath { path, stages: vec![] };

        loop {
            let index = this.path.recipes.len();

            if tokens
                .peek()
                .is_some_and(|s| *s == PARALLEL_SEPARATOR || *s == STAGE_SEPARATOR)
            {
                return Err(StagedPathParseError::UnexpectedSeparator { index });
            }

            let recipe = parse::parse_recipe::<F::Recipe, _>(&mut tokens)
                .map_err(|error| StagedPathParseError::InvalidRecipe { index, error })?;

            this.path.recipes.push(recipe);

            let Some(separator) = tokens.next() else {
                //  Nothing else, we're done!
                break;
            };

            if separator == PARALLEL_SEPARATOR {
                continue;
            }

            if separator == STAGE_SEPARATOR {
                let index = this
                    .path
                    .recipes
                    .len()
                    .try_into()
                    .map_err(|_| StagedPathParseError::TooManyRecipes)?;

                this.stages.push(index);
                continue;
            }

            let error = if separator.parse::<F::Set>().is_ok() {
                StagedPathParseError::MissingSeparator { index }
            } else {
                StagedPathParseError::InvalidSeparator { index }
            };

            return Err(error);
        }

        Ok(this)
    }
}

/// Error which may arise when parsing a path.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StagedPathParseError {
    /// The head of the path (SOURCE -> TARGET xCOUNT + CATALYSTS) could not be parsed.
    InvalidHead {
        /// Reason for which the head is invalid.
        error: PathHeadParseError,
    },
    /// A recipe in the path is invalid.
    InvalidRecipe {
        /// Index of the invalid recipe.
        index: usize,
        /// Reason for which the recipe is invalid.
        error: RecipeParseError,
    },
    /// The path contains too many recipes.
    TooManyRecipes,
    /// A separator in the path is not "|".
    InvalidSeparator {
        /// Index of the recipe after which the invalid separator occurs.
        index: usize,
    },
    /// A separator in the path is missing.
    MissingSeparator {
        /// Index of the recipe after which the missing separator should occur.
        index: usize,
    },
    /// A separator was found in the path where none was expected
    UnexpectedSeparator {
        /// Index of the recipe before which the unexpected separator occurs.
        index: usize,
    },
}

impl fmt::Display for StagedPathParseError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for StagedPathParseError {}

//
//  Identity operations
//

impl<F> cmp::PartialEq for StagedPath<F>
where
    F: ArcosphereFamily,
{
    fn eq(&self, other: &Self) -> bool {
        self.tuplify() == other.tuplify()
    }
}

impl<F> cmp::Eq for StagedPath<F> where F: ArcosphereFamily {}

impl<F> hash::Hash for StagedPath<F>
where
    F: ArcosphereFamily,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.tuplify().hash(state);
    }
}

//
//  Order operations
//

impl<F> cmp::PartialOrd for StagedPath<F>
where
    F: ArcosphereFamily,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> cmp::Ord for StagedPath<F>
where
    F: ArcosphereFamily,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.tuplify().cmp(&other.tuplify())
    }
}

/// A set of arcosphere.
///
/// A given arcosphere may appear multiple times in the set.
#[derive(Clone, Copy)]
pub struct Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    spheres: [u8; A::DIMENSION],
    _marker: PhantomData<A>,
}

impl<A> Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Creates an empty set.
    pub const fn new() -> Self {
        let _marker = PhantomData;

        Self {
            spheres: [0; A::DIMENSION],
            _marker,
        }
    }

    /// Creates a full set.
    pub const fn full() -> Self {
        let _marker = PhantomData;

        Self {
            spheres: [1; A::DIMENSION],
            _marker,
        }
    }

    /// Creates a set from a list of spheres.
    pub const fn from_spheres<const N: usize>(spheres: [A; N]) -> Self
    where
        A: ~const Arcosphere,
    {
        let mut this = Self::new();

        let mut i = 0;

        while i < spheres.len() {
            let sphere = spheres[i];
            i += 1;

            let index = sphere.into_index();

            this.spheres[index] += 1;
        }

        this
    }

    /// Returns where the set contains any arcosphere.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of spheres in the set.
    pub fn len(&self) -> usize {
        self.spheres.iter().map(|n| *n as usize).sum()
    }

    /// Returns where a sphere is contained in the set.
    pub fn contains(&self, sphere: A) -> bool {
        let index = sphere.into_index();

        self.spheres[index] > 0
    }

    /// Returns whether `self` is a subset of `other`.
    ///
    /// A set may be neither a subset nor a superset of another.
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.spheres.iter().zip(&other.spheres).all(|(s, o)| *s <= *o)
    }

    /// Returns whether `self` is a superset of `candidate`.
    ///
    /// A set may be neither a subset nor a superset of another.
    pub fn is_superset_of(&self, other: &Self) -> bool {
        other.is_subset_of(self)
    }

    /// Inserts a sphere in the set.
    ///
    /// #   Panics
    ///
    /// If there is already 255 such spheres in the set.
    pub fn insert(&mut self, sphere: A) {
        let index = sphere.into_index();

        let n = &mut self.spheres[index];

        *n = n.strict_add(1);
    }

    /// Removes a sphere from the set.
    ///
    /// #   Panics
    ///
    /// If there is no such sphere in the set.
    pub fn remove(&mut self, sphere: A) {
        let index = sphere.into_index();

        let n = &mut self.spheres[index];

        *n = n.strict_sub(1);
    }
}

impl<A> Default for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> ArcosphereSet for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Arcosphere = A;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, sphere: Self::Arcosphere) -> bool {
        self.contains(sphere)
    }

    fn is_subset_of(&self, other: &Self) -> bool {
        self.is_subset_of(other)
    }

    fn is_superset_of(&self, other: &Self) -> bool {
        self.is_superset_of(other)
    }

    fn insert(&mut self, sphere: Self::Arcosphere) {
        self.insert(sphere)
    }

    fn remove(&mut self, sphere: Self::Arcosphere) {
        self.remove(sphere)
    }
}

//
//  Visualization
//

impl<A> fmt::Debug for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self}")
    }
}

impl<A> fmt::Display for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use fmt::Write;

        for (index, n) in self.spheres.iter().enumerate() {
            if *n == 0 {
                continue;
            }

            let arcosphere = A::from_index(index).abbr();

            for _ in 0..*n {
                f.write_char(arcosphere)?;
            }
        }

        Ok(())
    }
}

impl<A> str::FromStr for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Err = SetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mapping: [char; A::DIMENSION] = array::from_fn(|index| A::from_index(index).abbr());

        let mut result = Set::new();

        for c in s.chars() {
            let index = mapping
                .iter()
                .position(|m| *m == c)
                .ok_or(SetParseError::UnknownArcosphere(c))?;

            result.insert(A::from_index(index));
        }

        Ok(result)
    }
}

/// An error which occurs when parsing a set of arcospheres.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SetParseError {
    /// Unknown arcosphere.
    UnknownArcosphere(char),
}

impl fmt::Display for SetParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for SetParseError {}

//
//  Iteration
//

impl<A> iter::Extend<A> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        for sphere in iter {
            self.insert(sphere);
        }
    }
}

impl<A> iter::FromIterator<A> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let mut result = Self::new();

        result.extend(iter);

        result
    }
}

impl<A> iter::IntoIterator for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Item = A;
    type IntoIter = IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.spheres)
    }
}

/// Iterator over a set of arcospheres.
#[derive(Clone, Debug)]
pub struct IntoIter<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    next: usize,
    spheres: [u8; A::DIMENSION],
    _marker: PhantomData<A>,
}

impl<A> IntoIter<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn new(spheres: [u8; A::DIMENSION]) -> Self {
        let next = 0;
        let _marker = PhantomData;

        Self { next, spheres, _marker }
    }
}

impl<A> Iterator for IntoIter<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.spheres.get_mut(self.next)?;

            if *n == 0 {
                self.next += 1;
                continue;
            }

            *n -= 1;

            return Some(A::from_index(self.next));
        }
    }
}

//
//  Identity operations
//

impl<A> cmp::PartialEq for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.spheres == other.spheres
    }
}

impl<A> cmp::Eq for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
}

impl<A> hash::Hash for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.spheres.hash(state);
    }
}

//
//  Order operations
//

impl<A> cmp::PartialOrd for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A> cmp::Ord for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.spheres.cmp(&other.spheres).reverse()
    }
}

//
//  Set operations
//

impl<A> ops::AddAssign for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Adds all the elements of `other` to `self`.
    ///
    /// #   Panics
    ///
    /// If one of the counts overflows.
    fn add_assign(&mut self, other: Self) {
        self.spheres
            .iter_mut()
            .zip(&other.spheres)
            .for_each(|(s, o)| *s = s.strict_add(*o));
    }
}

impl<A> ops::Add for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;

        self
    }
}

impl<A> ops::SubAssign for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Removes all the elements of `other` to `self`.
    fn sub_assign(&mut self, other: Self) {
        self.spheres
            .iter_mut()
            .zip(&other.spheres)
            .for_each(|(s, o)| *s = s.saturating_sub(*o));
    }
}

impl<A> ops::Sub for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;

        self
    }
}

impl<A> ops::MulAssign<u8> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Multiplies the number of each elements of the set by `other`.
    ///
    /// #   Panics
    ///
    /// If one of the counts overflows.
    fn mul_assign(&mut self, other: u8) {
        self.spheres.iter_mut().for_each(|s| *s = s.strict_mul(other));
    }
}

impl<A> ops::Mul<u8> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Output = Self;

    fn mul(mut self, other: u8) -> Self::Output {
        self *= other;

        self
    }
}

impl<A> ops::MulAssign<NonZeroU8> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Multiplies the number of each elements of the set by `other`.
    ///
    /// #   Panics
    ///
    /// If one of the counts overflows.
    fn mul_assign(&mut self, other: NonZeroU8) {
        self.spheres.iter_mut().for_each(|s| *s = s.strict_mul(other.get()));
    }
}

impl<A> ops::Mul<NonZeroU8> for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Output = Self;

    fn mul(mut self, other: NonZeroU8) -> Self::Output {
        self *= other;

        self
    }
}

//
//  Parsing operations
//

mod parse {
    use core::{iter::Peekable, str::FromStr};

    use super::*;

    //  Parses the beginning of the path, stopping short of the recipes.
    pub(super) fn parse_path_head<'a, F, I>(tokens: &mut Peekable<I>) -> Result<Path<F>, PathHeadParseError>
    where
        F: ArcosphereFamily<Set: FromStr<Err = SetParseError>>,
        I: Iterator<Item = &'a str>,
    {
        const SINGLE_ARROW: &str = "->";
        const DOUBLE_ARROW: &str = "=>";

        const ONE: NonZeroU8 = NonZeroU8::new(1).unwrap();

        let source = tokens
            .next()
            .ok_or(PathHeadParseError::MissingSource)
            .and_then(|s| s.parse().map_err(PathHeadParseError::InvalidSource))?;

        let arrow = tokens.next().ok_or(PathHeadParseError::MissingArrow)?;

        if arrow != SINGLE_ARROW {
            let error = if arrow.parse::<F::Set>().is_ok() {
                PathHeadParseError::MissingArrow
            } else {
                PathHeadParseError::InvalidArrow
            };

            return Err(error);
        }

        let target = tokens
            .next()
            .ok_or(PathHeadParseError::MissingTarget)
            .and_then(|s| s.parse().map_err(PathHeadParseError::InvalidTarget))?;

        let count = if let Some(count) = tokens.next_if(|s| s.starts_with('x')) {
            count
                .strip_prefix('x')
                .unwrap_or(count)
                .parse()
                .map_err(|_| PathHeadParseError::InvalidCount)?
        } else {
            ONE
        };

        let catalysts = if tokens.next_if_eq(&"+").is_some() {
            let catalysts = tokens.next().ok_or(PathHeadParseError::MissingCatalysts)?;

            if catalysts == DOUBLE_ARROW {
                return Err(PathHeadParseError::MissingCatalysts);
            }

            catalysts.parse().map_err(PathHeadParseError::InvalidCatalysts)?
        } else {
            F::Set::default()
        };

        let arrow = tokens.next().ok_or(PathHeadParseError::MissingEnd)?;

        if arrow != DOUBLE_ARROW {
            return Err(PathHeadParseError::MissingEnd);
        }

        let recipes = Vec::new();

        Ok(Path {
            source,
            target,
            count,
            catalysts,
            recipes,
        })
    }

    //  Parses one recipe.
    pub(super) fn parse_recipe<'a, R, I>(tokens: &mut Peekable<I>) -> Result<R, RecipeParseError>
    where
        R: ArcosphereRecipe<Set: FromStr<Err = SetParseError>>,
        I: Iterator<Item = &'a str>,
        [(); R::DIMENSION]: Sized,
    {
        const ARROW: &str = "->";

        let input = tokens
            .next()
            .ok_or(RecipeParseError::MissingInput)
            .and_then(|s| s.parse().map_err(RecipeParseError::InvalidInput))?;

        let arrow = tokens.next().ok_or(RecipeParseError::MissingArrow)?;

        if arrow != ARROW {
            let error = if arrow.parse::<R::Set>().is_ok() {
                RecipeParseError::MissingArrow
            } else {
                RecipeParseError::InvalidArrow
            };

            return Err(error);
        }

        let output = tokens
            .next()
            .ok_or(RecipeParseError::MissingOutput)
            .and_then(|s| s.parse().map_err(RecipeParseError::InvalidOutput))?;

        Ok(R::find(input, output)?)
    }
} // mod parse

//
//  Serialization operations
//

#[cfg(feature = "serde")]
mod serialization {
    use core::{fmt, marker::PhantomData};

    use serde::{de, ser, Deserialize, Serialize};

    use super::{Arcosphere, Set};

    impl<A> Serialize for Set<A>
    where
        A: Arcosphere,
        [(); A::DIMENSION]: Sized,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            //  Let's be reasonable, it's unlikely a set of arcosphere would have over 4096 arcospheres in there.
            let mut buffer = [0u8; 4096];
            let mut consumed = 0;

            for sphere in *self {
                let written = sphere.abbr().encode_utf8(&mut buffer[consumed..]).len();

                consumed += written;
            }

            let result = core::str::from_utf8(&buffer[..consumed]).expect("valid UTF-8");

            serializer.serialize_str(result)
        }
    }

    struct SetVisitor<A>(PhantomData<A>);

    impl<A> de::Visitor<'_> for SetVisitor<A>
    where
        A: Arcosphere,
        [(); A::DIMENSION]: Sized,
    {
        type Value = Set<A>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            f.write_str("a set of arcospheres")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }

    impl<'de, A> Deserialize<'de> for Set<A>
    where
        A: Arcosphere,
        [(); A::DIMENSION]: Sized,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(SetVisitor(PhantomData))
        }
    }
} // mod serialization
