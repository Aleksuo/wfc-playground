use rand::{prelude::*, rngs::Xoshiro256PlusPlus};
use std::{collections::VecDeque, num::NonZeroU32};

use crate::{
    core::ContradictionStrategy::{Fail, Retry},
    model::{
        cell::Cell,
        direction::{ALL_DIRECTIONS, Direction},
        rule_model::{AdjadencyRules, FrequencyHints},
        simple_bit_set::SimpleBitSet,
        wfc_state::WfcState,
    },
    util::entropy::calculate_shannon_entropy,
};

pub enum ContradictionStrategy {
    Fail,
    Retry { max_attempts: NonZeroU32 },
}

#[derive(Debug)]
pub enum WfcError {
    AttemptsExhausted,
}

enum WfcRunError {
    Contradiction,
}

pub struct WfcModel {
    pub adj_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub num_patterns: usize,
}

pub struct WfcRunConfig {
    pub output_width: u32,
    pub output_height: u32,
    pub seed: u64,
    pub contradiction_strategy: ContradictionStrategy,
}

pub fn solve(model: &WfcModel, run_config: &WfcRunConfig) -> Result<Vec<u16>, WfcError> {
    run_with_contradiction_strategy(
        run_config.seed,
        &run_config.contradiction_strategy,
        |derived_seed| run_attempt(model, run_config, derived_seed),
    )
}

fn run_with_contradiction_strategy(
    run_seed: u64,
    contradiction_strategy: &ContradictionStrategy,
    mut run_attempt: impl FnMut(u64) -> Result<Vec<u16>, WfcRunError>,
) -> Result<Vec<u16>, WfcError> {
    let max_attempts = match contradiction_strategy {
        Fail => 1,
        Retry { max_attempts } => max_attempts.get(),
    };

    for attempt_index in 0..max_attempts {
        let derived_seed = run_seed.wrapping_add(attempt_index as u64);
        if let Ok(res) = run_attempt(derived_seed) {
            return Ok(res);
        }
    }

    Err(WfcError::AttemptsExhausted)
}

fn run_attempt(
    model: &WfcModel,
    run_config: &WfcRunConfig,
    derived_seed: u64,
) -> Result<Vec<u16>, WfcRunError> {
    let WfcModel {
        adj_rules,
        frequency_hints,
        num_patterns,
    } = model;
    let output_width = run_config.output_width;
    let output_height = run_config.output_height;
    let total_output = output_height * output_width;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(derived_seed);
    let mut state = WfcState {
        cells: Vec::new(),
        uncollapsed_num: total_output,
        adjadency_rules: adj_rules.clone(),
    };
    let initial_possible_values = SimpleBitSet::full(*num_patterns);
    let initial_entropy = calculate_initial_entropy(frequency_hints);
    for _ in 0..(total_output) {
        state.cells.push(Cell::new(
            initial_possible_values.clone(),
            initial_entropy,
            &mut rng,
        ));
    }

    let mut union_map: [SimpleBitSet; 4] = [
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
        SimpleBitSet::new(*num_patterns),
    ];

    let mut propagation_queue: VecDeque<usize> = VecDeque::new();

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
            for (dir, neighbor_idx) in get_neighbor_indices(next_prop, output_width, output_height)
                .iter()
                .enumerate()
            {
                if let Some(n_idx) = neighbor_idx {
                    let neighbor_cell = &mut state.cells[*n_idx];
                    if neighbor_cell.is_collapsed {
                        continue;
                    }
                    let dir_union = &union_map[dir];
                    let (changed, new_count) = neighbor_cell
                        .possible_values
                        .intersect_with_stats(dir_union);

                    if !changed {
                        continue;
                    }

                    match new_count {
                        // TODO: Implement handling for contradictions
                        0 => return Err(WfcRunError::Contradiction),
                        1 => {
                            neighbor_cell.collapse(frequency_hints, &mut rng);
                            state.uncollapsed_num -= 1;
                            if state.uncollapsed_num != 0 {
                                propagation_queue.push_back(*n_idx);
                            }
                        }
                        _ => {
                            neighbor_cell.calculate_entropy(frequency_hints);
                            propagation_queue.push_back(*n_idx);
                        }
                    }
                }
            }
        }
    }
    Ok(state.get_sampled_output())
}

fn calculate_initial_entropy(frequency_hints: &FrequencyHints) -> f32 {
    let total_weight: f32 = frequency_hints.weights.iter().sum::<u32>() as f32;
    let total_log_weight: f32 = frequency_hints.weighted_logs.iter().sum();
    calculate_shannon_entropy(total_weight, total_log_weight)
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
            let test_freqs = FrequencyHints::new(checkerboard_frequencies());
            let run_seed: u64 = 27;
            let width = 16;
            let height = 16;

            let num_checks = 5;
            let model = WfcModel {
                num_patterns: test_freqs.weights.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
            };
            let run_config = WfcRunConfig {
                output_width: width,
                output_height: height,
                seed: run_seed,
                contradiction_strategy: ContradictionStrategy::Fail,
            };
            let first_run = solve(&model, &run_config).unwrap();

            for _ in 0..num_checks {
                assert_eq!(first_run, solve(&model, &run_config).unwrap())
            }
        }

        #[test]
        fn different_seed_changes_output() {
            let test_ruleset = checkerboard_rules();
            let test_freqs = FrequencyHints::new(checkerboard_frequencies());
            let seed_1: u64 = 27;
            let seed_2: u64 = 10;
            let width = 16;
            let height = 16;

            let model = WfcModel {
                num_patterns: test_freqs.weights.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
            };
            let run_config = WfcRunConfig {
                output_width: width,
                output_height: height,
                seed: seed_1,
                contradiction_strategy: ContradictionStrategy::Fail,
            };
            let first_run = solve(&model, &run_config).unwrap();

            let second_run = solve(
                &model,
                &WfcRunConfig {
                    seed: seed_2,
                    ..run_config
                },
            )
            .unwrap();

            assert_ne!(first_run, second_run);
        }

        #[test]
        fn fail_contradiction_strategy_returns_error_on_first_failure() {
            let mut attempted_seeds = Vec::new();

            let result =
                run_with_contradiction_strategy(10, &ContradictionStrategy::Fail, |seed| {
                    attempted_seeds.push(seed);
                    Err(WfcRunError::Contradiction)
                });

            assert!(matches!(result, Err(WfcError::AttemptsExhausted)));
            assert_eq!(attempted_seeds, vec![10]);
        }

        #[test]
        fn retry_contradiction_strategy_retries_attempts_the_specified_amount() {
            let strategy = ContradictionStrategy::Retry {
                max_attempts: NonZeroU32::new(3).unwrap(),
            };
            let mut attempted_seeds = Vec::new();

            let result = run_with_contradiction_strategy(u64::MAX - 1, &strategy, |seed| {
                attempted_seeds.push(seed);
                if attempted_seeds.len() == 3 {
                    Ok(vec![42])
                } else {
                    Err(WfcRunError::Contradiction)
                }
            });

            assert_eq!(result.unwrap(), vec![42]);
            assert_eq!(attempted_seeds, vec![u64::MAX - 1, u64::MAX, 0]);
        }

        #[test]
        fn retry_contradiction_strategy_exhausts_on_exceeding_max_attempts() {
            let strategy = ContradictionStrategy::Retry {
                max_attempts: NonZeroU32::new(3).unwrap(),
            };
            let mut attempted_seeds = Vec::new();

            let result = run_with_contradiction_strategy(10, &strategy, |seed| {
                attempted_seeds.push(seed);
                Err(WfcRunError::Contradiction)
            });

            assert!(matches!(result, Err(WfcError::AttemptsExhausted)));
            assert_eq!(attempted_seeds, vec![10, 11, 12]);
        }
    }
}
