use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::Display;

const THESAURUS_TEXT: &str = include_str!("thesaurus");

type WordId = u32;

#[derive(Default, Debug, PartialEq)]
pub struct Thesaurus {
    entries: Vec<Entry>,
    known_terms: Vec<Word>,
    words_lookup: HashMap<&'static str, WordId>,
}

#[derive(Default, Debug, PartialEq)]
struct Entry {
    word_ids: Vec<WordId>,
}

/// Represents the lengths of the individual words in a thesaurus term.
/// For example, "bacon" has length "5"; "bacon and eggs" has length
/// "11 (5,3,4)".
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordLengths {
    lengths: SmallVec<[u8; 8]>,
}

impl WordLengths {
    fn new(word_lengths: &[u8]) -> Self {
        let total = word_lengths.iter().sum::<u8>();
        let mut lengths = SmallVec::new();
        lengths.push(total);
        if word_lengths.len() > 1 {
            lengths.extend_from_slice(word_lengths);
        }
        Self { lengths }
    }

    pub fn format(&self) -> String {
        if self.lengths.len() == 1 {
            format!("{}", self.lengths[0])
        } else {
            let breakdown = self.lengths[1..]
                .to_vec()
                .iter()
                .map(|len| format!("{}", len))
                .collect::<Vec<_>>()
                .join(",");
            format!("{} ({})", self.lengths[0], &breakdown)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    pub term: &'static str,
    pub word_lengths: WordLengths,
}

impl Word {
    fn new(term: &'static str) -> Self {
        let lengths = term
            .bytes()
            .group_by(|b| b.is_ascii_alphabetic())
            .into_iter()
            .filter_map(|(alpha, bytes)| {
                if alpha {
                    Some(u8::try_from(bytes.count()).unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Self {
            term,
            word_lengths: WordLengths::new(&lengths),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.term)
    }
}

impl Thesaurus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init() -> Self {
        Self::import(THESAURUS_TEXT.lines())
    }

    pub fn import<I>(lines: I) -> Self
    where
        I: Iterator<Item = &'static str>,
    {
        let mut terms = Vec::new();
        let mut term_id_map: HashMap<&'static str, WordId> = HashMap::new();

        let mut entries = Vec::new();

        for line in lines {
            let mut word_ids = Vec::new();
            for term in line.split(',') {
                let id = if let Some(&id) = term_id_map.get(term) {
                    id
                } else {
                    let id = terms.len() as u32;
                    terms.push(term);
                    term_id_map.insert(term, id);
                    id
                };
                word_ids.push(id);
            }

            entries.push(Entry { word_ids });
        }

        let known_terms = terms.into_iter().map(Word::new).collect::<Vec<_>>();
        Thesaurus {
            entries,
            known_terms,
            words_lookup: term_id_map,
        }
    }

    pub fn push(&mut self, synonyms: &[&'static str]) {
        let mut word_ids = Vec::new();
        for term in synonyms {
            let id = if let Some(&id) = self.words_lookup.get(term) {
                id
            } else {
                let id = self.known_terms.len() as u32;
                self.known_terms.push(Word::new(term));
                self.words_lookup.insert(term, id);
                id
            };
            word_ids.push(id);
        }
        self.entries.push(Entry { word_ids });
    }

    pub fn lookup<'a>(&'a self, word: &str) -> impl Iterator<Item = &'a Word> {
        let words = if let Some(word_id) = self.lookup_word(word) {
            self.list_single_entry(word_id)
                .unwrap_or_else(|| self.list_match_headwords(word_id))
        } else {
            HashSet::new()
        };

        words
            .into_iter()
            .map(|id| &self.known_terms[id as usize])
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn list_single_entry(&self, word_id: WordId) -> Option<HashSet<WordId>> {
        self.entries
            .iter()
            .find(|entry| entry.word_ids[0] == word_id)
            .map(|entry| {
                println!("Found entry");
                entry.word_ids.iter().cloned().collect()
            })
    }

    fn list_match_headwords(&self, word_id: WordId) -> HashSet<WordId> {
        println!("Listing all matches");
        self.entries
            .iter()
            .filter(|entry| entry.word_ids.contains(&word_id))
            .map(|entry| entry.word_ids[0])
            .collect()
    }

    fn lookup_word(&self, word: &str) -> Option<WordId> {
        self.words_lookup.get(word).map(Clone::clone)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn pack_word_lengths_single() {
        let lengths = WordLengths::new(&[8]);
        assert_eq!("8", lengths.format());
    }

    #[test]
    pub fn pack_word_lengths_multiple() {
        let lengths = WordLengths::new(&[3, 5]);
        assert_eq!("8 (3,5)", lengths.format());
    }

    fn example_thesaurus() -> Thesaurus {
        let mut t = Thesaurus::new();
        t.push(&["dog", "puppy"]);
        t.push(&["goat", "kid"]);
        t.push(&["human", "kid"]);
        t
    }

    #[test]
    pub fn lookup_headword_found() {
        let t = example_thesaurus();
        let mut matches = t.lookup("dog").map(|t| t.term).collect::<Vec<_>>();
        matches.sort_unstable();
        assert_eq!(matches, vec!["dog", "puppy"]);
    }

    #[test]
    pub fn lookup_headword_not_found() {
        let t = example_thesaurus();
        let mut matches = t.lookup("kid").map(|t| t.term).collect::<Vec<_>>();
        matches.sort_unstable();
        assert_eq!(matches, vec!["goat", "human"]);
    }

    #[test]
    pub fn lookup_word_not_found_anywhere() {
        let t = example_thesaurus();
        let mut matches = t.lookup("kitten");
        assert_eq!(matches.next(), None);
    }
}
