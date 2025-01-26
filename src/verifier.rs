//! A verifier.
//!
//! The verifier can be used to verify the legitimacy of a path:
//!
//! 1.  Verifying that all recipes do, in fact, exist.
//! 2.  Verifying that applying the recipes in the path do indeed lead from source to target, returning the catalysts
//!     back.

use core::{error, fmt};

use crate::model::{ArcosphereFamily, ArcosphereSet, StagedPath};

/// Error which may occur during the verification.
#[derive(Clone, Copy, Debug)]
pub enum VerificationError<F>
where
    F: ArcosphereFamily,
{
    /// A stage could not be applied due to insufficient spheres.
    FailedApplication {
        /// Index of the stage in the path.
        index: usize,
        /// State prior to attempting to apply the stage.
        current: F::Set,
        /// Expected input of the stage.
        input: F::Set,
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
            Self::FailedApplication { index, current, input } => {
                write!(f, "failed to apply step {index} on {current}: required {input}")
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
    pub fn verify(&self, staged: &StagedPath<F>) -> Result<(), VerificationError<F>> {
        let mut step = staged.path.source * staged.path.count + staged.path.catalysts;

        for (index, stage) in staged.stages().enumerate() {
            let input = stage.input();

            if !input.is_subset_of(&step) {
                return Err(VerificationError::FailedApplication {
                    index,
                    current: step,
                    input,
                });
            }

            step = step - input + stage.output();
        }

        let target = staged.path.target * staged.path.count;

        if !target.is_subset_of(&step) {
            return Err(VerificationError::FailedTarget { result: step });
        }

        let remainder = step - target;

        if remainder != staged.path.catalysts {
            return Err(VerificationError::FailedCatalysts { remainder });
        }

        Ok(())
    }
}
