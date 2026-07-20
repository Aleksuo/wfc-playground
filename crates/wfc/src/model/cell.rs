use rand::{Rng, RngExt};

use crate::model::{pattern_model::FrequencyHints, simple_bit_set::SimpleBitSet};

pub struct Cell {
    pub possible_values: SimpleBitSet,
    pub collapsed_val: Option<u16>,
    pub entropy: Option<f32>,
    pub is_collapsed: bool,
}

impl Cell {
    pub fn calculate_entropy(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let total_weight: f32 = {
            let mut total = 0;
            for possible_sample_val in self.possible_values.into_iter() {
                total += frequency_hints[possible_sample_val];
            }
            total as f32
        };
        let log_weight = {
            let mut total = 0.0;
            for possible_sample_val in self.possible_values.into_iter() {
                let freq = frequency_hints[possible_sample_val] as f32;
                total += freq * freq.log2();
            }
            total
        };
        let tie_breaker_noise = rng.random_range(0.0..1e-6);
        self.entropy =
            Some((total_weight.log2() - (log_weight / total_weight)) + tie_breaker_noise);
    }
    pub fn collapse(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let total_weight: u32 = self
            .possible_values
            .into_iter()
            .map(|v| frequency_hints[v])
            .sum();
        let roll = rng.random_range(0..total_weight);
        let mut sum = 0;
        let mut chosen = self.possible_values.into_iter().next().unwrap();
        for val in self.possible_values.into_iter() {
            let weight = frequency_hints[val];
            sum += weight;
            if sum > roll {
                chosen = val;
                break;
            }
        }
        self.possible_values.clear_all();
        self.possible_values.set(chosen);
        self.collapsed_val = Some(chosen as u16);
        self.is_collapsed = true;
    }
}
