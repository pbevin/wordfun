use crate::{AsciiString, SortedAscii, WordBreaks};

/// A word or phrase stored in the Lexicon.
///
/// As well as the word or phrase itself, the Entry also contains:
///   * The letters in their original order, stripped of accents and converted to lower case
///   * The letters in alphabetical order -- this makes anagram search go faster
///   * The letter positions of word breaks.
///
/// # Examples
///
/// ```
/// use wordfun::Entry;
///
/// let entry = Entry::from("Île-de-France");
/// assert_eq!("Île-de-France", entry.word());
/// assert_eq!("iledefrance", entry.letters().to_str());
/// assert_eq!("acdeeefilnr", entry.sorted().to_str());
/// assert_eq!(vec![3, 5], entry.breaks().to_vec());
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    head_word: String,
    letters: AsciiString,
    sorted: SortedAscii,
    breaks: WordBreaks,
}

impl Entry {
    /// The original word or phrase, with capitalization, spacing, and so on intact
    pub fn word(&self) -> &str {
        &self.head_word
    }

    /// The letters in their original order, stripped of accents and converted to lower case
    pub fn letters(&self) -> &AsciiString {
        &self.letters
    }

    /// The letters in alphabetical order, including repeats.
    pub fn sorted(&self) -> &SortedAscii {
        &self.sorted
    }

    /// The number of letters. This is simply the length of `letters()`.
    pub fn len(&self) -> usize {
        self.letters.len()
    }

    /// The positions of the word breaks. The positions refer to indexes into `letters()`
    /// starting from 0. A phrase with n distinct words has n-1 word breaks.
    pub fn breaks(&self) -> &WordBreaks {
        &self.breaks
    }
}

impl From<&str> for Entry {
    fn from(input: &str) -> Self {
        let head_word = input.trim().to_string();
        let (letters, breaks) = super::parse_word(&head_word);
        let sorted = SortedAscii::from_bytes(letters.as_ref());

        Self {
            head_word,
            letters,
            sorted,
            breaks,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn trimmed_word_stays_the_same(word: String) {
            let word = word.trim();
            let entry = Entry::from(word);
            prop_assert_eq!(word, entry.word());
        }

        #[test]
        fn letters_only_has_lowercase_ascii(word: String) {
            let entry = Entry::from(word.as_ref());
            let letters = entry.letters();
            let a = letters.as_ref().iter().all(|b| b.is_ascii_alphabetic());
            prop_assert!(a);
        }

        #[test]
        fn letters_is_idempotent(word: String) {
            let entry1 = Entry::from(word.as_ref());
            let entry2 = Entry::from(entry1.letters().to_str());
            prop_assert_eq!(entry1.letters(), entry2.letters());
        }
    }
}
