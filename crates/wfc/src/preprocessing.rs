use std::collections::{BTreeSet, HashMap};

use image::{DynamicImage, Rgb};

use crate::model::{
    direction::ALL_DIRECTIONS,
    pattern::Pattern,
    pattern_model::{AdjadencyRules, FrequencyHints, PatternModel},
    simple_bit_set::SimpleBitSet,
};

pub fn create_pattern_model(
    img: DynamicImage,
    pattern_width: u32,
    pattern_height: u32,
) -> PatternModel {
    let (width, height, sample, palette) = sample_dynamic_image(&img);
    print_sampled_input(width, height, &sample);
    let (patterns, frequency_hints) =
        find_patterns(pattern_width, pattern_height, width, height, &sample);
    print_patterns(&patterns, &frequency_hints);
    let adjadency_rules = recognize_adjadency_rules(&patterns);
    print_adjadency_rule(&adjadency_rules);
    PatternModel {
        palette,
        patterns,
        adjadency_rules,
        frequency_hints,
        pattern_height,
        pattern_width,
    }
}

fn sample_dynamic_image(img: &DynamicImage) -> (u32, u32, Vec<u16>, Vec<Rgb<u8>>) {
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut sample: Vec<u16> = vec![0; (height * width) as usize];
    let mut palette: Vec<Rgb<u8>> = vec![];
    for (x, y, pixel) in img.enumerate_pixels() {
        let k = match palette.iter().position(|c| c == pixel) {
            Some(i) => i,
            None => {
                palette.push(*pixel);
                palette.len() - 1
            }
        };
        let index = x + y * width;
        sample[index as usize] = k as u16;
    }
    (width, height, sample, palette)
}

fn find_patterns(
    pattern_width: u32,
    pattern_height: u32,
    input_width: u32,
    input_height: u32,
    sampled_input: &[u16],
) -> (Vec<Pattern>, FrequencyHints) {
    // BTreeSet return the patterns Ord sorted, making the vec conversion deterministic.
    let mut patterns: BTreeSet<Pattern> = BTreeSet::new();
    let mut pattern_frequencies: HashMap<Pattern, u32> = HashMap::new();
    let max_width = input_width - pattern_width + 1;
    let max_height = input_height - pattern_height + 1;
    for i in 0..max_height {
        for j in 0..max_width {
            let mut pattern_samples = Vec::new();
            for y in 0..pattern_height {
                for x in 0..pattern_width {
                    let sample_idx = (j + x) + ((i + y) * input_width);
                    pattern_samples.push(sampled_input[sample_idx as usize]);
                }
            }
            let new_pattern = Pattern {
                samples: pattern_samples,
                width: pattern_width,
                height: pattern_height,
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
    let frequency_vec: FrequencyHints = pattern_vec
        .iter()
        .map(|p| *pattern_frequencies.get(p).unwrap())
        .collect();
    (pattern_vec, frequency_vec)
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

fn print_patterns(patterns: &[Pattern], frequencies: &FrequencyHints) {
    println!("Found {} unique patterns:", patterns.len());
    for (i, pattern) in patterns.iter().enumerate() {
        let freq = frequencies.get(i).unwrap_or(&0);
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

fn print_adjadency_rule(adj_rules: &AdjadencyRules) {
    println!("Printing found rules:");
    for (i, rule) in adj_rules.iter().enumerate() {
        println!("{}: {:?}", i, rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_order_is_deterministic() {
        let sampled_input = vec![0, 1, 2, 2, 0, 1, 1, 2, 0];
        let (test_patterns, test_frequencies) = find_patterns(2, 2, 3, 3, &sampled_input);

        assert!(test_patterns.len() > 1);

        for _ in 0..5 {
            let (patterns, frequencies) = find_patterns(2, 2, 3, 3, &sampled_input);

            assert!(test_patterns == patterns);
            assert_eq!(test_frequencies, frequencies);
        }
    }
}
