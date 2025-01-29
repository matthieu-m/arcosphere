//!  CLI wrapper around the library functionality, for human explorations.
//!
//! There are two sub-commands:
//!
//! -   `<arcosphere> solve SOURCE TARGET`.
//! -   `<arcosphere> verify PATH`.
//!     where PATH is SOURCE -> TARGET [xCOUNT] [+CATALYSTS] => [IN -> OUT] ((// | '|') [IN -> OUT])*.
//! -   `<arcosphere> plan PATH`.
//!     where PATH is SOURCE -> TARGET [xCOUNT] [+CATALYSTS] => [IN -> OUT] ((// | '|') [IN -> OUT])*.

//  Features
#![feature(generic_const_exprs)]
#![feature(let_chains)]
//  Lints
#![allow(incomplete_features)]

mod command;

use std::{env, error::Error};

use arcosphere::space_exploration::{SeArcosphereSet, SeStagedPath};

use command::{Command, SortBy};

fn main() -> Result<(), Box<dyn Error>> {
    let command = command::parse(env::args().skip(1))?;

    match command {
        Command::Help => print_help(),
        Command::Solve {
            source,
            target,
            plan,
            sort_by,
        } => run_solve(source, target, plan, sort_by),
        Command::Verify { path } => run_verify(&path),
        Command::Plan { path } => run_plan(path),
    }
}

//
//  Implementation
//

fn print_help() -> Result<(), Box<dyn Error>> {
    const HELP: &str = "
<arcosphere> [--help] [solve|verify|plan] ARGUMENTS

Generic options:

-h,--help           Print this help.


Solve subcommand:

<arcosphere> solve [OPTIONS] SOURCE TARGET

                    Attempts to find a recipe path from source to target, returning all results.

SOURCE              The set of source arcospheres.
TARGET              The set of target arcospheres.

-p,--plan           Execute plan subcommand on each result.
-r,--sort-recipes   Sort by number of recipes, from smallest to largest.
-s,--sort-stages    Sort by number of stages, from smallest to largest.


Verify subcommand:

<arcosphere> verify PATH

                    Verifies that the PATH specified is valid. Specifically, verifies that each stage can be executed
                    given the input, and verifies that at the end the expected target (and catalysts) are output.

PATH                The path, as output by the solve subcommand. On the command line, quoting is necessary to pass it
                    as a single argument, and avoid the pesky shell from interpreting | or > as special characters.


Plan subcommand:

<arcosphere> plan PATH

                    Prints the detailed plan for the given path, if valid.

PATH                The path, as output by the solve subcommand. On the command line, quoting is necessary to pass it
                    as a single argument, and avoid the pesky shell from interpreting | or > as special characters.
";

    println!("{HELP}");

    Ok(())
}

fn run_solve(
    source: SeArcosphereSet,
    target: SeArcosphereSet,
    plan: bool,
    sort_by: SortBy,
) -> Result<(), Box<dyn Error>> {
    let mut paths = arcosphere::solve(source, target)?;

    match sort_by {
        SortBy::Stages => paths.sort_by_key(|staged| staged.stages.len()),
        SortBy::Recipes => paths.sort_by_key(|staged| staged.path.recipes.len()),
    }

    if !plan {
        for path in paths {
            println!("{path}");
        }

        return Ok(());
    }

    for path in paths {
        println!("{path}");

        let plan = arcosphere::plan(path)?;

        println!("{plan}");
    }

    Ok(())
}

fn run_verify(path: &SeStagedPath) -> Result<(), Box<dyn Error>> {
    arcosphere::verify(path)?;

    Ok(())
}

fn run_plan(path: SeStagedPath) -> Result<(), Box<dyn Error>> {
    let plan = arcosphere::plan(path)?;

    print!("{plan}");

    Ok(())
}
