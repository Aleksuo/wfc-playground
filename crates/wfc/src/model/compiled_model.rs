use crate::model::rule_model::{AdjadencyRules, FrequencyHints};

#[derive(Debug)]
pub struct CompiledModel {
    pub adj_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_patterns: usize,
    pub num_directions: usize,
}
