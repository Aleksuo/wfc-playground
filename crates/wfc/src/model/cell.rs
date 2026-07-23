use rand::{Rng, RngExt};

use crate::model::{pattern_model::FrequencyHints, simple_bit_set::SimpleBitSet};

pub struct Cell {
    pub possible_values: SimpleBitSet,
    pub collapsed_val: Option<u16>,
    pub entropy: Option<f32>,
    pub is_collapsed: bool,
    pub tie_breaker_noise: f32,
}

impl Cell {
    pub fn calculate_entropy(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let mut total_weight = 0u32;
        let mut weighted_log_sum = 0.0f32;
        for val in self.possible_values.into_iter() {
            total_weight += frequency_hints.weights[val];
            weighted_log_sum += frequency_hints.weighted_logs[val];
        }
        let total_weight = total_weight as f32;
        self.entropy = Some(
            (total_weight.log2() - (weighted_log_sum / total_weight)) + self.tie_breaker_noise,
        );
    }
    pub fn collapse(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
        let total_weight: u32 = self
            .possible_values
            .into_iter()
            .map(|v| frequency_hints.weights[v])
            .sum();
        let roll = rng.random_range(0..total_weight);
        let mut sum = 0;
        let mut chosen = self.possible_values.into_iter().next().unwrap();
        for val in self.possible_values.into_iter() {
            let weight = frequency_hints.weights[val];
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
