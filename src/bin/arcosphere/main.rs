//!  CLI wrapper around the library functionality, for human explorations.
//!
//! There are two sub-commands:
//!
//! -   `<arcosphere> solve SOURCE TARGET`.
//! -   `<arcosphere> verify SOURCE TARGET [xCOUNT] [+CATALYSTS] [IN -> OUT]*`.

//  Features
#![feature(generic_const_exprs)]
#![feature(let_chains)]
//  Lints
#![allow(incomplete_features)]

mod command;

use std::{env, error::Error};

use arcosphere::{
    model::ArcosphereRecipe,
    space_exploration::{SeArcosphereSet, SePath},
};

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
    use std::fmt::Write;

    let paths = arcosphere::solve(source, target)?;

    for path in paths {
        let mut line = format!("{} {}", path.source, path.target);

        if path.count.get() != 1 {
            write!(&mut line, " x{}", path.count.get())?;
        }

        if !path.catalysts.is_empty() {
            write!(&mut line, " +{}", path.catalysts)?;
        }

        let mut separator = "  =>  ";

        for recipe in path.recipes {
            write!(&mut line, "{separator}{} -> {}", recipe.input(), recipe.output())?;

            separator = " | ";
        }

        println!("{line}");
    }

    Ok(())
}

fn run_verify(path: &SePath) -> Result<(), Box<dyn Error>> {
    arcosphere::verify(path)?;

    Ok(())
}
