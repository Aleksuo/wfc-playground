use std::collections::{HashMap, HashSet, VecDeque};

use crate::model::{
    cell::Cell,
    direction::{ALL_DIRECTIONS, Direction},
    pattern_model::FrequencyHints,
    wfc_state::WfcState,
};

pub fn wfc(
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
        if state.uncollapsed_num.is_multiple_of(100) {
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
        state.cells[cell_to_collapse_idx].collapse(frequency_hints, &mut rng);
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
            for possible in next_cell.possible_values.iter() {
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
                    neighbor_cell.collapse(frequency_hints, &mut rng);
                    state.uncollapsed_num -= 1;
                    println!("Remaining uncollapsed: {}", state.uncollapsed_num);
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
