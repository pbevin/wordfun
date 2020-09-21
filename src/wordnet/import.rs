mod data;
mod sense;

use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;

pub use data::lookup;
pub use sense::Sense;

pub use super::{DictionaryData, WordType};

const INDEX_SENSE: &str = include_str!("data/index.sense");

pub fn from_wordnet() -> DictionaryData {
    let entries = INDEX_SENSE
        .lines()
        .map(|line| line.parse::<Sense>())
        .take_while(std::result::Result::is_ok)
        .map(std::result::Result::unwrap)
        .group_by(|sense| sense.lemma.clone());

    let mut map = HashMap::new();
    for (lemma, senses) in entries.into_iter() {
        let senses: SenseList = senses.collect();
        // Sort the list so that the first sense for each word
        // type will be the first found.
        let mut best_definitions = Vec::new();
        if let Some(sense) = senses.find(WordType::Noun) {
            if let Some(defn) = lookup(sense) {
                best_definitions.push((WordType::Noun, defn));
            }
        }
        if let Some(sense) = senses.find(WordType::Verb) {
            if let Some(defn) = lookup(sense) {
                best_definitions.push((WordType::Verb, defn));
            }
        }
        if let Some(sense) = senses.find(WordType::Adj) {
            if let Some(defn) = lookup(sense) {
                best_definitions.push((WordType::Adj, defn));
            }
        }
        if let Some(sense) = senses.find(WordType::Adv) {
            if let Some(defn) = lookup(sense) {
                best_definitions.push((WordType::Adv, defn));
            }
        }
        map.insert(lemma, best_definitions);
    }

    map
}

struct SenseList {
    senses: Vec<Sense>,
}

impl FromIterator<Sense> for SenseList {
    fn from_iter<I: IntoIterator<Item = Sense>>(iter: I) -> Self {
        let mut vec: Vec<Sense> = iter.into_iter().collect();
        vec.sort_unstable_by_key(|sense| sense.sense_num);
        Self { senses: vec }
    }
}

impl SenseList {
    pub fn find(&self, word_type: WordType) -> Option<&Sense> {
        self.senses
            .iter()
            .find(|sense| sense.word_type == word_type)
    }
}
