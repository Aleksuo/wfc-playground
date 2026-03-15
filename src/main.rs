use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops, vec,
};

use image::{DynamicImage, ImageBuffer, ImageReader, Rgb, RgbImage};
use rand::{Rng, RngExt};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

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

type AdjadencyRules = HashMap<(u16, Direction), HashSet<u16>>;
type FrequencyHints = HashMap<u16, u32>;

struct WfcState {
    cells: Vec<Cell>,
    uncollapsed_num: u32,
    adjadency_rules: AdjadencyRules,
    frequency_hints: FrequencyHints,
}

impl WfcState {
    fn get_sampled_output(self) -> Vec<u16> {
        self.cells
            .iter()
            .map(|cell| cell.collapsed_val.unwrap())
            .collect()
    }
}

struct Cell {
    possible_values: HashSet<u16>,
    collapsed_val: Option<u16>,
    entropy: Option<f32>,
    is_collapsed: bool,
}

impl Cell {
    fn calculate_entropy(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
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
    fn collapse(&mut self, frequency_hints: &FrequencyHints, rng: &mut impl Rng) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_img = ImageReader::open("./input/beach.bmp")?.decode()?;
    let (palette, adjadency_rules, frequency_hints) = overlap_model(input_img);
    let output_width = 64;
    let output_height = 64;
    let max_val = (palette.len() - 1) as u16;
    let output = wfc(
        output_width,
        output_height,
        &adjadency_rules,
        &frequency_hints,
        max_val,
    );
    let img = reconstruct_image(&output, output_width, output_height, &palette);
    std::fs::create_dir_all(".output")?;
    img.save(".output/output.bmp")?;
    Ok(())
}

fn reconstruct_image(
    output: &Vec<u16>,
    width: u32,
    height: u32,
    palette: &Vec<Rgb<u8>>,
) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let idx = (x + y * width) as usize;
            let color = palette[output[idx] as usize];
            img.put_pixel(x, y, color);
        }
    }
    img
}

fn wfc(
    output_width: u32,
    output_height: u32,
    adj_rules: &HashMap<(u16, Direction), HashSet<u16>>,
    frequency_hints: &FrequencyHints,
    max_val: u16,
) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut state = WfcState {
        cells: Vec::new(),
        uncollapsed_num: output_width * output_height,
        adjadency_rules: adj_rules.clone(),
        frequency_hints: frequency_hints.clone(),
    };
    let possible_values = HashSet::from_iter(0..=max_val);
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

    while state.uncollapsed_num > 0 {
        if state.uncollapsed_num % 100 == 0 {
            println!("Reimaining uncollapsed cells: {}", state.uncollapsed_num);
        }

        // Find a cell to collapse
        let cell_to_collapse_idx = state
            .cells
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.is_collapsed)
            .min_by(|(_, a), (_, b)| a.entropy.partial_cmp(&b.entropy).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        state.cells[cell_to_collapse_idx].collapse(&frequency_hints, &mut rng);
        state.uncollapsed_num -= 1;
        // Init propagation queue with the collapsed cell
        let mut propagation_queue: VecDeque<usize> = VecDeque::new();
        propagation_queue.push_back(cell_to_collapse_idx);
        // While propagation queue is not empty propagate
        while let Some(next_prop) = propagation_queue.pop_front() {
            let next_cell = &state.cells[next_prop];
            let mut union_map: HashMap<Direction, HashSet<u16>> = HashMap::from([
                (Direction::Up, HashSet::new()),
                (Direction::Right, HashSet::new()),
                (Direction::Left, HashSet::new()),
                (Direction::Down, HashSet::new()),
            ]);
            // Construct union map of all possible values in each direction for the cell
            for (_, possible) in next_cell.possible_values.iter().enumerate() {
                for direction in ALL_DIRECTIONS {
                    let dir_set = union_map.get_mut(&direction).unwrap();
                    if let Some(possible_adj) = state.adjadency_rules.get(&(*possible, direction)) {
                        dir_set.extend(possible_adj);
                    }
                }
            }
            // Iterate neigbors and intersect with the union set
            for (dir, neighbor_idx) in get_neighbor_indices(next_prop, output_width, output_height)
            {
                let neighbor_cell = &mut state.cells[neighbor_idx];
                if neighbor_cell.is_collapsed {
                    continue;
                }
                let dir_union = union_map.get(&dir).unwrap();
                let possible_val_len = neighbor_cell.possible_values.len();
                // println!("Union {:?} {:?}", &dir, &union_map.get(&dir));
                // println!("Neighbor possible: {:?}", &neighbor_cell.possible_values);
                neighbor_cell.possible_values = neighbor_cell
                    .possible_values
                    .intersection(dir_union)
                    .cloned()
                    .collect();

                let new_possible_val_len = neighbor_cell.possible_values.len();
                neighbor_cell.calculate_entropy(frequency_hints, &mut rng);
                if new_possible_val_len == 0 {
                    // TODO: Implement handling for contradictions
                    panic!("Contradiction");
                } else if new_possible_val_len == 1 && !neighbor_cell.is_collapsed {
                    neighbor_cell.collapse(&frequency_hints, &mut rng);
                    state.uncollapsed_num -= 1;
                    if state.uncollapsed_num != 0 {
                        propagation_queue.push_back(neighbor_idx);
                    }
                } else if possible_val_len > neighbor_cell.possible_values.len() {
                    propagation_queue.push_back(neighbor_idx);
                }
            }
        }
    }
    state.get_sampled_output()
}

fn get_neighbor_indices(index: usize, width: u32, height: u32) -> Vec<(Direction, usize)> {
    let x = (index as u32) % width;
    let y = (index as u32) / width;
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((Direction::Left, index - 1));
    }
    if x + 1 < width {
        neighbors.push((Direction::Right, index + 1));
    }
    if y > 0 {
        neighbors.push((Direction::Up, index - width as usize));
    }
    if y + 1 < height {
        neighbors.push((Direction::Down, index + width as usize));
    }
    neighbors
}

fn overlap_model(img: DynamicImage) -> (Vec<Rgb<u8>>, AdjadencyRules, FrequencyHints) {
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
