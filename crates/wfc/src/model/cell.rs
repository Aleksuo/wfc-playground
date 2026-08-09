use rand::{Rng, RngExt};

use crate::{
    model::{rule_model::FrequencyHints, simple_bit_set::SimpleBitSet},
    util::entropy::calculate_shannon_entropy,
};

pub struct Cell {
    pub possible_values: SimpleBitSet,
    pub collapsed_val: Option<u32>,
    pub entropy: f32,
    pub is_collapsed: bool,
    pub tie_breaker_noise: f32,
}

impl Cell {
    pub fn new(
        initial_possible_values: SimpleBitSet,
        initial_entropy: f32,
        rng: &mut impl Rng,
    ) -> Self {
        let tie_breaker_noise = rng.random_range(0.0..1e-6);
        Self {
            possible_values: initial_possible_values,
            collapsed_val: None,
            entropy: initial_entropy + tie_breaker_noise,
            is_collapsed: false,
            tie_breaker_noise,
        }
    }

    pub fn calculate_entropy(&mut self, frequency_hints: &FrequencyHints) {
        let mut weight_sum = 0u32;
        let mut weighted_log_sum = 0.0f32;
        for val in self.possible_values.into_iter() {
            weight_sum += frequency_hints.weights[val];
            weighted_log_sum += frequency_hints.weighted_logs[val];
        }
        self.entropy =
            calculate_shannon_entropy(weight_sum as f32, weighted_log_sum) + self.tie_breaker_noise;
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
        self.collapsed_val = Some(chosen as u32);
        self.is_collapsed = true;
    }
}
