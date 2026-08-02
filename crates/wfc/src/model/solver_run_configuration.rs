use std::num::NonZeroU32;

use crate::Dimensions;

pub enum ContradictionStrategy {
    Fail,
    Retry { max_attempts: NonZeroU32 },
}

#[derive(Debug)]
pub enum SolverError {
    AttemptsExhausted,
}

pub enum SolverRunError {
    Contradiction,
}

pub struct SolverRunConfiguration {
    pub output_dimensions: Dimensions<2>,
    pub seed: u64,
    pub contradiction_strategy: ContradictionStrategy,
}
