//! Executors for the solver.
//!
//! By default the solver explores the solution space sequentially, for a given number of catalysts, however it is more
//! efficient to explore it in parallel using the [`RayonExecutor`].

/// The default executor.
#[cfg(feature = "rayon")]
pub type DefaultExecutor = RayonExecutor;

/// The default executor.
#[cfg(not(feature = "rayon"))]
pub type DefaultExecutor = SequentialExecutor;

/// Abstract executor.
pub trait Executor {
    /// Executes the closures provided, returning their results.
    fn execute<I, F, R>(&self, tasks: I) -> impl IntoIterator<Item = R> + use<Self, I, F, R>
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() -> R + Send,
        R: Send;
}

/// A simple, sequential, executor.
#[derive(Default)]
pub struct SequentialExecutor;

impl Executor for SequentialExecutor {
    fn execute<I, F, R>(&self, tasks: I) -> impl IntoIterator<Item = R> + use<I, F, R>
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() -> R + Send,
        R: Send,
    {
        tasks.into_iter().map(|f| f())
    }
}

#[cfg(feature = "rayon")]
pub use rayon::RayonExecutor;

#[cfg(feature = "rayon")]
mod rayon {
    use rayon::prelude::*;

    use super::Executor;

    /// A simple parallel executor, using the rayon crate.
    #[derive(Default)]
    pub struct RayonExecutor;

    impl Executor for RayonExecutor {
        fn execute<I, F, R>(&self, tasks: I) -> impl IntoIterator<Item = R> + use<I, F, R>
        where
            I: IntoIterator<Item = F>,
            F: FnOnce() -> R + Send,
            R: Send,
        {
            //  FIXME: Is there no way to bridge _without_ allocation?
            //
            //  (Note that the cost of allocation is likely to matter much for our usecase)

            let tasks: Vec<_> = tasks.into_iter().collect();

            let results: Vec<_> = tasks.into_par_iter().map(|f| f()).collect();

            results
        }
    }
} // mod rayon
