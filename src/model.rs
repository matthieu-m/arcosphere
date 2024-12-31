//! Collection of types & traits used by the solver.
//!
//! The `space_exploration` module provides the default arcospheres & recipes normally available in SE.

use core::{array, cmp, error, fmt, hash, iter, marker::PhantomData, ops, str};

/// An arcosphere.
pub trait Arcosphere: Copy + fmt::Debug {
    /// The total number of arcospheres.
    ///
    /// The existing arcospheres are expected to map to indexes `0..Self::DIMENSION`.
    const DIMENSION: usize;

    /// Creates an arcosphere from an index in `0..Self::DIMENSION`.
    ///
    /// If `index` lies outside `0..Self::DIMENSION`, the implementation may either panic or return any value.
    fn from_index(index: usize) -> Self;

    /// Returns the index of an arcosphere.
    ///
    /// The implementation should ensure that the index lies within `0..Self::DIMENSION`, and uniquely identifies the
    /// given arcosphere.
    fn into_index(self) -> usize;

    /// Returns the polarity of an arcosphere.
    fn polarity(&self) -> Polarity;

    /// Returns the abbreviated name of the arcosphere, eg. 'E'.
    fn abbr(&self) -> char;

    /// Returns the full name of the arcosphere, eg. 'Epsilon'.
    fn full(&self) -> &'static str;

    /// Returns the fancy name of the arcosphere, eg. 'ε'.
    fn fancy(&self) -> &'static str {
        self.full()
    }
}

/// A set of recipes: inversions & folding available.
///
/// The recipes MUST preserve a number of properties:
///
/// -   A recipe MUST preserve the number of arcospheres: N in, N out.
/// -   An inversion recipe MUST flip the polarities.
/// -   A folding recipe MUST preserve the polarities.
///
/// Those properties are asserted during the construction of the recipes, and a panic will occur should they not hold.
pub trait RecipeSet {
    /// The Arcospheres used by the recipes.
    type Arcosphere: Arcosphere;

    /// The inversion recipes.
    fn inversions(&self) -> impl IntoIterator<Item = InversionRecipe<Self::Arcosphere>>
    where
        [(); Self::Arcosphere::DIMENSION]: Sized;

    /// The folding recipes.
    fn foldings(&self) -> impl IntoIterator<Item = FoldingRecipe<Self::Arcosphere>>
    where
        [(); Self::Arcosphere::DIMENSION]: Sized;
}

/// Polarity of an arcosphere.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Polarity {
    /// Polarity of ELPX with the default arcospheres & recipes.
    Negative,
    /// Polarity of GOTZ with the default arcospheres & recipes.
    Positive,
}

/// Possible path computed by the solver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Source arcospheres.
    pub source: Set<A>,
    /// Target arcospheres.
    pub target: Set<A>,
    /// Catalysts to use for this path.
    pub catalysts: Set<A>,
    /// Recipes to use, in order.
    pub recipes: Vec<Recipe<A>>,
}

/// A recipe, either inversion or folding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Recipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// A folding recipe.
    Folding(FoldingRecipe<A>),
    /// An inversion recipe.
    Inversion(InversionRecipe<A>),
}

impl<A> Recipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Creates a new recipe.
    ///
    /// #   Panics
    ///
    /// If the recipe is neither an inversion nor a folding recipe.
    pub fn new(input: Set<A>, output: Set<A>) -> Self {
        if input.count_negatives() == output.count_negatives() {
            Recipe::Folding(FoldingRecipe::new(input, output))
        } else {
            Recipe::Inversion(InversionRecipe::new(input, output))
        }
    }

    /// Returns a copy of the input set.
    pub fn input(&self) -> Set<A> {
        match self {
            Self::Folding(this) => this.input(),
            Self::Inversion(this) => this.input(),
        }
    }

    /// Returns a copy of the output set.
    pub fn output(&self) -> Set<A> {
        match self {
            Self::Folding(this) => this.output(),
            Self::Inversion(this) => this.output(),
        }
    }
}

/// A folding recipe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FoldingRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// The input of the recipe.
    input: Set<A>,
    /// The output of the recipe.
    output: Set<A>,
}

impl<A> FoldingRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Creates a new folding recipe.
    ///
    /// #   Panics
    ///
    /// If the number of arcospheres are not preserved.
    /// If the polarities are not preserved.
    pub fn new(input: Set<A>, output: Set<A>) -> Self {
        assert_eq!(input.len(), output.len());
        assert_eq!(input.count_negatives(), output.count_negatives());
        assert_eq!(input.count_positives(), output.count_positives());

        Self { input, output }
    }

    /// Returns a copy of the input set.
    pub fn input(&self) -> Set<A> {
        self.input
    }

    /// Returns a copy of the output set.
    pub fn output(&self) -> Set<A> {
        self.output
    }
}

/// An inversion recipe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InversionRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// The input of the recipe.
    input: Set<A>,
    /// The output of the recipe.
    output: Set<A>,
}

impl<A> InversionRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Creates a new recipe.
    ///
    /// #   Panics
    ///
    /// If the number of arcospheres are not preserved.
    /// If the polarities are not flipped.
    pub fn new(input: Set<A>, output: Set<A>) -> Self {
        assert_eq!(input.len(), output.len());
        assert_eq!(input.count_negatives(), output.count_positives());
        assert_eq!(input.count_positives(), output.count_negatives());

        Self { input, output }
    }

    /// Returns a copy of the input set.
    pub fn input(&self) -> Set<A> {
        self.input
    }

    /// Returns a copy of the output set.
    pub fn output(&self) -> Set<A> {
        self.output
    }
}

//
//  Visualization
//

impl<A> fmt::Display for Recipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        format_recipe(f, self.input(), self.output())
    }
}

impl<A> fmt::Display for FoldingRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        format_recipe(f, self.input(), self.output())
    }
}

impl<A> fmt::Display for InversionRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        format_recipe(f, self.input(), self.output())
    }
}

impl<A> str::FromStr for Recipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Err = RecipeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = parse_recipe(s)?;

        Ok(Self::new(input, output))
    }
}

impl<A> str::FromStr for FoldingRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Err = RecipeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = parse_recipe(s)?;

        Ok(Self::new(input, output))
    }
}

impl<A> str::FromStr for InversionRecipe<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    type Err = RecipeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = parse_recipe(s)?;

        Ok(Self::new(input, output))
    }
}

//  Helper to format a recipe.
fn format_recipe<A>(f: &mut fmt::Formatter<'_>, input: Set<A>, output: Set<A>) -> Result<(), fmt::Error>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    write!(f, "{input} -> {output}")
}

//  Helper to parse a recipe.
fn parse_recipe<A>(s: &str) -> Result<(Set<A>, Set<A>), RecipeParseError>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    let (input, tail) = s.split_once(' ').ok_or(RecipeParseError::IllFormatted)?;
    let (arrow, output) = tail.split_once(' ').ok_or(RecipeParseError::IllFormatted)?;

    if arrow != "->" {
        return Err(RecipeParseError::IllFormatted);
    }

    let input = input.parse()?;
    let output = output.parse()?;

    Ok((input, output))
}

/// An error which occurs when parsing a recipe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecipeParseError {
    /// Format error.
    IllFormatted,
    /// Unknown arcosphere.
    Unknown(char),
}

impl fmt::Display for RecipeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl error::Error for RecipeParseError {}

impl From<SetParseError> for RecipeParseError {
    fn from(value: SetParseError) -> RecipeParseError {
        match value {
            SetParseError::Unknown(c) => RecipeParseError::Unknown(c),
        }
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
    pub fn new() -> Self {
        let _marker = PhantomData;

        Self {
            spheres: [0; A::DIMENSION],
            _marker,
        }
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

impl<A> Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// Returns the number of spheres with negative polarity in the set.
    pub fn count_negatives(&self) -> usize {
        self.negatives().len()
    }

    /// Returns the number of spheres with positive polarity in the set.
    pub fn count_positives(&self) -> usize {
        self.positives().len()
    }

    /// Returns the number of spheres with the given polarity.
    pub fn count_polarity(&self, polarity: Polarity) -> usize {
        self.polarized(polarity).len()
    }

    /// Returns the subset of spheres with negative polarity.
    pub fn negatives(&self) -> Self {
        self.polarized(Polarity::Negative)
    }

    /// Returns the subset of spheres with positive polarity.
    pub fn positives(&self) -> Self {
        self.polarized(Polarity::Positive)
    }

    /// Returns the subset of spheres with the given polarity.
    pub fn polarized(&self, polarity: Polarity) -> Self {
        let mut spheres = self.spheres;

        spheres.iter_mut().enumerate().for_each(|(index, n)| {
            if *n == 0 {
                return;
            }

            if A::from_index(index).polarity() == polarity {
                return;
            }

            *n = 0;
        });

        let _marker = PhantomData;

        Self { spheres, _marker }
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
            let index = mapping.iter().position(|m| *m == c).ok_or(SetParseError::Unknown(c))?;

            result.insert(A::from_index(index));
        }

        Ok(result)
    }
}

/// An error which occurs when parsing a set of arcospheres.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SetParseError {
    /// Unknown arcosphere.
    Unknown(char),
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
//  Set operations
//

/// Subset trait.
pub trait IsSubsetOf {
    /// Is `self` a subset of `other`?
    fn is_subset_of(&self, other: &Self) -> bool;
}

impl<A> IsSubsetOf for Set<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn is_subset_of(&self, other: &Self) -> bool {
        self.is_subset_of(other)
    }
}

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
    ///
    /// #   Panics
    ///
    /// If one of the counts overflows.
    fn sub_assign(&mut self, other: Self) {
        self.spheres
            .iter_mut()
            .zip(&other.spheres)
            .for_each(|(s, o)| *s = s.strict_sub(*o));
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
