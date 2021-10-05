use std::str::FromStr;

use super::WordType;

#[derive(Debug)]
pub struct ParseError {
    line: String,
    message: String,
}

fn parse_error(line: &str, message: &str) -> ParseError {
    ParseError {
        line: line.to_string(),
        message: message.to_string(),
    }
}

pub struct Sense {
    pub lemma: String,
    pub word_type: WordType,
    pub offset: u64,
    pub sense_num: u32,
}

impl FromStr for Sense {
    type Err = ParseError;

    // dog%1:05:00:: 02086723 1 42
    // ^^^ ^         ^^^^^^^^ ^
    //  |  word type offset   sense_num
    // lemma
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split(' ');
        let sense_key = tokens.next().unwrap();
        let offset = tokens
            .next()
            .ok_or_else(|| parse_error(line, "Synset Offset not found"))?
            .parse::<u64>()
            .map_err(|_| parse_error(line, "Unparseable Synset Offset"))?;
        let sense_num = tokens
            .next()
            .ok_or_else(|| parse_error(line, "Sense Number not found"))?
            .parse::<u32>()
            .map_err(|_| parse_error(line, "Unparseable Sense Number"))?;

        let pct_pos = sense_key
            .find('%')
            .ok_or_else(|| parse_error(line, "lemma % delimiter not found"))?;
        let lemma = line[..pct_pos].to_string();
        if pct_pos >= line.len() - 1 {
            return Err(parse_error(line, "Word type not found"));
        }
        let word_type = parse_word_type(&line[pct_pos + 1..][0..1])
            .ok_or_else(|| parse_error(line, "Unknown word_type"))?;

        Ok(Self {
            lemma,
            word_type,
            offset,
            sense_num,
        })
    }
}

fn parse_word_type(n: &str) -> Option<WordType> {
    match n {
        "1" => Some(WordType::Noun),
        "2" => Some(WordType::Verb),
        "3" => Some(WordType::Adj),
        "4" => Some(WordType::Adv),
        "5" => Some(WordType::Adj),
        _ => None,
    }
}
