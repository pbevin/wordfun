//! This library performs word search, anagramming, definition lookup, and thesaurus search.
//!
//! The central data structure is the [Lexicon], which holds a list of words and phrase that can be
//! searched.  Each word is stored as an [Entry], which caches various information about the word
//! or phrase (such as the list of letters it contains in alphabetical order, and the positions of
//! word breaks) so that linear search over the lexicon is as fast as possible.
//!
//! [Lexicon]: ./struct.Lexicon.html

mod ascii_string;
mod dictionary;
mod lexicon;
mod plural;
mod popularity;
mod sorted_ascii;
mod thesaurus;
mod word_breaks;
mod wordnet;

pub use ascii_string::AsciiString;
pub use dictionary::Dictionary;
pub use lexicon::{parse_word, Entry, Lexicon, Results, SearchKey};
pub use plural::plural;
pub use popularity::{Popularity, Ranked};
pub use sorted_ascii::SortedAscii;
pub use thesaurus::Thesaurus;
pub use word_breaks::WordBreaks;
