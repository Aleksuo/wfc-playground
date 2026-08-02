use crate::model::dimensions::Dimensions;

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
}
