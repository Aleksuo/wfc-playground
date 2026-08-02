use std::num::NonZeroU32;

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
    pub output_width: u32,
    pub output_height: u32,
    pub seed: u64,
    pub contradiction_strategy: ContradictionStrategy,
}
