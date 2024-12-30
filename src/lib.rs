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
//! let paths = solve(source, target);
//!
//! assert!(!paths.is_empty());
//! # Ok(())
//! # }
//! ```

//  Features
#![feature(generic_const_exprs)]
#![feature(precise_capturing_in_traits)]
#![feature(strict_overflow_ops)]
//  Lints
#![deny(missing_docs)]
#![allow(incomplete_features)]

pub mod executor;
pub mod model;
pub mod solver;
pub mod space_exploration;

use model::Set;
use solver::{Path, Solver};
use space_exploration::{SeArcosphere, SeRecipeSet};

/// Default Space Exploration solve function.
pub fn solve(input: Set<SeArcosphere>, output: Set<SeArcosphere>) -> Vec<Path<SeArcosphere>> {
    let recipes = SeRecipeSet::new();
    let executor = executor::DefaultExecutor::default();

    Solver::new(recipes).solve(executor, input, output)
}
