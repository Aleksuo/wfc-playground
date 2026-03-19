use std::{
    collections::{HashMap, HashSet},
    vec,
};

use image::{DynamicImage, Rgb};

use crate::model::{ALL_DIRECTIONS, AdjadencyRules, FrequencyHints, Pattern};

pub struct PatternModel {
    pub palette: Vec<Rgb<u8>>,
    pub patterns: Vec<Pattern>,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
    pub pattern_height: u32,
    pub pattern_width: u32,
}

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
    sampled_input: &Vec<u16>,
) -> (Vec<Pattern>, FrequencyHints) {
    let mut patterns: HashSet<Pattern> = HashSet::new();
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
            if patterns.contains(&new_pattern) {
                let new_val = *pattern_frequencies.get(&new_pattern).unwrap() + 1;
                pattern_frequencies.insert(new_pattern, new_val);
            } else {
                patterns.insert(new_pattern.clone());
                pattern_frequencies.insert(new_pattern, 1);
            }
        }
    }
    let pattern_vec: Vec<Pattern> = patterns
        .iter()
        .enumerate()
        .map(|(_, p)| p)
        .cloned()
        .collect();
    let indexed_frequency_hints: FrequencyHints = {
        let mut idx_freq_hints = HashMap::new();
        for (i, pattern) in pattern_vec.iter().enumerate() {
            let freq = *pattern_frequencies.get(pattern).unwrap();
            idx_freq_hints.insert(i as u16, freq);
        }
        idx_freq_hints
    };
    (pattern_vec, indexed_frequency_hints)
}

fn recognize_adjadency_rules(patterns: &Vec<Pattern>) -> AdjadencyRules {
    let mut adjadency_map: AdjadencyRules = HashMap::new();
    for i in 0..patterns.len() {
        let first_pattern = &patterns[i];
        for j in 0..patterns.len() {
            let second_pattern = &patterns[j];
            for (_, dir) in ALL_DIRECTIONS.iter().enumerate() {
                if first_pattern.compatible(&second_pattern, &dir) {
                    let maybe_rules = adjadency_map.get_mut(&(i as u16, *dir));
                    if let Some(rules) = maybe_rules {
                        rules.insert(j as u16);
                    } else {
                        adjadency_map.insert((i as u16, *dir), HashSet::from([j as u16]));
                    }
                }
            }
        }
    }
    adjadency_map
}

fn print_patterns(patterns: &Vec<Pattern>, frequencies: &FrequencyHints) {
    println!("Found {} unique patterns:", patterns.len());
    for (i, pattern) in patterns.iter().enumerate() {
        let freq = frequencies.get(&(i as u16)).unwrap_or(&0);
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

fn print_sampled_input(width: u32, height: u32, sample_arr: &Vec<u16>) {
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
    for rule in adj_rules.iter().enumerate() {
        println!("{:?}", rule.1);
    }
}
