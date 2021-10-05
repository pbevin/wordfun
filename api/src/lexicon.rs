mod entry;
mod parse_word;
mod results;
mod search;
mod search_key;

use std::slice;

pub use entry::Entry;
pub use parse_word::parse_word;
pub use results::Results;
pub use search::{Anagram, FindWord, Matches};
pub use search_key::SearchKey;

use crate::{AsciiString, SortedAscii, WordBreaks};

/// The searchable list of words and phrases.
///
/// The Lexicon holds a list of [entries], which are pre-processed to help searching.
///
/// [entries]: [`Entry`]
pub struct Lexicon {
    entries: Vec<Entry>,
}

impl Lexicon {
    /// Builds a Lexicon from an iterator of string slices.
    pub fn new<'a, I>(words: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        Self {
            entries: words.map(Entry::from).collect(),
        }
    }

    /// Returns an iterator over all entries.
    /// The iterator element type is `&Entry`.
    pub fn entries(&self) -> Entries {
        Entries {
            it: self.entries.iter(),
        }
    }

    /// Search the lexicon for entries matching an anagram query.
    pub fn anagram<'a>(&'a self, query: &str) -> Results<'a> {
        let pat = Anagram::parse(query);
        Results::new(pat, self)
    }

    /// Search the lexicon for entries matching a Find Word query.
    pub fn find_word<'a>(&'a self, query: &'a str) -> Results<'a> {
        let pat = FindWord::parse(query);
        Results::new(pat, self)
    }
}

/// An iterator over the [`Entry`] values in a lexicon.
///
/// This struct is created by the [`entries`] method on [`Lexicon`].
/// See its documentation for more.
pub struct Entries<'a> {
    it: slice::Iter<'a, Entry>,
}

impl<'a> Iterator for Entries<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn anagram_search() {
        let lex = Lexicon::new(vec!["ace", "café", "efface", "face", "fact", "fake"].into_iter());
        let results = lex.anagram("acef");
        assert_eq!("4", results.search_key_len());
        assert_eq!("acef", results.search_string());
        assert_eq!(
            vec!["café", "face"],
            results.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn anagram_search_missing_letters() {
        let lex = Lexicon::new(vec!["ace", "café", "efface", "face", "fact", "fake"].into_iter());
        let results = lex.anagram("ac..");
        assert_eq!("4", results.search_key_len());
        assert_eq!("ac..", results.search_string());
        assert_eq!(
            vec!["café", "face", "fact"],
            results.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn find_word() {
        let lex = Lexicon::new(vec!["ace", "café", "efface", "face", "fact", "fake"].into_iter());
        let results = lex.find_word(".a.e");
        assert_eq!("4", results.search_key_len());
        assert_eq!(".a.e", results.search_string());
        assert_eq!(
            vec!["café", "face", "fake"],
            results.into_iter().collect::<Vec<_>>()
        );
    }
}
