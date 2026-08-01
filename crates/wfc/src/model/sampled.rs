use crate::model::dimensions::{DimensionValidationError, Dimensions};

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
    pub fn from_fn(
        dimensions: Dimensions<N>,
        mut value_at: impl FnMut(usize) -> T,
    ) -> Result<Self, SamplingError> {
        match dimensions.valid() {
            Ok(_) => (),
            Err(DimensionValidationError::EmptyDimension) => {
                return Err(SamplingError::InvalidDimensions);
            }
        };
        let mut indices: Vec<u32> = vec![0; dimensions.total()];
        let mut sample_palette: Vec<T> = vec![];
        for i in 0..dimensions.total() {
            let sample = value_at(i);
            let k = match sample_palette.iter().position(|c| *c == sample) {
                Some(i) => i,
                None => {
                    sample_palette.push(sample);
                    sample_palette.len() - 1
                }
            };
            indices[i] = k as u32;
        }
        Ok(Sampled {
            sample_palette,
            indices,
            dimensions,
        })
    }
}

#[derive(Debug)]
pub enum SamplingError {
    InvalidDimensions,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_fn {
        use super::*;

        const SAMPLE_VALUES: [u32; 9] = [0, 1, 2, 2, 0, 1, 1, 2, 0];

        fn sample_3x3() -> Sampled<u32, 2> {
            Sampled::from_fn(Dimensions::new([3, 3]), |i| SAMPLE_VALUES[i])
                .expect("3x3 is a valid sample")
        }

        #[test]
        fn interns_values_in_first_seen_order() {
            let sampled = sample_3x3();

            assert_eq!(sampled.palette(), &[0, 1, 2]);
            assert_eq!(sampled.indices(), &SAMPLE_VALUES[..]);
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

        #[test]
        fn rejects_a_zero_length_dimension() {
            let result = Sampled::from_fn(Dimensions::new([0, 3]), |i| SAMPLE_VALUES[i]);

            assert!(matches!(result, Err(SamplingError::InvalidDimensions)));
        }
    }
}
