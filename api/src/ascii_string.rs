use std::convert::TryFrom;
use std::convert::TryInto;

/// A string containing only lowercase ASCII letters.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone)]
pub struct AsciiString(Vec<u8>);

impl AsciiString {
    /// Returns the number of characters in the string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts the ASCII string to a string slice.
    pub fn to_str(&self) -> &str {
        // OK to unwrap this because the bytes are all ASCII.
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl std::fmt::Debug for AsciiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsciiString({:?})", self.to_str())
    }
}

impl AsRef<[u8]> for AsciiString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<AsciiString> for Vec<u8> {
    fn from(val: AsciiString) -> Self {
        val.0
    }
}

impl TryFrom<Vec<u8>> for AsciiString {
    type Error = AsciiError;

    /// Convert a Vec of ASCII letters into an AsciiString. Uppercase
    /// letters are converted to lowercase, but no other changes are made.
    fn try_from(mut bytes: Vec<u8>) -> Result<Self, Self::Error> {
        for b in bytes.iter_mut() {
            if b.is_ascii_uppercase() {
                b.make_ascii_lowercase();
            } else if !b.is_ascii_lowercase() {
                return Err(AsciiError::NotAsciiLetter);
            }
        }
        Ok(Self(bytes))
    }
}

impl TryFrom<&str> for AsciiString {
    type Error = AsciiError;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.as_bytes().to_vec().try_into()
    }
}

#[derive(Debug)]
pub enum AsciiError {
    NotAsciiLetter,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_from_str() {
        assert_eq!(AsciiString::try_from("word").unwrap().to_str(), "word");
        assert_eq!(AsciiString::try_from("WORD").unwrap().to_str(), "word");
        assert!(AsciiString::try_from("y'all").is_err());
        assert!(AsciiString::try_from("common idiom").is_err());
    }
}
