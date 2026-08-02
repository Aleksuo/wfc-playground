use crate::model::rule_model::{AdjadencyRules, FrequencyHints};

pub struct CompiledModel {
    pub adj_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_patterns: usize,
}
