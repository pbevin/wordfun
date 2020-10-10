use super::{Lexicon, Matches, SearchKey};

/// Results returned from Lexicon::anagram, containing the
/// key used for the search, and an iterator that can return
/// matching entries.
pub struct Results<'a> {
    pub words: Vec<&'a str>,
    pub key: SearchKey,
}

impl<'a> Results<'a> {
    pub fn new<M>(pattern: M, lexicon: &'a Lexicon) -> Self
    where
        M: Matches + Into<SearchKey>,
    {
        let words: Vec<_> = lexicon
            .entries()
            .filter(|e| pattern.matches(e))
            .map(|e| e.word())
            .collect();

        let key = pattern.into();

        Self { words, key }
    }
    pub fn search_key_len(&self) -> &str {
        self.key.search_len()
    }

    pub fn search_string(&self) -> &str {
        self.key.search_string()
    }

    pub fn into_iter(self) -> impl Iterator<Item = &'a str> {
        self.words.into_iter()
    }
}
