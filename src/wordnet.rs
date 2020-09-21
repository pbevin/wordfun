mod import;
mod stemming;
mod word_type;

use std::collections::HashMap;

pub use import::from_wordnet;
pub use stemming::{stem, StemmedWord};
pub use word_type::WordType;

/// The type of the object contained in a save file.
pub type DictionaryData = HashMap<String, DefinitionList>;

/// Possible definitions of a word. The first definition in the list is always the best
/// if you don't know the part of speech you're after; otherwise, filter it by the first
/// field.
pub type DefinitionList = Vec<(WordType, String)>;
