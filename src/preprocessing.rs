use std::{
    collections::{HashMap, HashSet},
    vec,
};

use image::{DynamicImage, Rgb};

use crate::model::{AdjadencyRules, FrequencyHints, Vec2, get_dir_vecs};

pub fn overlap_model(img: DynamicImage) -> (Vec<Rgb<u8>>, AdjadencyRules, FrequencyHints) {
    let (width, height, sample, palette) = sample_dynamic_image(&img);
    print_sampled_input(width, height, &sample);
    let frequency_hints = calculate_frequency_hints(&sample);
    print_frequency_hints(&frequency_hints);
    let adjadency_rules = recognize_adjadency_rules(width, height, &sample);
    print_adjadency_rule(&adjadency_rules);
    (palette, adjadency_rules, frequency_hints)
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

fn calculate_frequency_hints(sample_arr: &Vec<u16>) -> FrequencyHints {
    let mut frequency_hints: FrequencyHints = HashMap::new();
    for val in sample_arr {
        let maybe_cur_freq = frequency_hints.get(val);
        if let Some(cur_freq) = maybe_cur_freq {
            frequency_hints.insert(*val, *cur_freq + 1);
        } else {
            frequency_hints.insert(*val, 1);
        }
    }
    frequency_hints
}

fn recognize_adjadency_rules(width: u32, height: u32, samples: &Vec<u16>) -> AdjadencyRules {
    let dir_vecs = get_dir_vecs();
    let mut adjadency_map: AdjadencyRules = HashMap::new();
    for i in 0..height {
        for j in 0..width {
            let cur_pos = Vec2::new(j as i32, i as i32);
            let cur_index = xy_index(&cur_pos, width);
            let cur_sample = get_sample(cur_index, &samples).unwrap();
            for (dir, dir_vec) in dir_vecs.iter() {
                let dir_pos = cur_pos.clone() + dir_vec.clone();
                let dir_index = xy_index(&dir_pos, width);
                let dir_sample = get_sample(dir_index, &samples);
                if let Some(s) = dir_sample {
                    let adj = adjadency_map.get_mut(&(cur_sample, *dir));
                    if let Some(rules) = adj {
                        rules.insert(s);
                    } else {
                        adjadency_map.insert((cur_sample, *dir), HashSet::from([s]));
                    }
                }
            }
        }
    }
    adjadency_map
}

fn xy_index(coord: &Vec2, width: u32) -> i32 {
    coord.x + coord.y * width as i32
}

fn get_sample(index: i32, sample_arr: &Vec<u16>) -> Option<u16> {
    if index < 0 || index as usize > sample_arr.len() - 1 {
        return None;
    }
    return Some(sample_arr[index as usize]);
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

fn print_frequency_hints(frequency_hints: &FrequencyHints) {
    println!("Printing frequencies:");
    for freq in frequency_hints.iter().enumerate() {
        println!("{:?}", freq.1);
    }
}

fn print_adjadency_rule(adj_rules: &AdjadencyRules) {
    println!("Printing found rules:");
    for rule in adj_rules.iter().enumerate() {
        println!("{:?}", rule.1);
    }
}
