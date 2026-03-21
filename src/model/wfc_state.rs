use crate::model::{
    cell::Cell,
    pattern_model::{AdjadencyRules, FrequencyHints},
};

pub struct WfcState {
    pub cells: Vec<Cell>,
    pub uncollapsed_num: u32,
    pub adjadency_rules: AdjadencyRules,
    pub frequency_hints: FrequencyHints,
}

impl WfcState {
    pub fn get_sampled_output(self) -> Vec<u16> {
        self.cells
            .iter()
            .map(|cell| cell.collapsed_val.unwrap())
            .collect()
    }
}
