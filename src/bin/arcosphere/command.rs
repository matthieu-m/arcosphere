//! Command parsing.

use core::{error::Error, num::NonZeroU8};

use arcosphere::{
    model::ArcosphereRecipe,
    space_exploration::{SeArcosphereRecipe, SeArcosphereSet, SePath, SeStagedPath},
};

/// Parses the command, returning it if valid.
pub fn parse<I>(args: I) -> Result<Command, Box<dyn Error>>
where
    I: IntoIterator<Item = String>,
{
    Command::parse(args)
}

/// Command passed to the binary.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Solve {
        source: SeArcosphereSet,
        target: SeArcosphereSet,
    },
    Verify {
        path: SeStagedPath,
    },
}

impl Command {
    /// Parses the command, returning it if valid.
    pub fn parse<I>(args: I) -> Result<Self, Box<dyn Error>>
    where
        I: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();

        let subcommand = args.next().ok_or("Select a subcommand: solve or verify")?;

        match subcommand.as_str() {
            "solve" => Self::parse_solve(args),
            "verify" => Self::parse_verify(args),
            _ => Err(format!("Unknown subcommand {subcommand}, only solve and verify are accepted").into()),
        }
    }
}

//
//  Implementation
//

const ONE: NonZeroU8 = NonZeroU8::new(1).unwrap();

impl Command {
    fn parse_solve<I>(mut args: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        let (Some(source), Some(target), None) = (args.next(), args.next(), args.next()) else {
            return Err("Specify exactly two arguments to solve: SOURCE and TARGET".into());
        };

        let source: SeArcosphereSet = source
            .parse()
            .map_err(|e| format!("Failed to parse SOURCE {source}: {e}"))?;
        let target: SeArcosphereSet = target
            .parse()
            .map_err(|e| format!("Failed to parse TARGET {target}: {e}"))?;

        Ok(Self::Solve { source, target })
    }

    fn parse_verify<I>(mut args: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        let (Some(source), Some(target)) = (args.next(), args.next()) else {
            return Err("Specify at least two arguments to verify: SOURCE and TARGET".into());
        };

        let source: SeArcosphereSet = source
            .parse()
            .map_err(|e| format!("Failed to parse SOURCE {source}: {e}"))?;
        let target: SeArcosphereSet = target
            .parse()
            .map_err(|e| format!("Failed to parse TARGET {target}: {e}"))?;

        let mut args = args.peekable();

        let mut count = ONE;
        let mut catalysts = SeArcosphereSet::new();
        let mut recipes = Vec::new();

        if let Some(argument) = args.peek()
            && let Some(argument) = argument.strip_prefix('x')
        {
            count = argument
                .parse()
                .map_err(|e| format!("Failed to parse COUNT x{argument}: {e}"))?;

            args.next();
        }

        if let Some(argument) = args.peek()
            && let Some(argument) = argument.strip_prefix('+')
        {
            catalysts = argument
                .parse()
                .map_err(|e| format!("Failed to parse CATALYSTS +{argument}: {e}"))?;

            args.next();
        }

        while let Some(input) = args.next() {
            let n = recipes.len();

            let (Some(arrow), Some(output)) = (args.next(), args.next()) else {
                return Err(format!("Failed to parse {n}th recipe, not formatted as: IN -> OUT").into());
            };

            if arrow != "->" {
                return Err(format!("Failed to parse {n}th recipe, not formatted as: IN -> OUT").into());
            }

            let input: SeArcosphereSet = input
                .parse()
                .map_err(|e| format!("Failed to parse IN {input} of {n}th recipe: {e}"))?;
            let output: SeArcosphereSet = output
                .parse()
                .map_err(|e| format!("Failed to parse OUT {output} of {n}th recipe: {e}"))?;

            let recipe = SeArcosphereRecipe::find(input, output)
                .map_err(|e| format!("Invalid recipe {input} -> {output}: {e}"))?;

            recipes.push(recipe);

            if args.peek().is_some_and(|argument| argument == "//" || argument == "|") {
                args.next();
            }
        }

        let path = SePath {
            source,
            target,
            count,
            catalysts,
            recipes,
        };

        let path = SeStagedPath { path, stages: vec![] };

        Ok(Self::Verify { path })
    }
}

#[cfg(test)]
mod tests {
    use arcosphere::space_exploration::SeArcosphereRecipe;

    use super::*;

    #[test]
    fn parse_unknown() {
        let result = parse_command(&["hello", "world"]);

        assert!(result.is_err());
    }

    #[test]
    fn parse_solve() {
        let expected = Command::Solve {
            source: "EP".parse().unwrap(),
            target: "LX".parse().unwrap(),
        };

        let command = parse_command(&["solve", "EP", "LX"]).expect("success");

        assert_eq!(expected, command);
    }

    #[test]
    fn parse_verify_minimal() {
        let expected = Command::Verify {
            path: SeStagedPath {
                path: SePath {
                    source: "EP".parse().unwrap(),
                    target: "LX".parse().unwrap(),
                    count: ONE,
                    catalysts: SeArcosphereSet::new(),
                    recipes: Vec::new(),
                },
                stages: Vec::new(),
            },
        };

        let command = parse_command(&["verify", "EP", "LX"]).expect("success");

        assert_eq!(expected, command);
    }

    #[test]
    fn parse_verify_complete() {
        let expected = Command::Verify {
            path: SeStagedPath {
                path: SePath {
                    source: "EP".parse().unwrap(),
                    target: "LX".parse().unwrap(),
                    count: NonZeroU8::new(2).unwrap(),
                    catalysts: "G".parse().unwrap(),
                    recipes: vec![SeArcosphereRecipe::PG, SeArcosphereRecipe::EO],
                },
                stages: Vec::new(),
            },
        };

        let command =
            parse_command(&["verify", "EP", "LX", "x2", "+G", "PG", "->", "XO", "EO", "->", "LG"]).expect("success");

        assert_eq!(expected, command);
    }

    fn parse_command(command: &[&str]) -> Result<Command, Box<dyn Error>> {
        Command::parse(command.iter().map(|s| String::from(*s)))
    }
} // mod tests
