#[derive(Debug, PartialEq, Default, Clone)]
pub struct SortedAscii(Vec<u8>);

impl SortedAscii {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut vec = bytes.to_vec();
        vec.sort();
        Self(vec)
    }

    pub fn to_str(&self) -> &str {
        // OK to unwrap this because the bytes are all ASCII.
        std::str::from_utf8(&self.0).unwrap()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_subset(&self, other: &SortedAscii) -> bool {
        let mut i = 0;
        let mut j = 0;
        loop {
            if i == self.len() {
                // Reached the end of self - we are a subset!
                return true;
            }
            if j == other.len() {
                // Exhausted other while self still has elements
                return false;
            }
            let a = self.0[i];
            let b = other.0[j];

            if a == b {
                // Match - skip both
                i += 1;
                j += 1;
            } else if a > b {
                j += 1;
            } else {
                return false;
            }
        }
    }
}

impl AsRef<[u8]> for SortedAscii {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
