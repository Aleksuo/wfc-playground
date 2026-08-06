use crate::{RuleModel, Solution, model::dimensions::Dimensions};

pub struct Sampled<T, const N: usize> {
    sample_palette: Vec<T>,
    indices: Vec<u32>,
    dimensions: Dimensions<N>,
}

impl<T, const N: usize> Sampled<T, N> {
    pub fn dimensions(&self) -> &Dimensions<N> {
        &self.dimensions
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn palette(&self) -> &[T] {
        &self.sample_palette
    }

    pub fn value_at(&self, coord: [u32; N]) -> &T {
        &self.sample_palette[self.indices[self.dimensions.index_of(coord)] as usize]
    }
}

impl<T: Eq, const N: usize> Sampled<T, N> {
    pub fn from_fn(dimensions: Dimensions<N>, mut value_at: impl FnMut([u32; N]) -> T) -> Self {
        let total = dimensions.total();
        let mut indices: Vec<u32> = Vec::with_capacity(total);
        let mut sample_palette: Vec<T> = vec![];
        for index in 0..total {
            let sample = value_at(dimensions.coord_of(index));
            let palette_index = match sample_palette.iter().position(|value| *value == sample) {
                Some(existing) => existing,
                None => {
                    sample_palette.push(sample);
                    sample_palette.len() - 1
                }
            };
            indices.push(palette_index as u32);
        }
        Sampled {
            sample_palette,
            indices,
            dimensions,
        }
    }
}

impl<T: Clone> Sampled<T, 2> {
    pub fn decode(&self, solution: &Solution, rule_model: &RuleModel) -> Sampled<T, 2> {
        let grid = solution.output_dimensions;
        let RuleModel {
            patterns,
            pattern_dimensions,
            adjadency_rules: _,
            frequency_hints: _,
            num_directions: _,
        } = rule_model;
        let [grid_width, grid_height] = grid.get();
        let [pattern_width, pattern_height] = pattern_dimensions.get();

        let output_dimensions = Dimensions::new([
            grid_width + pattern_width - 1,
            grid_height + pattern_height - 1,
        ])
        .unwrap();
        let mut indices = vec![0u32; output_dimensions.total()];
        for gy in 0..grid_height {
            for gx in 0..grid_width {
                let grid_idx = grid.index_of([gx, gy]);
                let pattern = &patterns[solution.output[grid_idx] as usize];
                for py in 0..pattern_height {
                    for px in 0..pattern_width {
                        let output_x = gx + px;
                        let output_y = gy + py;
                        let sample = pattern.samples[pattern.dimensions.index_of([px, py])];
                        indices[output_dimensions.index_of([output_x, output_y])] = sample;
                    }
                }
            }
        }
        Sampled {
            sample_palette: self.sample_palette.clone(),
            indices,
            dimensions: output_dimensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_fn {
        use super::*;

        const SAMPLE_VALUES: [u32; 9] = [0, 1, 2, 2, 0, 1, 1, 2, 0];

        fn sample_3x3() -> Sampled<u32, 2> {
            let dimensions = Dimensions::new([3, 3]).expect("3x3 is non-empty");
            Sampled::from_fn(dimensions, |coord| {
                SAMPLE_VALUES[dimensions.index_of(coord)]
            })
        }

        #[test]
        fn interns_values_in_first_seen_order() {
            let sampled = sample_3x3();

            assert_eq!(sampled.palette(), &[0, 1, 2]);
            assert_eq!(sampled.indices(), &SAMPLE_VALUES[..]);
        }

        #[test]
        fn visits_coordinates_in_index_order() {
            let dimensions = Dimensions::new([3, 2]).expect("3x2 is non-empty");
            let mut visited = Vec::new();

            Sampled::from_fn(dimensions, |coord| {
                visited.push(coord);
                0u32
            });

            assert_eq!(visited, [[0, 0], [1, 0], [2, 0], [0, 1], [1, 1], [2, 1]]);
        }

        #[test]
        fn is_deterministic() {
            let first = sample_3x3();

            for _ in 0..5 {
                let repeat = sample_3x3();

                assert_eq!(first.palette(), repeat.palette());
                assert_eq!(first.indices(), repeat.indices());
            }
        }
    }

    mod decode {
        use super::*;
        use crate::model::{pattern::Pattern, rule_model::FrequencyHints};

        fn input() -> Sampled<u32, 2> {
            const VALUES: [u32; 4] = [10, 20, 30, 40];
            let dimensions = Dimensions::new([2, 2]).expect("2x2 is non-empty");
            Sampled::from_fn(dimensions, |coord| VALUES[dimensions.index_of(coord)])
        }

        fn model_with(samples: Vec<u32>) -> RuleModel {
            let pattern_dimensions = Dimensions::new([2, 2]).expect("2x2 is non-empty");
            RuleModel {
                patterns: vec![Pattern {
                    samples,
                    dimensions: pattern_dimensions,
                }],
                adjadency_rules: vec![],
                frequency_hints: FrequencyHints::new(vec![1]),
                num_directions: 4,
                pattern_dimensions,
            }
        }

        #[test]
        fn a_single_placement_reproduces_the_pattern() {
            let solution = Solution {
                output: vec![0],
                output_dimensions: Dimensions::new([1, 1]).expect("1x1 is non-empty"),
            };
            let model = model_with(vec![3, 1, 0, 2]);

            let decoded = input().decode(&solution, &model);

            assert_eq!(decoded.indices(), &[3, 1, 0, 2]);
        }

        #[test]
        fn output_extent_is_the_grid_grown_by_the_pattern_overlap() {
            let solution = Solution {
                output: vec![0; 6],
                output_dimensions: Dimensions::new([3, 2]).expect("3x2 is non-empty"),
            };
            let model = model_with(vec![0, 1, 2, 3]);

            let decoded = input().decode(&solution, &model);

            assert_eq!(decoded.dimensions(), &Dimensions::new([4, 3]).unwrap());
            assert_eq!(decoded.indices().len(), 12);
        }

        #[test]
        fn carries_the_input_palette_forward_unchanged() {
            let solution = Solution {
                output: vec![0],
                output_dimensions: Dimensions::new([1, 1]).expect("1x1 is non-empty"),
            };
            let model = model_with(vec![0, 1, 2, 3]);
            let source = input();

            let decoded = source.decode(&solution, &model);

            assert_eq!(decoded.palette(), source.palette());
            assert_eq!(*decoded.value_at([1, 1]), 40);
        }
    }
}
