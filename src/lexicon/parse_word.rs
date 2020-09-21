use super::{AsciiString, WordBreaks};
use deunicode::deunicode;

pub fn parse_word(word: &str) -> (AsciiString, WordBreaks) {
    // The most common case is that the word is all ASCII with no spaces,
    // and we can make that go faster.
    if word.bytes().all(|b| b.is_ascii_alphabetic()) {
        let letters: Vec<u8> = word.bytes().map(|b| b.to_ascii_lowercase()).collect();
        return (AsciiString::unchecked(&letters), WordBreaks::empty());
    }

    // Otherwise, we have spaces, accents, or both, and this more general algorithm
    // covers them.
    let mut letters = Vec::new();
    let mut positions = Vec::new();
    let mut len = 0;

    for &b in deunicode(word).as_bytes() {
        if b.is_ascii_alphabetic() {
            letters.push(b.to_ascii_lowercase());
            len += 1;
        } else if b.is_ascii_punctuation() || b.is_ascii_whitespace() {
            positions.push(len);
        }
    }

    let letters = AsciiString::unchecked(&letters);
    (letters, WordBreaks::from_positions(&positions))
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
            (ascii_string("simple"), WordBreaks::empty())
        );
    }

    #[test]
    pub fn parse_capitalized_word() {
        assert_eq!(
            parse_word("Kapital"),
            (ascii_string("kapital"), WordBreaks::empty())
        );
    }

    #[test]
    pub fn parse_accented_word() {
        assert_eq!(
            parse_word("abbé"),
            (ascii_string("abbe"), WordBreaks::empty())
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
}
