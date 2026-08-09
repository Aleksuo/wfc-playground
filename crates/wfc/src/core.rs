use rand::{prelude::*, rngs::Xoshiro256PlusPlus};
use std::collections::VecDeque;

use crate::{
    Solution,
    model::{
        cell::Cell,
        compiled_model::CompiledModel,
        direction::{ALL_DIRECTIONS, Direction},
        rule_model::FrequencyHints,
        simple_bit_set::SimpleBitSet,
        solver_run_configuration::*,
        solver_state::SolverState,
    },
    util::entropy::calculate_shannon_entropy,
};

pub fn solve(
    model: &CompiledModel,
    run_config: &SolverRunConfiguration,
) -> Result<Solution, SolverError> {
    run_with_contradiction_strategy(
        run_config.seed,
        &run_config.contradiction_strategy,
        |derived_seed| run_attempt(model, run_config, derived_seed),
    )
}

fn run_with_contradiction_strategy(
    run_seed: u64,
    contradiction_strategy: &ContradictionStrategy,
    mut run_attempt: impl FnMut(u64) -> Result<Solution, SolverRunError>,
) -> Result<Solution, SolverError> {
    let max_attempts = match contradiction_strategy {
        ContradictionStrategy::Fail => 1,
        ContradictionStrategy::Retry { max_attempts } => max_attempts.get(),
    };

    for attempt_index in 0..max_attempts {
        let derived_seed = run_seed.wrapping_add(attempt_index as u64);
        if let Ok(res) = run_attempt(derived_seed) {
            return Ok(res);
        }
    }

    Err(SolverError::AttemptsExhausted)
}

fn run_attempt(
    model: &CompiledModel,
    run_config: &SolverRunConfiguration,
    derived_seed: u64,
) -> Result<Solution, SolverRunError> {
    let CompiledModel {
        adj_rules,
        frequency_hints,
        num_patterns,
        num_directions,
    } = model;
    let [output_width, output_height] = run_config.output_dimensions.get();
    let total_output = output_height * output_width;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(derived_seed);
    let mut state = SolverState {
        cells: Vec::new(),
        uncollapsed_num: total_output,
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
            for possible in next_cell.possible_values.into_iter() {
                for direction in ALL_DIRECTIONS {
                    let dir_set = &mut union_map[direction as usize];
                    let rule_idx = possible * num_directions + direction as usize;
                    dir_set.union_with(&adj_rules[rule_idx]);
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
                        0 => return Err(SolverRunError::Contradiction),
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
    Ok(Solution {
        output: state.get_sampled_output(),
        output_dimensions: run_config.output_dimensions,
    })
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
    use std::num::NonZeroU32;

    mod wfc {
        use crate::Dimensions;

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

        fn stub_solution() -> Solution {
            Solution {
                output: vec![42],
                output_dimensions: Dimensions::new([1, 1]).expect("1x1 is non-empty"),
            }
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
            let model = CompiledModel {
                num_patterns: test_freqs.weights.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
                num_directions: 4,
            };
            let run_config = SolverRunConfiguration {
                output_dimensions: Dimensions::new([width, height]).unwrap(),
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

            let model = CompiledModel {
                num_patterns: test_freqs.weights.len(),
                adj_rules: test_ruleset,
                frequency_hints: test_freqs,
                num_directions: 4,
            };
            let run_config = SolverRunConfiguration {
                output_dimensions: Dimensions::new([width, height]).unwrap(),
                seed: seed_1,
                contradiction_strategy: ContradictionStrategy::Fail,
            };
            let first_run = solve(&model, &run_config).unwrap();

            let second_run = solve(
                &model,
                &SolverRunConfiguration {
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
                    Err(SolverRunError::Contradiction)
                });

            assert!(matches!(result, Err(SolverError::AttemptsExhausted)));
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
                    Ok(stub_solution())
                } else {
                    Err(SolverRunError::Contradiction)
                }
            });

            assert_eq!(result.unwrap(), stub_solution());
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
                Err(SolverRunError::Contradiction)
            });

            assert!(matches!(result, Err(SolverError::AttemptsExhausted)));
            assert_eq!(attempted_seeds, vec![10, 11, 12]);
        }
    }
}
