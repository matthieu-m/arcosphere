//! Default implementation of the various traits.

use core::{fmt, str};

use crate::model::{Arcosphere, ArcosphereFamily, ArcosphereRecipe, Path, RecipeParseError, Set, StagedPath};

/// Set of arcospheres for Space Exploration.
pub type SeArcosphereSet = Set<SeArcosphere>;

/// Path for Space Exploration.
pub type SePath = Path<SeArcosphereFamily>;

/// StagedPath for Space Exploration.
pub type SeStagedPath = StagedPath<SeArcosphereFamily>;

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

impl fmt::Display for SeArcosphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use core::fmt::Write;

        f.write_char(self.abbr())
    }
}

impl const Arcosphere for SeArcosphere {
    const DIMENSION: usize = 8;

    fn from_index(index: usize) -> Self {
        assert!(index < Self::DIMENSION);

        let index = index as u8;

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

/// Space Exploration default recipes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SeArcosphereRecipe {
    /// Inversion: γζθω -> ελξφ.
    GOTZ,
    /// Inversion: ελξφ -> γζθω.
    ELPX,
    /// Folding: εω -> γλ.
    EO,
    /// Folding: εθ -> φω.
    ET,
    /// Folding: λω -> θξ.
    LO,
    /// Folding: θλ -> εζ.
    LT,
    /// Folding: γφ -> ξω.
    PG,
    /// Folding: ζφ -> γε.
    PZ,
    /// Folding: γξ -> ζλ.
    XG,
    /// Folding: ζξ -> θφ.
    XZ,
}

impl ArcosphereRecipe for SeArcosphereRecipe {
    const DIMENSION: usize = 10;

    type Arcosphere = SeArcosphere;
    type Set = SeArcosphereSet;

    fn from_index(index: usize) -> Self {
        let index: u8 = index.try_into().expect("index to be in 0..10");

        match index {
            _ if index == Self::GOTZ as u8 => Self::GOTZ,
            _ if index == Self::ELPX as u8 => Self::ELPX,
            _ if index == Self::EO as u8 => Self::EO,
            _ if index == Self::ET as u8 => Self::ET,
            _ if index == Self::LO as u8 => Self::LO,
            _ if index == Self::LT as u8 => Self::LT,
            _ if index == Self::PG as u8 => Self::PG,
            _ if index == Self::PZ as u8 => Self::PZ,
            _ if index == Self::XG as u8 => Self::XG,
            _ if index == Self::XZ as u8 => Self::XZ,
            _ => panic!("expect index to be in 0..10"),
        }
    }

    fn into_index(self) -> usize {
        self as u8 as usize
    }

    fn input(&self) -> Self::Set {
        type A = SeArcosphere;

        const GOTZ: Set<A> = Set::from_spheres([A::Gamma, A::Omega, A::Theta, A::Zeta]);
        const ELPX: Set<A> = Set::from_spheres([A::Epsilon, A::Lambda, A::Phi, A::Xi]);
        const EO: Set<A> = Set::from_spheres([A::Epsilon, A::Omega]);
        const ET: Set<A> = Set::from_spheres([A::Epsilon, A::Theta]);
        const LO: Set<A> = Set::from_spheres([A::Lambda, A::Omega]);
        const LT: Set<A> = Set::from_spheres([A::Lambda, A::Theta]);
        const PG: Set<A> = Set::from_spheres([A::Phi, A::Gamma]);
        const PZ: Set<A> = Set::from_spheres([A::Phi, A::Zeta]);
        const XG: Set<A> = Set::from_spheres([A::Xi, A::Gamma]);
        const XZ: Set<A> = Set::from_spheres([A::Xi, A::Zeta]);

        match self {
            Self::GOTZ => GOTZ,
            Self::ELPX => ELPX,
            Self::EO => EO,
            Self::ET => ET,
            Self::LO => LO,
            Self::LT => LT,
            Self::PG => PG,
            Self::PZ => PZ,
            Self::XG => XG,
            Self::XZ => XZ,
        }
    }

    fn output(&self) -> Self::Set {
        type A = SeArcosphere;

        const ELPX: Set<A> = Set::from_spheres([A::Epsilon, A::Lambda, A::Phi, A::Xi]);
        const GOTZ: Set<A> = Set::from_spheres([A::Gamma, A::Omega, A::Theta, A::Zeta]);
        const LG: Set<A> = Set::from_spheres([A::Lambda, A::Gamma]);
        const PO: Set<A> = Set::from_spheres([A::Phi, A::Omega]);
        const XT: Set<A> = Set::from_spheres([A::Xi, A::Theta]);
        const EZ: Set<A> = Set::from_spheres([A::Epsilon, A::Zeta]);
        const XO: Set<A> = Set::from_spheres([A::Xi, A::Omega]);
        const EG: Set<A> = Set::from_spheres([A::Epsilon, A::Gamma]);
        const LZ: Set<A> = Set::from_spheres([A::Lambda, A::Zeta]);
        const PT: Set<A> = Set::from_spheres([A::Phi, A::Theta]);

        match self {
            Self::GOTZ => ELPX,
            Self::ELPX => GOTZ,
            Self::EO => LG,
            Self::ET => PO,
            Self::LO => XT,
            Self::LT => EZ,
            Self::PG => XO,
            Self::PZ => EG,
            Self::XG => LZ,
            Self::XZ => PT,
        }
    }
}

impl fmt::Display for SeArcosphereRecipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.display(f)
    }
}

impl str::FromStr for SeArcosphereRecipe {
    type Err = RecipeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Space exploration default family.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeArcosphereFamily;

impl ArcosphereFamily for SeArcosphereFamily {
    type Arcosphere = SeArcosphere;
    type Set = SeArcosphereSet;
    type Recipe = SeArcosphereRecipe;
}
