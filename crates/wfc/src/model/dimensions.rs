use std::num::NonZeroU32;

#[derive(Copy, Clone)]
pub struct Dimensions<const N: usize>([NonZeroU32; N]);

impl<const N: usize> Dimensions<N> {
    pub const fn new(lengths: [u32; N]) -> Option<Self> {
        let mut checked = [const { NonZeroU32::new(1).unwrap() }; N];
        let mut axis = 0;
        while axis < N {
            match NonZeroU32::new(lengths[axis]) {
                Some(length) => checked[axis] = length,
                None => return None,
            }
            axis += 1;
        }
        Some(Self(checked))
    }

    pub fn total(&self) -> usize {
        self.0.iter().map(|length| length.get() as usize).product()
    }

    pub const fn get(&self, index: usize) -> u32 {
        self.0[index].get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn rejects_a_zero_length_axis() {
            assert!(Dimensions::new([0, 3]).is_none());
            assert!(Dimensions::new([3, 0]).is_none());
            assert!(Dimensions::new([3, 3, 0]).is_none());
        }

        #[test]
        fn accepts_every_axis_non_zero() {
            assert!(Dimensions::new([3, 3]).is_some());
        }
    }

    mod total {
        use super::*;

        #[test]
        fn multiplies_every_axis() {
            assert_eq!(Dimensions::new([4, 3]).unwrap().total(), 12);
            assert_eq!(Dimensions::new([4, 3, 2]).unwrap().total(), 24);
        }

        #[test]
        #[cfg(target_pointer_width = "64")]
        fn widens_before_multiplying() {
            let dimensions = Dimensions::new([100_000, 100_000, 100_000]).unwrap();

            assert_eq!(dimensions.total(), 1_000_000_000_000_000);
        }
    }
}
