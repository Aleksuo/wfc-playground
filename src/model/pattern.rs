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

    pub fn rowwise_mirror(&self) -> Self {
        let mut res_vec = Vec::with_capacity(self.samples.len());
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                let sample_idx = self.width * y + x;
                let sample = self.samples[sample_idx as usize];
                res_vec.push(sample);
            }
        }
        Pattern {
            width: self.width,
            height: self.height,
            samples: res_vec,
        }
    }

    pub fn rotate(&self, degrees: f32) -> Self {
        let mut rotated_samples = vec![0; self.samples.len()];
        let radians = degrees.to_radians();
        let rad_sin = radians.sin();
        let rad_cos = radians.cos();
        let max_width = (self.width - 1) as f32;
        let max_height = (self.height - 1) as f32;

        // Translate rotated values to the positive range using minimums from corners
        let min_corner_x = {
            let c_0_x: f32 = 0.0;
            let c_1_x = max_width * rad_cos;
            let c_2_x = -max_height * rad_sin;
            let c_3_x = max_width * rad_cos - max_height * rad_sin;
            c_0_x.min(c_1_x).min(c_2_x).min(c_3_x)
        };
        let min_corner_y = {
            let c_0_y: f32 = 0.0;
            let c_1_y = max_width * rad_sin;
            let c_2_y = max_height * rad_cos;
            let c_3_y = max_width * rad_sin + max_height * rad_cos;
            c_0_y.min(c_1_y).min(c_2_y).min(c_3_y)
        };
        for y in 0..self.height {
            for x in 0..self.width {
                let sample_idx = y * self.width + x;
                let sample = self.samples[sample_idx as usize];
                let f_x = x as f32;
                let f_y = y as f32;
                let rot_x = (f_x * rad_cos - f_y * rad_sin) - min_corner_x;
                let rot_y = (f_x * rad_sin + f_y * rad_cos) - min_corner_y;
                let rot_idx = (rot_x.round() as u32) + (rot_y.round() as u32) * self.width;
                rotated_samples[rot_idx as usize] = sample;
            }
        }
        Pattern {
            samples: rotated_samples,
            height: self.height,
            width: self.width,
        }
    }
}
