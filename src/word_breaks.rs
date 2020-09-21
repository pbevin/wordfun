use deunicode::deunicode;
use fixedbitset::FixedBitSet;

#[derive(PartialEq, Clone)]
pub struct WordBreaks(FixedBitSet);

impl WordBreaks {
    const SIZE: usize = 60;

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn for_word(word: &str) -> Self {
        let mut breaks = Self::default();
        let mut was_letter = false;
        let mut pos = 0;
        for ch in deunicode(word).chars() {
            if was_letter && !ch.is_ascii_alphabetic() {
                breaks.0.insert(pos);
            }
            if ch.is_ascii_alphabetic() {
                pos += 1;
                was_letter = true;
            } else {
                was_letter = false;
            }
        }
        breaks
    }

    pub fn positions(&self) -> Vec<usize> {
        self.0.ones().collect()
    }

    pub fn from_positions(positions: &[usize]) -> Self {
        let vec: Vec<usize> = positions.into();
        let mut bitset = FixedBitSet::with_capacity(Self::SIZE);
        bitset.extend(vec);
        Self(bitset)
    }

    pub fn is_superset(&self, other: &WordBreaks) -> bool {
        self.0.is_superset(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.count_ones(..) == 0
    }

    pub fn to_vec(&self) -> Vec<usize> {
        self.0.ones().collect()
    }
}

impl Default for WordBreaks {
    fn default() -> Self {
        Self(FixedBitSet::with_capacity(Self::SIZE))
    }
}

impl std::fmt::Debug for WordBreaks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WordBreaks({:?})", &self.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn find_breaks_one_word() {
        assert_eq!(WordBreaks::for_word("test"), Default::default());
    }

    #[test]
    pub fn find_breaks_three_words() {
        assert_eq!(WordBreaks::for_word("café au lait").positions(), vec![4, 6]);
    }
}
