//! Command parsing.

use core::error::Error;

use arcosphere::space_exploration::{SeArcosphereSet, SeStagedPath};

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
    Plan {
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
            "plan" => Self::parse_plan(args),
            _ => Err(format!("Unknown subcommand {subcommand}, only solve, verify and plan are accepted").into()),
        }
    }
}

//
//  Implementation
//

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
        let Some(path) = args.next() else {
            return Err("Specify exactly one argument to verify: PATH".into());
        };

        let path = path.parse().map_err(|e| format!("Failed to parse PATH: {e}"))?;

        Ok(Self::Verify { path })
    }

    fn parse_plan<I>(mut args: I) -> Result<Self, Box<dyn Error>>
    where
        I: Iterator<Item = String>,
    {
        let Some(path) = args.next() else {
            return Err("Specify exactly one argument to plan: PATH".into());
        };

        let path = path.parse().map_err(|e| format!("Failed to parse PATH: {e}"))?;

        Ok(Self::Plan { path })
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU8;

    use arcosphere::space_exploration::{SeArcosphereRecipe, SePath, SeStagedPath};

    use super::*;

    const ONE: NonZeroU8 = NonZeroU8::new(1).unwrap();

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
                    source: "PG".parse().unwrap(),
                    target: "XO".parse().unwrap(),
                    count: ONE,
                    catalysts: SeArcosphereSet::new(),
                    recipes: vec![SeArcosphereRecipe::PG],
                },
                stages: Vec::new(),
            },
        };

        let command = parse_command(&["verify", "PG -> XO => PG -> XO"]).expect("success");

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
                stages: vec![1],
            },
        };

        let command = parse_command(&["verify", "EP -> LX x2 + G => PG -> XO | EO -> LG"]).expect("success");

        assert_eq!(expected, command);
    }

    fn parse_command(command: &[&str]) -> Result<Command, Box<dyn Error>> {
        Command::parse(command.iter().map(|s| String::from(*s)))
    }
} // mod tests
