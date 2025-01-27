//!  CLI wrapper around the library functionality, for human explorations.
//!
//! There are two sub-commands:
//!
//! -   `<arcosphere> solve SOURCE TARGET`.
//! -   `<arcosphere> verify PATH`.
//!     where PATH is SOURCE -> TARGET [xCOUNT] [+CATALYSTS] => [IN -> OUT] ((// | '|') [IN -> OUT])*.

//  Features
#![feature(generic_const_exprs)]
#![feature(let_chains)]
//  Lints
#![allow(incomplete_features)]

mod command;

use std::{env, error::Error};

use arcosphere::space_exploration::{SeArcosphereSet, SeStagedPath};

use command::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let command = command::parse(env::args().skip(1))?;

    match command {
        Command::Solve { source, target } => run_solve(source, target),
        Command::Verify { path } => run_verify(&path),
    }
}

//
//  Implementation
//

fn run_solve(source: SeArcosphereSet, target: SeArcosphereSet) -> Result<(), Box<dyn Error>> {
    let paths = arcosphere::solve(source, target)?;

    for path in paths {
        println!("{path}");
    }

    Ok(())
}

fn run_verify(path: &SeStagedPath) -> Result<(), Box<dyn Error>> {
    arcosphere::verify(path)?;

    Ok(())
}
