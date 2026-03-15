use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops,
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

pub fn get_dir_vecs() -> HashMap<Direction, Vec2> {
    HashMap::from([
        (Direction::Up, Vec2::new(0, -1)),
        (Direction::Down, Vec2::new(0, 1)),
        (Direction::Right, Vec2::new(1, 0)),
        (Direction::Left, Vec2::new(-1, 0)),
    ])
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Pattern {
    pub samples: Vec<u16>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
