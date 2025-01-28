//! A generic solver of arcosphere transformation paths.
//!
//! The solver is purposefully open-ended, though not infinitely so:
//!
//! -   It expects a notion of polarity.
//! -   It expects two sets of transformations: inversion & folding.
//! -   It expects inversion to flip polarities, and folding to preserve them.
//!
//! See the [`model`] module for the set of types & traits available.
//!
//! If no customization is desired, then just use the default solver:
//!
//! ```rust
//! use arcosphere::solve;
//!
//! # fn main() -> Result<(), Box<dyn core::error::Error>> {
//! let source = "EP".parse()?;
//! let target = "LX".parse()?;
//!
//! let paths = solve(source, target)?;
//!
//! assert!(!paths.is_empty());
//! # Ok(())
//! # }
//! ```

//  Features
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(iter_map_windows)]
#![feature(precise_capturing_in_traits)]
#![feature(strict_overflow_ops)]
//  Lints
#![deny(missing_docs)]
#![allow(incomplete_features)]

pub mod executor;
pub mod model;
pub mod planner;
pub mod solver;
pub mod space_exploration;
pub mod verifier;

use model::StagedPath;

use planner::{Plan, Planner, PlanningError};
use solver::{ResolutionError, Solver};
use space_exploration::{SeArcosphereFamily, SeArcosphereSet};
use verifier::{VerificationError, Verifier};

/// Default Space Exploration solve function.
pub fn solve(
    input: SeArcosphereSet,
    output: SeArcosphereSet,
) -> Result<Vec<StagedPath<SeArcosphereFamily>>, ResolutionError> {
    Solver::<_, executor::DefaultExecutor>::new(SeArcosphereFamily).solve(input, output)
}

/// Default Space Exploration verify function.
pub fn verify(path: &StagedPath<SeArcosphereFamily>) -> Result<(), VerificationError<SeArcosphereFamily>> {
    Verifier::new(SeArcosphereFamily).verify(path)
}

/// Default Space Exploration plan function.
pub fn plan(
    path: StagedPath<SeArcosphereFamily>,
) -> Result<Plan<SeArcosphereFamily>, PlanningError<SeArcosphereFamily>> {
    Planner::new(SeArcosphereFamily).plan(path)
}
