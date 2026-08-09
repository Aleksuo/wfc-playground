pub mod core;
pub mod model;
pub mod preprocessing;
mod util;

pub use crate::core::solve;
pub use crate::model::{
    compiled_model::CompiledModel,
    dimensions::Dimensions,
    pattern::Pattern,
    rule_model::{AdjadencyRules, FrequencyHints, RuleModel},
    sample_lattice::SampleLattice,
    solution::Solution,
    solver_run_configuration::{ContradictionStrategy, SolverError, SolverRunConfiguration},
};
pub use crate::preprocessing::{PatternError, create_rule_model};
