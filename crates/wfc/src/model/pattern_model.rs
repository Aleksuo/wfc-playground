use image::Rgb;

use crate::model::{pattern::Pattern, simple_bit_set::SimpleBitSet};

pub type AdjadencyRules = Vec<SimpleBitSet>;
pub struct PatternModel {
    pub palette: Vec<Rgb<u8>>,
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub pattern_height: u32,
    pub pattern_width: u32,
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
