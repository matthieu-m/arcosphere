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
//! let source = "EP".try_into()?;
//! let target = "LX".try_into()?;
//!
//! let path = solve(source, target)?;
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
pub mod space_exploration;
