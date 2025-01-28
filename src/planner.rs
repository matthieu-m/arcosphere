//! A planner.
//!
//! The planner can be used to better understand the execution of a staged path, and notably how to route the
//! arcospheres from input to their point of use, from point of production to point of use, and from point of production
//! to output.
//!
//! This decomposition helps planning how to arrange the various Gravimetrics facilities to actually execute the path.

use core::{error, fmt};

use crate::model::{ArcosphereFamily, ArcosphereSet, StagedPath};

/// Description of the arcospheres flowing through the path.
///
/// In a given plan, all stages have the same number of spheres (ie, input + remainder + extracted is constant).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Plan<F>
where
    F: ArcosphereFamily,
{
    /// The path for which the plan was computed.
    pub path: StagedPath<F>,
    /// The description of each stage.
    pub stages: Vec<StageDescription<F>>,
}

impl<F> fmt::Display for Plan<F>
where
    F: ArcosphereFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (desc, stage) in self.stages.iter().zip(self.path.stages()) {
            writeln!(
                f,
                "[{}] + [{}] + [{}] | {stage}",
                desc.remainder,
                stage.input(),
                desc.extracted
            )?;
        }

        Ok(())
    }
}

/// Description of the arcospheres flowing through the stage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StageDescription<F>
where
    F: ArcosphereFamily,
{
    //
    //  Should this be split up further?
    //
    //  -   input: arcospheres still remaining from the source & catalysts.
    //  -   passed: arcospheres produced by the before-previous stages, not used in this stage.
    //  -   sidelined: arcospheres produced by the previous stage, not used in this stage.
    //  -   recovered: arcospheres produced by the before-previous stages, to be immediately used in this stage.
    //  -   fresh: arcospheres produced by the previous stage, to be immediately used in this stage.
    //  -   extracted: arcospheres produced by the previous stage, and to be extracted as targets or catalysts.
    //  -   output: arcospheres produced by the before-previous stages, and already extracted as targets or catalysts.
    /// Arcospheres available at the start of the stage, minus those used by the stage.
    pub remainder: F::Set,
    /// Arcospheres extracted by previous stages, as targets or catalysts.
    pub extracted: F::Set,
}

impl<F> StageDescription<F>
where
    F: ArcosphereFamily,
{
    /// Returns whether the description is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of arcospheres in the description.
    pub fn len(&self) -> usize {
        self.remainder.len() + self.extracted.len()
    }
}

impl<F> Default for StageDescription<F>
where
    F: ArcosphereFamily,
{
    fn default() -> Self {
        let set = F::Set::default();

        Self {
            remainder: set,
            extracted: set,
        }
    }
}

/// Error which may occur during the planning.
#[derive(Clone, Copy, Debug)]
pub enum PlanningError<F>
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

impl<F> fmt::Display for PlanningError<F>
where
    F: ArcosphereFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::FailedApplication { index, current, input } => {
                write!(f, "failed to apply stage {index} on {current}: required {input}")
            }
            Self::FailedTarget { result } => write!(f, "failed to reach target, reached {result} instead"),
            Self::FailedCatalysts { remainder } => {
                write!(f, "failed to restore catalysts, got {remainder} left instead")
            }
        }
    }
}

impl<F> error::Error for PlanningError<F> where F: ArcosphereFamily {}

/// Planner.
#[derive(Clone, Debug, Default)]
pub struct Planner<F>
where
    F: ArcosphereFamily,
{
    _family: F,
}

impl<F> Planner<F>
where
    F: ArcosphereFamily,
{
    /// Creates a new planner.
    pub fn new(_family: F) -> Self {
        Self { _family }
    }

    /// Creates an execution plan for the path, if correct.
    pub fn plan(&self, staged: StagedPath<F>) -> Result<Plan<F>, PlanningError<F>> {
        let mut remainders = Self::compute_remainders(&staged)?;

        let extracteds = Self::compute_extracteds(&mut remainders, &staged);

        let stages = remainders
            .into_iter()
            .zip(extracteds)
            .map(|(remainder, extracted)| StageDescription { remainder, extracted })
            .collect();

        Ok(Plan { path: staged, stages })
    }
}

//
//  Implementation
//

impl<F> Planner<F>
where
    F: ArcosphereFamily,
{
    //  Computes the remainder of the stage's inputs, for each stage.
    fn compute_remainders(staged: &StagedPath<F>) -> Result<Vec<F::Set>, PlanningError<F>> {
        let number_stages = staged.stages().count();

        let mut remainders = Vec::with_capacity(number_stages);

        let mut step = staged.path.source * staged.path.count + staged.path.catalysts;

        for (index, stage) in staged.stages().enumerate() {
            let input = stage.input();

            if !input.is_subset_of(&step) {
                return Err(PlanningError::FailedApplication {
                    index,
                    current: step,
                    input,
                });
            }

            remainders.push(step - input);

            step = step - input + stage.output();
        }

        let target = staged.path.target * staged.path.count;

        if !target.is_subset_of(&step) {
            return Err(PlanningError::FailedTarget { result: step });
        }

        let remainder = step - target;

        if remainder != staged.path.catalysts {
            return Err(PlanningError::FailedCatalysts { remainder });
        }

        Ok(remainders)
    }

    fn compute_extracteds(remainders: &mut [F::Set], staged: &StagedPath<F>) -> Vec<F::Set> {
        fn find_earliest_extraction_stage<S>(remainders: &[S], element: S::Arcosphere) -> usize
        where
            S: ArcosphereSet,
        {
            remainders
                .iter()
                .rposition(|remainder| !remainder.contains(element))
                .map(|index| index + 1)
                .unwrap_or_default()
        }

        let output = staged.path.target * staged.path.count + staged.path.catalysts;

        let mut extracteds = vec![F::Set::default(); remainders.len()];

        for sphere in output {
            let earliest = find_earliest_extraction_stage(remainders, sphere);

            extracteds.iter_mut().skip(earliest).for_each(|e| e.insert(sphere));
            remainders.iter_mut().skip(earliest).for_each(|r| r.remove(sphere));
        }

        extracteds
    }
}
