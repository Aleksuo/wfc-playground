#[derive(Copy, Clone)]
pub struct Dimensions<const N: usize>([u32; N]);

impl<const N: usize> Dimensions<N> {
    pub fn valid(&self) -> Result<(), DimensionValidationError> {
        for i in self.0 {
            if i == 0 {
                return Err(DimensionValidationError::EmptyDimension);
            }
        }
        Ok(())
    }

    pub fn total(&self) -> usize {
        self.0.iter().map(|length| *length as usize).product()
    }

    pub fn get(&self, index: usize) -> u32 {
        self.0[index]
    }

    pub fn new(dims: [u32; N]) -> Self {
        Self(dims)
    }
}

pub enum DimensionValidationError {
    EmptyDimension,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod total {
        use super::*;

        #[test]
        fn multiplies_every_axis() {
            assert_eq!(Dimensions::new([4, 3]).total(), 12);
            assert_eq!(Dimensions::new([4, 3, 2]).total(), 24);
        }

        #[test]
        #[cfg(target_pointer_width = "64")]
        fn widens_before_multiplying() {
            let dimensions = Dimensions::new([100_000, 100_000, 100_000]);

            assert_eq!(dimensions.total(), 1_000_000_000_000_000);
        }
    }
}
