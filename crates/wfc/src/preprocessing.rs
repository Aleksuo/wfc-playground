use std::collections::{BTreeSet, HashMap};

use crate::model::{
    dimensions::Dimensions,
    direction::ALL_DIRECTIONS,
    pattern::Pattern,
    rule_model::{AdjadencyRules, FrequencyHints, RuleModel},
    sampled::Sampled,
    simple_bit_set::SimpleBitSet,
};

#[derive(Debug, PartialEq, Eq)]
pub enum PatternError {
    /// The requested pattern is longer than the input on at least one axis, so the input
    /// contains no window to sample.
    PatternLargerThanInput,
}

pub fn create_pattern_model<T>(
    input: &Sampled<T, 2>,
    pattern_dimensions: &Dimensions<2>,
) -> Result<RuleModel, PatternError> {
    let (patterns, frequency_hints) = find_patterns(pattern_dimensions, input)?;
    // print_patterns(&patterns, &frequency_hints);
    let adjadency_rules = recognize_adjadency_rules(&patterns);
    // print_adjadency_rule(&adjadency_rules);
    Ok(RuleModel {
        patterns,
        adjadency_rules,
        frequency_hints,
        num_directions: ALL_DIRECTIONS.len(),
    })
}

fn find_patterns<T>(
    pattern_dimensions: &Dimensions<2>,
    sampled_input: &Sampled<T, 2>,
) -> Result<(Vec<Pattern>, FrequencyHints), PatternError> {
    // BTreeSet return the patterns Ord sorted, making the vec conversion deterministic.
    let mut patterns: BTreeSet<Pattern> = BTreeSet::new();
    let mut pattern_frequencies: HashMap<Pattern, u32> = HashMap::new();
    let windows = sampled_input
        .dimensions()
        .windows(*pattern_dimensions)
        .ok_or(PatternError::PatternLargerThanInput)?;
    for i in 0..windows.get(1) {
        for j in 0..windows.get(0) {
            let mut pattern_samples = Vec::new();
            for y in 0..pattern_dimensions.get(1) {
                for x in 0..pattern_dimensions.get(0) {
                    let sample_idx = sampled_input.dimensions().index_of([j + x, i + y]);
                    pattern_samples.push(sampled_input.indices()[sample_idx]);
                }
            }
            let new_pattern = Pattern {
                samples: pattern_samples,
                width: pattern_dimensions.get(0),
                height: pattern_dimensions.get(1),
            };
            let base_mirrored = new_pattern.rowwise_mirror();
            let pat_rot_90 = new_pattern.rotate(90.0);
            let pat_rot_90_mirrored = pat_rot_90.rowwise_mirror();
            let pat_rot_180 = new_pattern.rotate(180.0);
            let pat_rot_180_mirrored = pat_rot_180.rowwise_mirror();
            let pat_rot_270 = new_pattern.rotate(270.0);
            let pat_rot_270_mirrored = pat_rot_270.rowwise_mirror();

            let new_patterns = vec![
                new_pattern,
                base_mirrored,
                pat_rot_90,
                pat_rot_90_mirrored,
                pat_rot_180,
                pat_rot_180_mirrored,
                pat_rot_270,
                pat_rot_270_mirrored,
            ];
            for pat in new_patterns {
                if patterns.contains(&pat) {
                    let new_val = *pattern_frequencies.get(&pat).unwrap() + 1;
                    pattern_frequencies.insert(pat, new_val);
                } else {
                    patterns.insert(pat.clone());
                    pattern_frequencies.insert(pat, 1);
                }
            }
        }
    }
    let pattern_vec: Vec<Pattern> = patterns.iter().cloned().collect();
    let frequency_vec: Vec<u32> = pattern_vec
        .iter()
        .map(|p| *pattern_frequencies.get(p).unwrap())
        .collect();
    Ok((pattern_vec, FrequencyHints::new(frequency_vec)))
}

fn recognize_adjadency_rules(patterns: &[Pattern]) -> AdjadencyRules {
    let num_patterns = patterns.len();
    let num_directions = ALL_DIRECTIONS.len();
    let mut rules = vec![SimpleBitSet::new(num_patterns); num_patterns * num_directions];
    for i in 0..num_patterns {
        let first_pattern = &patterns[i];
        for (j, second_pattern) in patterns.iter().enumerate() {
            for dir in ALL_DIRECTIONS.iter() {
                if first_pattern.compatible(second_pattern, dir) {
                    rules[i * num_directions + *dir as usize].set(j);
                }
            }
        }
    }
    rules
}

#[allow(dead_code)]
fn print_patterns(patterns: &[Pattern], frequencies: &FrequencyHints) {
    println!("Found {} unique patterns:", patterns.len());
    for (i, pattern) in patterns.iter().enumerate() {
        let freq = frequencies.weights.get(i).unwrap_or(&0);
        println!("  Pattern {} (freq: {}):", i, freq);
        for y in 0..pattern.height {
            print!("    ");
            for x in 0..pattern.width {
                let idx = (x + y * pattern.width) as usize;
                print!("{:2} ", pattern.samples[idx]);
            }
            println!();
        }
    }
}

#[allow(dead_code)]
fn print_sampled_input(width: u32, height: u32, sample_arr: &[u16]) {
    println!("Sampled input:");
    for i in 0..height {
        for j in 0..width {
            let index = j + i * height;
            print!("{} ", sample_arr[index as usize]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_adjadency_rule(adj_rules: &AdjadencyRules) {
    println!("Printing found rules:");
    for (i, rule) in adj_rules.iter().enumerate() {
        println!("{}: {:?}", i, rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_VALUES: [u32; 9] = [0, 1, 2, 2, 0, 1, 1, 2, 0];

    fn sample_3x3() -> Sampled<u32, 2> {
        let dimensions = Dimensions::new([3, 3]).expect("3x3 is non-empty");
        Sampled::from_fn(dimensions, |coord| {
            SAMPLE_VALUES[dimensions.index_of(coord)]
        })
    }

    #[test]
    fn extracts_a_pattern_that_fills_the_input() {
        let pattern_dimensions = Dimensions::new([3, 3]).expect("3x3 is non-empty");
        let sampled_input = sample_3x3();

        let (patterns, _) = find_patterns(&pattern_dimensions, &sampled_input).expect("3x3 fits");

        assert!(patterns.iter().any(|p| p.samples == SAMPLE_VALUES));
    }

    #[test]
    fn rejects_a_pattern_larger_than_the_input() {
        let pattern_dimensions = Dimensions::new([4, 4]).expect("4x4 is non-empty");
        let sampled_input = sample_3x3();

        let result = find_patterns(&pattern_dimensions, &sampled_input);

        assert_eq!(result.err(), Some(PatternError::PatternLargerThanInput));
    }

    #[test]
    fn rejects_a_pattern_larger_than_the_input_on_one_axis() {
        let sampled_input = sample_3x3();

        for lengths in [[4, 2], [2, 4]] {
            let pattern_dimensions = Dimensions::new(lengths).expect("non-empty");

            let result = find_patterns(&pattern_dimensions, &sampled_input);

            assert_eq!(result.err(), Some(PatternError::PatternLargerThanInput));
        }
    }

    #[test]
    fn pattern_order_is_deterministic() {
        let pattern_dimensions = Dimensions::new([2, 2]).expect("2x2 is non-empty");
        let sampled_input = sample_3x3();
        let (test_patterns, test_frequencies) =
            find_patterns(&pattern_dimensions, &sampled_input).expect("2x2 fits");

        assert!(test_patterns.len() > 1);

        for _ in 0..5 {
            let (patterns, frequencies) =
                find_patterns(&pattern_dimensions, &sampled_input).expect("2x2 fits");

            assert!(test_patterns == patterns);
            assert_eq!(test_frequencies, frequencies);
        }
    }
}
