#[derive(Clone, Debug)]
pub struct SimpleBitSet {
    words: Vec<u64>,
}

impl SimpleBitSet {
    pub fn new(num_bits: usize) -> Self {
        let words_num = SimpleBitSet::calculate_words_from_bits(num_bits);
        SimpleBitSet {
            words: vec![0u64; words_num],
        }
    }

    pub fn full(num_bits: usize) -> Self {
        let words_num = SimpleBitSet::calculate_words_from_bits(num_bits);
        let mut word_vec = Vec::with_capacity(words_num);
        let needs_masking = !num_bits.is_multiple_of(64);
        for i in 0..words_num {
            if needs_masking && i == words_num - 1 {
                word_vec.push((1u64 << (num_bits % 64)) - 1);
            } else {
                word_vec.push(u64::MAX);
            }
        }

        SimpleBitSet { words: word_vec }
    }

    fn calculate_words_from_bits(num_bits: usize) -> usize {
        num_bits.div_ceil(64)
    }

    pub fn set(&mut self, index: usize) {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.words[word_idx] |= 1u64 << bit_idx;
    }

    pub fn clear(&mut self, index: usize) {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.words[word_idx] &= !(1u64 << bit_idx);
    }

    pub fn contains(&self, index: usize) -> bool {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        ((self.words[word_idx] >> bit_idx) & 1u64) == 1
    }

    pub fn intersect_with(&mut self, other: &SimpleBitSet) {
        for i in 0..self.words.len() {
            self.words[i] &= other.words[i];
        }
    }

    pub fn union_with(&mut self, other: &SimpleBitSet) {
        for i in 0..self.words.len() {
            self.words[i] |= other.words[i];
        }
    }

    pub fn count(&self) -> usize {
        let mut sum = 0;
        for word in &self.words {
            sum += word.count_ones();
        }
        sum as usize
    }
}

impl<'a> IntoIterator for &'a SimpleBitSet {
    type Item = usize;
    type IntoIter = SimpleBitSetIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleBitSetIterator {
            all_words: &self.words,
            cur_word: self.words[0],
            word_idx: 0,
        }
    }
}

pub struct SimpleBitSetIterator<'a> {
    all_words: &'a [u64],
    cur_word: u64,
    word_idx: usize,
}
impl<'a> Iterator for SimpleBitSetIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_word == 0 {
            self.word_idx += 1;
            if self.word_idx < self.all_words.len() {
                self.cur_word = self.all_words[self.word_idx]
            } else {
                return None;
            }
        }
        let next_bit_idx = self.cur_word.trailing_zeros();
        self.cur_word &= self.cur_word - 1;
        Some(next_bit_idx as usize + self.word_idx * 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod calculate_words_from_bits {
        use super::*;

        #[test]
        fn returns_correct_word_num_with_no_remainder() {
            let num_bits = 64;
            let result = SimpleBitSet::calculate_words_from_bits(num_bits);

            assert_eq!(result, 1);
        }

        #[test]
        fn returns_correct_word_num_with_remainder() {
            let num_bits_1 = 65;
            let result_1 = SimpleBitSet::calculate_words_from_bits(num_bits_1);

            assert_eq!(result_1, 2);

            let num_bits_2 = 1;
            let result_2 = SimpleBitSet::calculate_words_from_bits(num_bits_2);

            assert_eq!(result_2, 1);
        }
    }

    mod new {
        use super::*;

        #[test]
        fn initializes_new_bit_set_with_empty_words() {
            let num_bits = 128;
            let result = SimpleBitSet::new(num_bits);

            assert_eq!(result.words.len(), 2);
            for word in result.words {
                assert_eq!(word, 0u64);
            }
        }
    }

    mod full {
        use super::*;

        #[test]
        fn initializes_new_bit_set_with_all_words_full() {
            let num_bits = 128;
            let result = SimpleBitSet::full(num_bits);

            assert_eq!(result.words.len(), 2);
            for word in result.words {
                assert_eq!(word, u64::MAX);
            }
        }

        #[test]
        fn initializes_new_bit_set_with_remainder_word() {
            let num_bits = 129;
            let result = SimpleBitSet::full(num_bits);

            assert_eq!(result.words.len(), 3);
            for i in 0..result.words.len() {
                if i == result.words.len() - 1 {
                    assert_eq!(result.words[i], 1);
                } else {
                    assert_eq!(result.words[i], u64::MAX);
                }
            }
        }
    }
}
