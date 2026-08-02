use crate::model::{pattern::Pattern, simple_bit_set::SimpleBitSet};

pub type AdjadencyRules = Vec<SimpleBitSet>;
pub struct RuleModel {
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
}
#[derive(PartialEq, Debug)]
pub struct FrequencyHints {
    pub weights: Vec<u32>,
    pub weighted_logs: Vec<f32>,
}

impl FrequencyHints {
    pub fn new(weights: Vec<u32>) -> Self {
        let weighted_logs = weights
            .iter()
            .map(|w| {
                let weight = *w as f32;
                weight * weight.log2()
            })
            .collect();
        Self {
            weights,
            weighted_logs,
        }
    }
}
