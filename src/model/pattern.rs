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

#[cfg(test)]
mod tests {
    use super::*;

    mod rowwise_mirror {
        use super::*;

        #[test]
        fn reverses_each_row() {
            let test_pattern = Pattern {
                samples: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                height: 3,
                width: 3,
            };
            let result = test_pattern.rowwise_mirror();

            assert_eq!(result.height, test_pattern.height);
            assert_eq!(result.width, test_pattern.width);
            assert_eq!(result.samples, vec![3, 2, 1, 6, 5, 4, 9, 8, 7]);
        }
    }

    mod rotate {
        use super::*;

        #[test]
        fn can_rotate_in_90_deg_increments() {
            let test_pattern = Pattern {
                samples: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                height: 3,
                width: 3,
            };

            let result_90 = test_pattern.rotate(90.0);
            assert_eq!(result_90.samples, vec![7, 4, 1, 8, 5, 2, 9, 6, 3]);

            let result_180 = test_pattern.rotate(180.0);
            assert_eq!(result_180.samples, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);

            let result_270 = test_pattern.rotate(270.0);
            assert_eq!(result_270.samples, vec![3, 6, 9, 2, 5, 8, 1, 4, 7]);

            let result_360 = test_pattern.rotate(360.0);
            assert_eq!(result_360.samples, test_pattern.samples);
        }
    }
}
