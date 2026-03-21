use std::collections::{HashMap, HashSet};

use image::Rgb;

use crate::model::{direction::Direction, pattern::Pattern};

pub type AdjadencyRules = HashMap<(u16, Direction), HashSet<u16>>;
pub type FrequencyHints = HashMap<u16, u32>;
pub struct PatternModel {
    pub palette: Vec<Rgb<u8>>,
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub pattern_height: u32,
    pub pattern_width: u32,
}
