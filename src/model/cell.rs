use std::collections::HashSet;

use rand::{Rng, RngExt};

use crate::model::pattern_model::FrequencyHints;

pub struct Cell {
    pub possible_values: HashSet<u16>,
    pub collapsed_val: Option<u16>,
    pub entropy: Option<f32>,
    pub is_collapsed: bool,
}

impl Cell {
    pub fn calculate_entropy(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let total_weight: f32 = {
            let mut total = 0;
            for (_, possible_sample_val) in self.possible_values.iter().enumerate() {
                total += frequency_hints.get(possible_sample_val).unwrap();
            }
            total as f32
        };
        let log_weight = {
            let mut total = 0.0;
            for (_, possible_sample_val) in self.possible_values.iter().enumerate() {
                let freq = *frequency_hints.get(possible_sample_val).unwrap() as f32;
                total += freq * freq.log2();
            }
            total as f32
        };
        let tie_breaker_noise = rng.random_range(0.0..1e-6);
        self.entropy =
            Some((total_weight.log2() - (log_weight / total_weight)) + tie_breaker_noise);
    }
    pub fn collapse(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let total_weight: u32 = self
            .possible_values
            .iter()
            .map(|v| frequency_hints.get(v).unwrap())
            .sum();
        let roll = rng.random_range(0..total_weight);
        let mut sum = 0;
        let mut chosen = *self.possible_values.iter().next().unwrap();
        for val in self.possible_values.iter() {
            let weight = *frequency_hints.get(val).unwrap();
            sum += weight;
            if sum > roll {
                chosen = *val;
                break;
            }
        }
        self.possible_values = HashSet::from([chosen]);
        self.collapsed_val = Some(chosen);
        self.is_collapsed = true;
    }
}
