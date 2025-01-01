//! A verifier.
//!
//! The verifier can be used to verify the legitimacy of a path:
//!
//! 1.  Verifying that all recipes do, in fact, exist.
//! 2.  Verifying that applying the recipes in the path do indeed lead from source to target, returning the catalysts
//!     back.

use core::{error, fmt};

use crate::model::{Arcosphere, Path, Recipe, RecipeSet, Set};

/// Error which may occur during the verification.
#[derive(Clone, Copy, Debug)]
pub enum VerificationError<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    /// A recipe is unknown.
    UnknownRecipe {
        /// Index of the recipe in the path.
        index: usize,
        /// Recipe itself.
        recipe: Recipe<A>,
    },
    /// A recipe could not be applied due to insufficient spheres.
    FailedApplication {
        /// Index of the recipe in the path.
        index: usize,
        /// Recipe itself.
        recipe: Recipe<A>,
        /// State prior to attempting to apply the recipe.
        current: Set<A>,
    },
    /// Applying all recipes in order did not result in the expected target.
    FailedTarget {
        /// Result of applying all recipes in order from the source (+ catalysts).
        result: Set<A>,
    },
    /// Applying all recipes in order did not recover the catalysts.
    FailedCatalysts {
        /// Remainder after applying all recipes in order from the source (+ catalysts), and removing the target.
        remainder: Set<A>,
    },
}

impl<A> fmt::Display for VerificationError<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::UnknownRecipe { index, recipe } => write!(f, "unknown recipe {recipe} at step {index}"),
            Self::FailedApplication { index, recipe, current } => {
                write!(f, "failed to apply {recipe} at step {index} on {current}")
            }
            Self::FailedTarget { result } => write!(f, "failed to reach target, reached {result} instead"),
            Self::FailedCatalysts { remainder } => {
                write!(f, "failed to restore catalysts, got {remainder} left instead")
            }
        }
    }
}

impl<A> error::Error for VerificationError<A>
where
    A: Arcosphere,
    [(); A::DIMENSION]: Sized,
{
}

/// Verifier.
#[derive(Clone, Debug, Default)]
pub struct Verifier<R>
where
    R: RecipeSet,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    recipes: R,
}

impl<R> Verifier<R>
where
    R: RecipeSet<Arcosphere: PartialEq>,
    [(); R::Arcosphere::DIMENSION]: Sized,
{
    /// Creates a new verifier.
    pub fn new(recipes: R) -> Self {
        Self { recipes }
    }

    /// Verifies that the path is correct.
    pub fn verify(&self, path: &Path<R::Arcosphere>) -> Result<(), VerificationError<R::Arcosphere>> {
        for (index, &recipe) in path.recipes.iter().enumerate() {
            let is_known = match recipe {
                Recipe::Inversion(inversion) => self.recipes.inversions().any(|i| inversion == i),
                Recipe::Folding(folding) => self.recipes.foldings().any(|f| folding == f),
            };

            if !is_known {
                return Err(VerificationError::UnknownRecipe { index, recipe });
            }
        }

        let mut step = path.source * path.count + path.catalysts;

        for (index, &recipe) in path.recipes.iter().enumerate() {
            if !recipe.input().is_subset_of(&step) {
                return Err(VerificationError::FailedApplication {
                    index,
                    recipe,
                    current: step,
                });
            }

            step = step - recipe.input() + recipe.output();
        }

        let target = path.target * path.count;

        if !target.is_subset_of(&step) {
            return Err(VerificationError::FailedTarget { result: step });
        }

        let remainder = step - target;

        if remainder != path.catalysts {
            return Err(VerificationError::FailedCatalysts { remainder });
        }

        Ok(())
    }
}
