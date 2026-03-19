use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use rand::{Rng, RngExt};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Pattern {
    pub samples: Vec<u16>,
    pub width: u32,
    pub height: u32,
}

impl Pattern {
    pub fn compatible(&self, other: &Pattern, direction: &Direction) -> bool {
        match direction {
            Direction::Up => {
                for row in 0..self.height - 1 {
                    for col in 0..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = (row + 1) * other.width + col;
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Down => {
                for row in 1..self.height {
                    for col in 0..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = (row - 1) * other.width + col;
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Right => {
                for row in 0..self.height {
                    for col in 1..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = row * other.width + (col - 1);
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Left => {
                for row in 0..self.height {
                    for col in 0..self.width - 1 {
                        let self_idx = row * self.width + col;
                        let other_idx = row * other.width + (col + 1);
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    }
}

pub type AdjadencyRules = HashMap<(u16, Direction), HashSet<u16>>;
pub type FrequencyHints = HashMap<u16, u32>;

pub struct WfcState {
    pub cells: Vec<Cell>,
    pub uncollapsed_num: u32,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
}

impl WfcState {
    pub fn get_sampled_output(self) -> Vec<u16> {
        self.cells
            .iter()
            .map(|cell| cell.collapsed_val.unwrap())
            .collect()
    }
}

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
