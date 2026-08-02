use std::num::NonZeroU32;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    /// The extent of the grid of positions at which a `window_dimensions` sized window fits inside
    /// this space, i.e. one more than the difference along each axis.
    ///
    /// Returns `None` if `pattern` is longer than this space on any axis, which is the
    /// one way a caller can ask for a window that does not exist.
    pub fn windows(&self, window_dimensions: Dimensions<N>) -> Option<Self> {
        let mut spans = [0u32; N];
        for (axis, span) in spans.iter_mut().enumerate() {
            *span = self.get(axis).checked_sub(window_dimensions.get(axis))? + 1;
        }
        Self::new(spans)
    }

    /// Converts an index into a coordinate in the N dimensional space.
    ///
    /// ## Panics
    ///
    /// If `index` is not a cell of this space, i.e. `index >= self.total()`.
    pub fn coord_of(&self, index: usize) -> [u32; N] {
        let mut total = self.total();
        assert!(
            index < total,
            "Index {} is out of range for an extent of {} cells",
            index,
            total
        );
        let mut coord = [0u32; N];
        let mut k = index;
        for (i, coord_val) in coord.iter_mut().enumerate().rev() {
            total /= self.get(i) as usize;
            let j = k / total;

            k -= j * total;
            *coord_val = j as u32;
        }
        coord
    }

    /// Converts an N dimensional coordinate into a flattened index.
    ///
    /// ## Panics
    ///
    /// If any component of `coordinate` is outside this space, i.e. not less than the
    /// length of its axis.
    pub fn index_of(&self, coordinate: [u32; N]) -> usize {
        let mut total = self.total();
        let mut index = 0;
        for (i, val) in coordinate.iter().enumerate().rev() {
            assert!(
                *val < self.get(i),
                "Coordinate axis {} value {} is out of range for an axis of length {}",
                i,
                *val,
                self.get(i)
            );
            total /= self.get(i) as usize;
            index += *val as usize * total;
        }
        index
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

    mod windows {
        use super::*;

        #[test]
        fn counts_positions_along_every_axis() {
            let dimensions = Dimensions::new([8, 5]).unwrap();

            let windows = dimensions
                .windows(Dimensions::new([3, 2]).unwrap())
                .unwrap();

            assert_eq!(windows, Dimensions::new([6, 4]).unwrap());
        }

        #[test]
        fn accepts_a_pattern_that_fills_the_space() {
            let dimensions = Dimensions::new([3, 3]).unwrap();

            let windows = dimensions.windows(dimensions).unwrap();

            assert_eq!(windows, Dimensions::new([1, 1]).unwrap());
        }

        #[test]
        fn rejects_a_pattern_longer_on_any_single_axis() {
            let dimensions = Dimensions::new([8, 8]).unwrap();

            assert!(
                dimensions
                    .windows(Dimensions::new([9, 3]).unwrap())
                    .is_none()
            );
            assert!(
                dimensions
                    .windows(Dimensions::new([3, 9]).unwrap())
                    .is_none()
            );
        }

        #[test]
        fn rejects_a_pattern_one_longer_than_the_space() {
            let dimensions = Dimensions::new([3, 3]).unwrap();

            assert!(
                dimensions
                    .windows(Dimensions::new([4, 4]).unwrap())
                    .is_none()
            );
        }

        #[test]
        fn counts_positions_in_three_dimensions() {
            let dimensions = Dimensions::new([4, 3, 2]).unwrap();

            let windows = dimensions
                .windows(Dimensions::new([2, 2, 2]).unwrap())
                .unwrap();

            assert_eq!(windows, Dimensions::new([3, 2, 1]).unwrap());
        }
    }

    mod coord_of {
        use super::*;

        #[test]
        fn handles_minimum_index_correctly() {
            let dimensions = Dimensions::new([100, 100, 100]).unwrap();

            assert_eq!(dimensions.coord_of(0), [0, 0, 0]);
        }

        #[test]
        fn converts_an_index_to_dimension_space_coordinate_correctly() {
            let dimensions = Dimensions::new([100, 100, 100]).unwrap();

            assert_eq!(dimensions.coord_of(101), [1, 1, 0]);
        }

        #[test]
        fn axis_zero_varies_fastest() {
            let dimensions = Dimensions::new([3, 2]).unwrap();

            assert_eq!(dimensions.coord_of(0), [0, 0]);
            assert_eq!(dimensions.coord_of(1), [1, 0]);
            assert_eq!(dimensions.coord_of(2), [2, 0]);
            assert_eq!(dimensions.coord_of(3), [0, 1]);
            assert_eq!(dimensions.coord_of(4), [1, 1]);
            assert_eq!(dimensions.coord_of(5), [2, 1]);
        }

        #[test]
        fn handles_maximum_index_correctly() {
            let dimensions = Dimensions::new([4, 3, 2]).unwrap();

            assert_eq!(dimensions.coord_of(dimensions.total() - 1), [3, 2, 1]);
        }

        #[test]
        #[should_panic(expected = "Index 1000 is out of range for an extent of 1 cells")]
        fn should_panic_on_out_of_bounds_index_with_descriptive_message() {
            let dimensions = Dimensions::new([1, 1, 1]).unwrap();
            dimensions.coord_of(1000);
        }

        #[test]
        #[should_panic(expected = "out of range")]
        fn rejects_an_index_at_the_cell_count() {
            let dimensions = Dimensions::new([3, 2]).unwrap();

            dimensions.coord_of(6);
        }

        #[test]
        fn handles_a_single_cell_extent() {
            let dimensions = Dimensions::new([1, 1, 1]).unwrap();

            assert_eq!(dimensions.coord_of(0), [0, 0, 0]);
        }

        #[test]
        fn handles_length_one_axes() {
            let dimensions = Dimensions::new([4, 1, 3]).unwrap();

            assert_eq!(dimensions.coord_of(0), [0, 0, 0]);
            assert_eq!(dimensions.coord_of(5), [1, 0, 1]);
            assert_eq!(dimensions.coord_of(11), [3, 0, 2]);
        }

        #[test]
        fn handles_one_axis() {
            let dimensions = Dimensions::new([5]).unwrap();

            assert_eq!(dimensions.coord_of(0), [0]);
            assert_eq!(dimensions.coord_of(3), [3]);
        }
    }

    mod index_of {
        use super::*;

        #[test]
        fn matches_the_row_major_formula() {
            let dimensions = Dimensions::new([5, 3]).unwrap();

            for y in 0..3 {
                for x in 0..5 {
                    assert_eq!(dimensions.index_of([x, y]), (x + y * 5) as usize);
                }
            }
        }

        #[test]
        fn inverts_coord_of_at_every_index() {
            let dimensions = Dimensions::new([4, 3, 2]).unwrap();

            for index in 0..dimensions.total() {
                assert_eq!(dimensions.index_of(dimensions.coord_of(index)), index);
            }
        }

        #[test]
        fn handles_length_one_axes() {
            let dimensions = Dimensions::new([4, 1, 3]).unwrap();

            assert_eq!(dimensions.index_of([0, 0, 0]), 0);
            assert_eq!(dimensions.index_of([1, 0, 1]), 5);
            assert_eq!(dimensions.index_of([3, 0, 2]), 11);
        }

        #[test]
        #[should_panic(
            expected = "Coordinate axis 1 value 5 is out of range for an axis of length 2"
        )]
        fn handles_out_of_bounds_coordinate_axes_with_descriptive_message() {
            let dimensions = Dimensions::new([3, 2]).unwrap();
            dimensions.index_of([1, 5]);
        }

        #[test]
        fn accepts_the_last_valid_coordinate() {
            let dimensions = Dimensions::new([3, 2]).unwrap();

            assert_eq!(dimensions.index_of([2, 1]), 5);
        }

        #[test]
        #[should_panic(expected = "out of range")]
        fn rejects_a_coordinate_at_the_axis_length() {
            let dimensions = Dimensions::new([3, 2]).unwrap();

            dimensions.index_of([3, 0]);
        }
    }
}
