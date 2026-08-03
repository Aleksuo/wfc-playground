use crate::model::cell::Cell;

pub struct WfcState {
    pub cells: Vec<Cell>,
    pub uncollapsed_num: u32,
}

impl WfcState {
    pub fn get_sampled_output(self) -> Vec<u32> {
        self.cells
            .iter()
            .map(|cell| cell.collapsed_val.unwrap())
            .collect()
    }
}
