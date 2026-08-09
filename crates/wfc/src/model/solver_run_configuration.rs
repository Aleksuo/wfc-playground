use std::{
    fmt::{self},
    num::NonZeroU32,
};

use crate::Dimensions;

pub enum ContradictionStrategy {
    Fail,
    Retry { max_attempts: NonZeroU32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverError {
    AttemptsExhausted,
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverError::AttemptsExhausted => write!(f, "Maximum number of retries was reached."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolverRunError {
    Contradiction,
}

impl fmt::Display for SolverRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverRunError::Contradiction => write!(f, "Solver run ran into a contradiction!"),
        }
    }
}

pub struct SolverRunConfiguration {
    pub output_dimensions: Dimensions<2>,
    pub seed: u64,
    pub contradiction_strategy: ContradictionStrategy,
}
