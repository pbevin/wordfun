use super::{AsciiString, WordBreaks};
use deunicode::deunicode;
use lazy_static::lazy_static;
use std::convert::TryInto;

/// Break a term into its consituent words, with accents and punctuation removed.
///
/// # Examples:
///
/// ```
/// let (word, breaks) = wordfun::parse_word("inhibition");
/// assert_eq!(word.to_str(), "inhibition");
/// assert!(breaks.is_empty());
/// ```
///
/// ```
/// let (word, breaks) = wordfun::parse_word("Bee's knees");
/// assert_eq!(word.to_str(), "beesknees");
/// assert_eq!(breaks.positions().collect::<Vec<_>>(), &[4]);
/// ```
pub fn parse_word(word: &str) -> (AsciiString, WordBreaks) {
    // The most common case is that the word is all ASCII letters with no spaces,
    // and we can make that go faster.
    if let Ok(ascii_string) = word.try_into() {
        return (ascii_string, WordBreaks::default());
    }

    // Otherwise, we have spaces, accents, or both, and this more general algorithm
    // covers them.
    let mut letters = Vec::new();
    let mut positions = Vec::new();
    let mut len = 0;

    for &b in deunicode(word).as_bytes() {
        match char_type(b) {
            CharType::Letter => {
                letters.push(b.to_ascii_lowercase());
                len += 1;
            }
            CharType::Punctuation | CharType::WhiteSpace => {
                positions.push(len);
            }
            CharType::Apostrophe | CharType::Digit | CharType::Unprintable => {
                // Apostrophe doesn't count as a word break, but we do ignore it.
                // Digits and any remaining unprintable characters just get stripped out.
            }
        }
    }

    // This will panic if there are any invalid characters left in the string,
    // which the above loop should have prevented.
    let letters = letters.try_into().unwrap();

    (letters, WordBreaks::from_positions(&positions))
}

/// Rough categorization of u8 values.
#[derive(Copy, Clone, Debug)]
pub enum CharType {
    Letter,
    Apostrophe,
    Digit,
    Punctuation,
    Unprintable,
    WhiteSpace,
}

/// Generate an array mapping `u8` to its [`CharType`].
fn gen_char_types() -> [CharType; 256] {
    #![allow(clippy::needless_range_loop)]

    // Everything below 32 and above 126 is unprintable
    // for our purposes.
    let mut types = [CharType::Unprintable; 256];

    // Most of the remainder is punctuation, aside from letters and digits.
    for i in 33..=126 {
        types[i] = CharType::Punctuation;
    }

    // Special cases:
    types[32] = CharType::WhiteSpace;
    types[b'\'' as usize] = CharType::Apostrophe;

    // Letters and digits
    for i in b'A'..=b'Z' {
        types[i as usize] = CharType::Letter;
    }
    for i in b'a'..=b'z' {
        types[i as usize] = CharType::Letter;
    }
    for i in b'0'..=b'9' {
        types[i as usize] = CharType::Digit;
    }

    types
}

/// Convert a u8 to its [`CharType`].
pub fn char_type(b: u8) -> CharType {
    lazy_static! {
        static ref CHAR_TYPES: [CharType; 256] = gen_char_types();
    }
    CHAR_TYPES[b as usize]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    fn ascii_string(string: &str) -> AsciiString {
        string.try_into().expect("String was not ASCII")
    }

    #[test]
    pub fn parse_simple_word() {
        assert_eq!(
            parse_word("simple"),
            (ascii_string("simple"), WordBreaks::default())
        );
    }

    #[test]
    pub fn parse_capitalized_word() {
        assert_eq!(
            parse_word("Kapital"),
            (ascii_string("kapital"), WordBreaks::default())
        );
    }

    #[test]
    pub fn parse_accented_word() {
        assert_eq!(
            parse_word("abbé"),
            (ascii_string("abbe"), WordBreaks::default())
        );
    }

    #[test]
    pub fn parse_phrase() {
        assert_eq!(
            parse_word("Das Kapital"),
            (ascii_string("daskapital"), WordBreaks::from_positions(&[3]))
        );
    }

    #[test]
    pub fn parse_punctuated_phrase() {
        assert_eq!(
            parse_word("tête-à-tête"),
            (
                ascii_string("teteatete"),
                WordBreaks::from_positions(&[4, 5])
            )
        );
    }

    #[test]
    pub fn parse_apostrophe() {
        assert_eq!(
            parse_word("bee's knees"),
            (ascii_string("beesknees"), WordBreaks::from_positions(&[4]))
        );
    }

    #[test]
    pub fn test_parse_three() {
        assert_eq!(parse_word("THREE").0.to_str(), "three");
    }
}
