use super::{Sense, WordType};
use WordType::*;

const DATA_NOUN: &str = include_str!("../data/data.noun");
const DATA_VERB: &str = include_str!("../data/data.verb");
const DATA_ADJ: &str = include_str!("../data/data.adj");
const DATA_ADV: &str = include_str!("../data/data.adv");

pub fn lookup(sense: &Sense) -> Option<String> {
    // The sense gives us a word type and a file offset.  We match the word
    // type to one of the DATA_* constants above, then read a line at the
    // given offset.
    let file = match sense.word_type {
        Noun => DATA_NOUN,
        Verb => DATA_VERB,
        Adj => DATA_ADJ,
        Adv => DATA_ADV,
    };

    // The line consists of some metadata, a `|` character, and the
    // definition terminated by \n.
    let line = read_line_at(file, sense.offset as usize)?;
    line.find('|').map(|pos| line[pos + 1..].trim().to_string())
}

fn read_line_at(file: &'static str, offset: usize) -> Option<&'static str> {
    file[offset..].lines().next()
}
