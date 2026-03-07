use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops, vec,
};

use image::{DynamicImage, ImageReader, Rgb};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn get_dir_vecs() -> HashMap<Direction, Vec2> {
    HashMap::from([
        (Direction::Up, Vec2::new(0, -1)),
        (Direction::Down, Vec2::new(0, 1)),
        (Direction::Right, Vec2::new(1, 0)),
        (Direction::Left, Vec2::new(-1, 0)),
    ])
}

#[derive(Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/test_input.bmp")?.decode()?;
    let (_sample_map, _adjadency_rules) = overlap_model(input_img);
    Ok(())
}

fn overlap_model(img: DynamicImage) -> (Vec<u16>, HashMap<(u16, Direction), HashSet<u16>>) {
    let (width, height, sample) = sample_dynamic_image(&img);
    print_sampled_input(width, height, &sample);
    let adjadency_rules = recognize_adjadency_rules(width, height, &sample);
    print_adjadency_rule(&adjadency_rules);
    (sample, adjadency_rules)
}

fn sample_dynamic_image(img: &DynamicImage) -> (u32, u32, Vec<u16>) {
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut sample: Vec<u16> = vec![0; (height * width) as usize];
    let mut colors: Vec<String> = vec![];
    for (x, y, pixel) in img.enumerate_pixels() {
        let pixel_string = rgb8_to_string(*pixel);
        let k = match colors.iter().position(|c| c == &pixel_string) {
            Some(i) => i,
            None => {
                colors.push(pixel_string);
                colors.len() - 1
            }
        };
        let index = x + y * width;
        sample[index as usize] = k as u16;
    }
    (width, height, sample)
}

fn recognize_adjadency_rules(
    width: u32,
    height: u32,
    samples: &Vec<u16>,
) -> HashMap<(u16, Direction), HashSet<u16>> {
    let dir_vecs = get_dir_vecs();
    let mut adjadency_map: HashMap<(u16, Direction), HashSet<u16>> = HashMap::new();
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

fn rgb8_to_string(pixel: Rgb<u8>) -> String {
    format!("{},{},{}", pixel[0], pixel[1], pixel[2])
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

fn print_adjadency_rule(adj_rules: &HashMap<(u16, Direction), HashSet<u16>>) {
    println!("Printing found rules:");
    for rule in adj_rules.iter().enumerate() {
        println!("{:?}", rule.1)
    }
}
