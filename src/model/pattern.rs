use crate::model::direction::Direction;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Pattern {
    pub samples: Vec<u16>,
    pub width: u32,
    pub height: u32,
}

impl Pattern {
    pub fn compatible(&self, other: &Pattern, direction: &Direction) -> bool {
        match direction {
            Direction::Up => {
                for row in 0..self.height - 1 {
                    for col in 0..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = (row + 1) * other.width + col;
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Down => {
                for row in 1..self.height {
                    for col in 0..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = (row - 1) * other.width + col;
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Right => {
                for row in 0..self.height {
                    for col in 1..self.width {
                        let self_idx = row * self.width + col;
                        let other_idx = row * other.width + (col - 1);
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
            Direction::Left => {
                for row in 0..self.height {
                    for col in 0..self.width - 1 {
                        let self_idx = row * self.width + col;
                        let other_idx = row * other.width + (col + 1);
                        if self.samples[self_idx as usize] != other.samples[other_idx as usize] {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    }
}
