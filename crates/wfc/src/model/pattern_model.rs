use std::collections::HashMap;

use image::Rgb;

use crate::model::{direction::Direction, pattern::Pattern, simple_bit_set::SimpleBitSet};

pub type AdjadencyRules = HashMap<(u16, Direction), SimpleBitSet>;
pub type FrequencyHints = Vec<u32>;
pub struct PatternModel {
    pub palette: Vec<Rgb<u8>>,
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub pattern_height: u32,
    pub pattern_width: u32,
}
