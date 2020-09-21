use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone)]
pub struct AsciiString(Vec<u8>);

impl AsciiString {
    pub fn unchecked(input: &[u8]) -> Self {
        Self(input.to_vec())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_str(&self) -> &str {
        // OK to unwrap this because the bytes are all ASCII.
        std::str::from_utf8(&self.0).unwrap()
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

impl TryFrom<&str> for AsciiString {
    type Error = AsciiError;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if str.is_ascii() {
            Ok(Self(str.as_bytes().to_vec()))
        } else {
            Err(AsciiError::NotAscii)
        }
    }
}

#[derive(Debug)]
pub enum AsciiError {
    NotAscii,
}
