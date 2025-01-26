//! A verifier.
//!
//! The verifier can be used to verify the legitimacy of a path:
//!
//! 1.  Verifying that all recipes do, in fact, exist.
//! 2.  Verifying that applying the recipes in the path do indeed lead from source to target, returning the catalysts
//!     back.

use core::{error, fmt};

use crate::model::{ArcosphereFamily, ArcosphereRecipe, ArcosphereSet, Path};

/// Error which may occur during the verification.
#[derive(Clone, Copy, Debug)]
pub enum VerificationError<F>
where
    F: ArcosphereFamily,
{
    /// A recipe could not be applied due to insufficient spheres.
    FailedApplication {
        /// Index of the recipe in the path.
        index: usize,
        /// Recipe itself.
        recipe: F::Recipe,
        /// State prior to attempting to apply the recipe.
        current: F::Set,
    },
    /// Applying all recipes in order did not result in the expected target.
    FailedTarget {
        /// Result of applying all recipes in order from the source (+ catalysts).
        result: F::Set,
    },
    /// Applying all recipes in order did not recover the catalysts.
    FailedCatalysts {
        /// Remainder after applying all recipes in order from the source (+ catalysts), and removing the target.
        remainder: F::Set,
    },
}

impl<F> fmt::Display for VerificationError<F>
where
    F: ArcosphereFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
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

impl<F> error::Error for VerificationError<F> where F: ArcosphereFamily {}

/// Verifier.
#[derive(Clone, Debug, Default)]
pub struct Verifier<F>
where
    F: ArcosphereFamily,
{
    _family: F,
}

impl<F> Verifier<F>
where
    F: ArcosphereFamily,
{
    /// Creates a new verifier.
    pub fn new(_family: F) -> Self {
        Self { _family }
    }

    /// Verifies that the path is correct.
    pub fn verify(&self, path: &Path<F>) -> Result<(), VerificationError<F>> {
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
