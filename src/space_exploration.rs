//! Default implementation of the various traits.

use crate::model::{
    Arcosphere, FoldingRecipe, InversionRecipe, Path, Polarity, Recipe, RecipeParseError, RecipeSet, Set,
};

/// Path for Space Exploration.
pub type SePath = Path<SeArcosphere>;

/// Recipe for Space Exploration.
pub type SeRecipe = Recipe<SeArcosphere>;

/// Inversion recipe for Space Exploration.
pub type SeInversionRecipe = InversionRecipe<SeArcosphere>;

/// Folding recipe for Space Exploration.
pub type SeFoldingRecipe = FoldingRecipe<SeArcosphere>;

/// Set of arcospheres for Space Exploration.
pub type SeSet = Set<SeArcosphere>;

/// Space Exploration set of recipes.
#[derive(Clone, Copy, Debug)]
pub struct SeRecipeSet {
    inversions: [SeInversionRecipe; 2],
    foldings: [SeFoldingRecipe; 8],
}

impl SeRecipeSet {
    /// Creates a new recipe set.
    pub fn new() -> Self {
        let inversions = Self::create_inversions().expect("correct inversions");
        let foldings = Self::create_foldings().expect("correct foldings");

        Self { inversions, foldings }
    }
}

impl RecipeSet for SeRecipeSet {
    type Arcosphere = SeArcosphere;

    fn inversions(&self) -> impl Iterator<Item = InversionRecipe<Self::Arcosphere>>
    where
        [(); Self::Arcosphere::DIMENSION]: Sized,
    {
        self.inversions.into_iter()
    }

    fn foldings(&self) -> impl Iterator<Item = FoldingRecipe<Self::Arcosphere>>
    where
        [(); Self::Arcosphere::DIMENSION]: Sized,
    {
        self.foldings.into_iter()
    }
}

impl Default for SeRecipeSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Space Exploration default Arcospheres.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SeArcosphere {
    /// ε -> [E]psilon.
    Epsilon,
    /// γ -> [G]amma.
    Gamma,
    /// λ -> [L]ambda.
    Lambda,
    /// ω -> [O]mega.
    Omega,
    /// φ -> [P]hi.
    Phi,
    /// θ -> [T]heta.
    Theta,
    /// ξ -> [X]i.
    Xi,
    /// ζ -> [Z]eta.
    Zeta,
}

impl Arcosphere for SeArcosphere {
    const DIMENSION: usize = 8;

    fn from_index(index: usize) -> Self {
        let index: u8 = index.try_into().expect("index to be in 0..8");

        match index {
            _ if index == Self::Epsilon as u8 => Self::Epsilon,
            _ if index == Self::Gamma as u8 => Self::Gamma,
            _ if index == Self::Lambda as u8 => Self::Lambda,
            _ if index == Self::Omega as u8 => Self::Omega,
            _ if index == Self::Phi as u8 => Self::Phi,
            _ if index == Self::Theta as u8 => Self::Theta,
            _ if index == Self::Xi as u8 => Self::Xi,
            _ if index == Self::Zeta as u8 => Self::Zeta,
            _ => panic!("expect index to be in 0..8"),
        }
    }

    fn into_index(self) -> usize {
        self as u8 as usize
    }

    fn polarity(&self) -> Polarity {
        let is_negative = matches!(*self, Self::Epsilon | Self::Lambda | Self::Phi | Self::Xi);

        if is_negative {
            Polarity::Negative
        } else {
            Polarity::Positive
        }
    }

    fn abbr(&self) -> char {
        match *self {
            Self::Epsilon => 'E',
            Self::Gamma => 'G',
            Self::Lambda => 'L',
            Self::Omega => 'O',
            Self::Phi => 'P',
            Self::Theta => 'T',
            Self::Xi => 'X',
            Self::Zeta => 'Z',
        }
    }

    fn full(&self) -> &'static str {
        match *self {
            Self::Epsilon => "Epsilon",
            Self::Gamma => "Gamma",
            Self::Lambda => "Lambda",
            Self::Omega => "Omega",
            Self::Phi => "Phi",
            Self::Theta => "Theta",
            Self::Xi => "Xi",
            Self::Zeta => "Zeta",
        }
    }

    fn fancy(&self) -> &'static str {
        match *self {
            Self::Epsilon => "ε",
            Self::Gamma => "γ",
            Self::Lambda => "λ",
            Self::Omega => "ω",
            Self::Phi => "φ",
            Self::Theta => "θ",
            Self::Xi => "ξ",
            Self::Zeta => "ζ",
        }
    }
}

//
//  Implementation
//

impl SeRecipeSet {
    fn create_inversions() -> Result<[SeInversionRecipe; 2], RecipeParseError> {
        let negatives = "ELPX".parse()?;
        let positives = "GOTZ".parse()?;

        let be_positive = InversionRecipe::new(negatives, positives)?;
        let be_negative = InversionRecipe::new(positives, negatives)?;

        Ok([be_positive, be_negative])
    }

    fn create_foldings() -> Result<[SeFoldingRecipe; 8], RecipeParseError> {
        let eo = "EO -> LG".parse()?;
        let et = "ET -> PO".parse()?;
        let lo = "LO -> XT".parse()?;
        let lt = "LT -> EZ".parse()?;

        let pg = "PG -> XO".parse()?;
        let pz = "PZ -> EG".parse()?;
        let xg = "XG -> LZ".parse()?;
        let xz = "XZ -> PT".parse()?;

        Ok([eo, et, lo, lt, pg, pz, xg, xz])
    }
}
