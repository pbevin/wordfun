use deunicode::deunicode;
use itertools::Itertools;

use super::{Entry, SearchKey, SortedAscii, WordBreaks};

pub trait Matches {
    fn matches(&self, entry: &Entry) -> bool;
}

pub struct Anagram {
    sorted: SortedAscii,
    num_blanks: usize,
    original: String,
}

impl Anagram {
    pub fn parse(input: &str) -> Self {
        let mut num_blanks = 0;
        let mut letters = Vec::new();
        for &b in deunicode(input).as_bytes() {
            if b == b'.' {
                num_blanks += 1;
            } else if b.is_ascii_alphabetic() {
                letters.push(b.to_ascii_lowercase())
            }
        }
        Self {
            sorted: SortedAscii::from_bytes(&letters),
            num_blanks,
            original: input.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        self.sorted.len() + self.num_blanks
    }
}

impl Matches for Anagram {
    fn matches(&self, entry: &Entry) -> bool {
        if self.len() != entry.len() {
            return false;
        }

        self.sorted.is_subset(entry.sorted())
    }
}

impl From<Anagram> for SearchKey {
    fn from(anagram: Anagram) -> SearchKey {
        SearchKey {
            search_string: anagram.original.clone(),
            len: format!("{}", anagram.len()),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct FindWord {
    display: String,
    pat: Vec<u8>,
    breaks: WordBreaks,
}

impl FindWord {
    pub fn parse(query: &str) -> Self {
        let mut pat = Vec::new();
        let mut positions = Vec::new();
        let mut display = String::new();
        for &b in deunicode(query).as_bytes() {
            if b == b'.' || b.is_ascii_alphabetic() {
                pat.push(b.to_ascii_lowercase());
                display.push(char::from(b.to_ascii_lowercase()));
            } else if b == b'/' {
                positions.push(pat.len());
                display.push(char::from(b));
            }
        }

        Self {
            display,
            pat,
            breaks: WordBreaks::from_positions(&positions),
        }
    }
}

impl Matches for FindWord {
    fn matches(&self, entry: &Entry) -> bool {
        let len = self.pat.len();
        if entry.letters().len() != len {
            return false;
        }
        if !self.breaks.is_empty() && &self.breaks != entry.breaks() {
            return false;
        }

        self.pat
            .iter()
            .zip(entry.letters().as_ref().iter())
            .all(|(&p, &ch)| p == b'.' || p == ch)
    }
}

impl From<FindWord> for SearchKey {
    fn from(find_word: FindWord) -> SearchKey {
        let lengths = std::iter::once(0)
            .chain(find_word.breaks.positions().iter().cloned())
            .chain(std::iter::once(find_word.pat.len()))
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| format!("{}", b - a))
            .collect::<Vec<_>>();

        SearchKey {
            search_string: find_word.display.clone(),
            len: lengths.join(","),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_anagram() {
        let search = Anagram::parse("retain");
        assert_eq!("aeinrt", search.sorted.to_str());
        assert_eq!(0, search.num_blanks);
        assert_eq!("retain", search.original);
    }

    #[test]
    pub fn parse_anagram_blanks() {
        let search = Anagram::parse("r.t.in");
        assert_eq!("inrt", search.sorted.to_str());
        assert_eq!(2, search.num_blanks);
        assert_eq!("r.t.in", search.original);
    }

    #[test]
    pub fn parse_anagram_accents() {
        let search = Anagram::parse("café");
        assert_eq!("acef", search.sorted.to_str());
        assert_eq!(0, search.num_blanks);
        assert_eq!("café", search.original);
    }

    #[test]
    pub fn parse_anagram_case() {
        let search = Anagram::parse("Pioneering tsar");
        assert_eq!("aeegiinnoprrst", search.sorted.to_str());
        assert_eq!(0, search.num_blanks);
        assert_eq!("Pioneering tsar", search.original);
    }

    #[test]
    pub fn parse_pattern_with_letters() {
        assert_eq!(
            FindWord::parse(".a.e"),
            FindWord {
                display: ".a.e".to_string(),
                pat: b".a.e".to_vec(),
                breaks: WordBreaks::default()
            }
        );
    }

    #[test]
    pub fn parse_pattern_dotdotdot() {
        assert_eq!(
            FindWord::parse("..."),
            FindWord {
                display: "...".to_string(),
                pat: b"...".to_vec(),
                breaks: WordBreaks::default()
            }
        );
    }

    #[test]
    pub fn parse_pattern_slashes() {
        assert_eq!(
            FindWord::parse("h.r./...l../e.g"),
            FindWord {
                display: "h.r./...l../e.g".to_string(),
                pat: b"h.r....l..e.g".to_vec(),
                breaks: WordBreaks::from_positions(&[4, 10]),
            }
        );
    }

    #[test]
    pub fn pattern_matches() {
        assert!(FindWord::parse(".a.e").matches(&Entry::from("café")));
    }

    #[test]
    pub fn match_pattern_with_one_break() {
        // A pattern with one break only matches an entry with
        // one break in the same place.
        // s.e./..... should match see reason but not see no evil
        let pattern = FindWord::parse("s.e/......");

        assert!(!pattern.matches(&Entry::from("scenarios")));
        assert!(pattern.matches(&Entry::from("see reason")));
        assert!(!pattern.matches(&Entry::from("see no evil")));
    }

    #[test]
    pub fn match_pattern_with_two_breaks() {
        // A pattern specifying 2 breaks has to match
        // entries with exactly 2 breaks.
        let pattern = FindWord::parse("s.e/../....");

        assert!(!pattern.matches(&Entry::from("scenarios")));
        assert!(!pattern.matches(&Entry::from("see reason")));
        assert!(pattern.matches(&Entry::from("see no evil")));
    }
}
