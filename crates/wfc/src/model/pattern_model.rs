use image::Rgb;

use crate::model::{pattern::Pattern, simple_bit_set::SimpleBitSet};

pub type AdjadencyRules = Vec<SimpleBitSet>;
pub type FrequencyHints = Vec<u32>;
pub struct PatternModel {
    pub palette: Vec<Rgb<u8>>,
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub pattern_height: u32,
    pub pattern_width: u32,
}
