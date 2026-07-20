use rand::{prelude::*, rngs::Xoshiro256PlusPlus};
use std::collections::VecDeque;

use crate::model::{
    cell::Cell,
    direction::{ALL_DIRECTIONS, Direction},
    pattern_model::{AdjadencyRules, FrequencyHints},
    simple_bit_set::SimpleBitSet,
    wfc_state::WfcState,
};

pub struct WfcConfig {
    pub output_width: u32,
    pub output_height: u32,
    pub adj_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_patterns: usize,
    pub seed: u64,
}

pub fn wfc(config: &WfcConfig) -> Vec<u16> {
    let WfcConfig {
        output_width,
        output_height,
        adj_rules,
        frequency_hints,
        num_patterns,
        seed,
    } = config;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(*seed);
    let mut state = WfcState {
        cells: Vec::new(),
        uncollapsed_num: output_width * output_height,
        adjadency_rules: adj_rules.clone(),
    };
    let possible_values = SimpleBitSet::full(*num_patterns);
    for _ in 0..(output_height * output_width) {
        let mut new_cell = Cell {
            possible_values: possible_values.clone(),
            entropy: None,
            is_collapsed: false,
            collapsed_val: None,
        };
        new_cell.calculate_entropy(frequency_hints, &mut rng);
        state.cells.push(new_cell);
    }

    let mut union_map: [SimpleBitSet; 4] = [
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
    ];

    while state.uncollapsed_num > 0 {
        // Find a cell to collapse
        let cell_to_collapse_idx = state
            .cells
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.is_collapsed)
            .min_by(|(_, a), (_, b)| a.entropy.partial_cmp(&b.entropy).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        state.cells[cell_to_collapse_idx].collapse(frequency_hints, &mut rng);
        state.uncollapsed_num -= 1;
        // Init propagation queue with the collapsed cell
        let mut propagation_queue: VecDeque<usize> = VecDeque::new();
        propagation_queue.push_back(cell_to_collapse_idx);
        // While propagation queue is not empty propagate
        while let Some(next_prop) = propagation_queue.pop_front() {
            let next_cell = &state.cells[next_prop];
            union_map.iter_mut().for_each(|f| f.clear_all());

            // Construct union map of all possible values in each direction for the cell
            let num_directions = ALL_DIRECTIONS.len();
            for possible in next_cell.possible_values.into_iter() {
                for direction in ALL_DIRECTIONS {
                    let dir_set = &mut union_map[direction as usize];
                    let rule_idx = possible * num_directions + direction as usize;
                    dir_set.union_with(&state.adjadency_rules[rule_idx]);
                }
            }
            // Iterate neigbors and intersect with the union set
            for (dir, neighbor_idx) in
                get_neighbor_indices(next_prop, *output_width, *output_height)
                    .iter()
                    .enumerate()
            {
                if let Some(n_idx) = neighbor_idx {
                    let neighbor_cell = &mut state.cells[*n_idx];
                    if neighbor_cell.is_collapsed {
                        continue;
                    }
                    let dir_union = &union_map[dir];
                    let possible_val_len = neighbor_cell.possible_values.count();
                    // println!("Union {:?} {:?}", &dir, &union_map.get(&dir));
                    // println!("Neighbor possible: {:?}", &neighbor_cell.possible_values);
                    neighbor_cell.possible_values.intersect_with(dir_union);

                    let new_possible_val_len = neighbor_cell.possible_values.count();
                    neighbor_cell.calculate_entropy(frequency_hints, &mut rng);
                    if new_possible_val_len == 0 {
                        // TODO: Implement handling for contradictions
                        panic!("Contradiction");
                    } else if new_possible_val_len == 1 && !neighbor_cell.is_collapsed {
                        neighbor_cell.collapse(frequency_hints, &mut rng);
                        state.uncollapsed_num -= 1;
                        if state.uncollapsed_num != 0 {
                            propagation_queue.push_back(*n_idx);
                        }
                    } else if possible_val_len > neighbor_cell.possible_values.count() {
                        propagation_queue.push_back(*n_idx);
                    }
                }
            }
        }
    }
    state.get_sampled_output()
}

#[inline(always)]
fn get_neighbor_indices(index: usize, width: u32, height: u32) -> [Option<usize>; 4] {
    let x = (index as u32) % width;
    let y = (index as u32) / width;
    let mut neighbors: [Option<usize>; 4] = [None; 4];
    if x > 0 {
        neighbors[Direction::Left as usize] = Some(index - 1);
    }
    if x + 1 < width {
        neighbors[Direction::Right as usize] = Some(index + 1);
    }
    if y > 0 {
        neighbors[Direction::Up as usize] = Some(index - width as usize);
    }
    if y + 1 < height {
        neighbors[Direction::Down as usize] = Some(index + width as usize);
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    mod wfc {
        use super::*;

        fn checkerboard_rules() -> Vec<SimpleBitSet> {
            let num_patterns = 4;
            let num_directions = ALL_DIRECTIONS.len();
            let mut rules = vec![SimpleBitSet::new(num_patterns); num_patterns * num_directions];

            for dir in ALL_DIRECTIONS {
                for pattern in 0..2 {
                    rules[pattern * num_directions + dir as usize].set(2);
                    rules[pattern * num_directions + dir as usize].set(3);
                }
                for pattern in 2..4 {
                    rules[pattern * num_directions + dir as usize].set(0);
                    rules[pattern * num_directions + dir as usize].set(1);
                }
            }
            rules
        }

        fn checkerboard_frequencies() -> Vec<u32> {
            vec![1, 1, 1, 1]
        }

        #[test]
        fn output_is_deterministic_with_same_seed() {
            let test_ruleset = checkerboard_rules();
            let test_freqs = checkerboard_frequencies();
            let seed: u64 = 27;
            let width = 16;
            let height = 16;

            let num_checks = 5;
            let config = WfcConfig {
                output_width: width,
                output_height: height,
                num_patterns: test_freqs.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
                seed,
            };
            let first_run = wfc(&config);

            for _ in 0..num_checks {
                assert_eq!(first_run, wfc(&config))
            }
        }

        #[test]
        fn different_seed_changes_output() {
            let test_ruleset = checkerboard_rules();
            let test_freqs = checkerboard_frequencies();
            let seed_1: u64 = 27;
            let seed_2: u64 = 10;
            let width = 16;
            let height = 16;

            let config = WfcConfig {
                output_width: width,
                output_height: height,
                num_patterns: test_freqs.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
                seed: seed_1,
            };
            let first_run = wfc(&config);

            let second_run = wfc(&WfcConfig {
                seed: seed_2,
                ..config
            });

            assert_ne!(first_run, second_run);
        }
    }
}
