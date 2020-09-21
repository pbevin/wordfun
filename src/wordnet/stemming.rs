use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter;

use super::WordType;

type ExclusionsTable = HashMap<&'static str, Vec<&'static str>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct StemmedWord {
    pub word_type: WordType,
    pub base_form: String,
}

impl StemmedWord {
    fn new<S: Into<String>>(base_form: S, word_type: WordType) -> Self {
        let base_form = base_form.into();
        Self {
            base_form,
            word_type,
        }
    }
}

pub fn stem(s: &str) -> Box<dyn Iterator<Item = StemmedWord> + '_> {
    let mut exc = find_all_exceptions(s).peekable();
    if exc.peek().is_some() {
        Box::new(exc.unique())
    } else {
        Box::new(stem_by_rules(s).unique())
    }
}

type RuleSet = &'static [(&'static str, &'static str)];

static NOUN_RULES: RuleSet = &[
    ("s", ""),
    ("ses", "s"),
    ("xes", "x"),
    ("zes", "z"),
    ("ches", "ch"),
    ("shes", "sh"),
    ("men", "man"),
    ("ies", "y"),
];

static VERB_RULES: RuleSet = &[
    ("s", ""),
    ("ies", "y"),
    ("es", "e"),
    ("es", ""),
    ("ed", "e"),
    ("ed", ""),
    ("ing", "e"),
    ("ing", ""),
];

static ADJ_RULES: RuleSet = &[("er", ""), ("est", ""), ("er", "e"), ("est", "e")];

fn stem_by_rules(input: &str) -> impl Iterator<Item = StemmedWord> + '_ {
    stem_ruleset(&NOUN_RULES, WordType::Noun, input)
        .chain(stem_ruleset(&VERB_RULES, WordType::Verb, input))
        .chain(stem_ruleset(&ADJ_RULES, WordType::Adj, input))
}

fn stem_ruleset<'a>(
    rule_set: &RuleSet,
    word_type: WordType,
    input: &'a str,
) -> impl Iterator<Item = StemmedWord> + 'a {
    rule_set.iter().filter_map(move |(suffix, replacement)| {
        if input.ends_with(suffix) {
            let mut base_form = input[0..input.len() - suffix.len()].to_string();
            base_form.push_str(replacement);
            Some(StemmedWord::new(base_form, word_type))
        } else {
            None
        }
    })
}

fn find_all_exceptions(s: &str) -> impl Iterator<Item = StemmedWord> {
    find_exceptions(&NOUN_EXC, WordType::Noun, s)
        .chain(find_exceptions(&VERB_EXC, WordType::Verb, s))
        .chain(find_exceptions(&ADJ_EXC, WordType::Adj, s))
        .chain(find_exceptions(&ADV_EXC, WordType::Adv, s))
}

fn find_exceptions(
    table: &'static ExclusionsTable,
    word_type: WordType,
    s: &str,
) -> Box<dyn Iterator<Item = StemmedWord>> {
    if let Some(exc) = table.get(s) {
        Box::new(
            exc.iter()
                .map(|base_form| base_form.to_string())
                .map(move |base_form| StemmedWord {
                    base_form,
                    word_type,
                }),
        )
    } else {
        Box::new(iter::empty())
    }
}

lazy_static! {
    pub static ref NOUN_EXC: ExclusionsTable = exc_map(include_str!("data/noun.exc"));
    pub static ref VERB_EXC: ExclusionsTable = exc_map(include_str!("data/verb.exc"));
    pub static ref ADJ_EXC: ExclusionsTable = exc_map(include_str!("data/adj.exc"));
    pub static ref ADV_EXC: ExclusionsTable = exc_map(include_str!("data/adv.exc"));
}

fn exc_map(content: &'static str) -> ExclusionsTable {
    let mut map = ExclusionsTable::default();
    for line in content.lines() {
        let mut words = line.split(' ');
        let inflected = words.next().unwrap();
        if let Some(vec) = map.get_mut(inflected) {
            vec.extend(words);
        } else {
            let vec: Vec<&str> = words.collect();
            map.insert(inflected, vec);
        }
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    fn noun<S: Into<String>>(base_form: S) -> StemmedWord {
        StemmedWord::new(base_form, WordType::Noun)
    }

    fn verb<S: Into<String>>(base_form: S) -> StemmedWord {
        StemmedWord::new(base_form, WordType::Verb)
    }

    fn adj<S: Into<String>>(base_form: S) -> StemmedWord {
        StemmedWord::new(base_form, WordType::Adj)
    }

    fn adv<S: Into<String>>(base_form: S) -> StemmedWord {
        StemmedWord::new(base_form, WordType::Adv)
    }

    #[test]
    pub fn stem_noun_with_exception() {
        assert_eq!(vec![noun("goy")], stem("goyim").collect::<Vec<_>>());
        // epizoon: any external parasitic organism (as fleas)
        assert_eq!(
            vec![noun("epizoan"), noun("epizoon")],
            stem("epizoa").collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn stem_verb_with_exception() {
        assert_eq!(vec![verb("go")], stem("went").collect::<Vec<_>>());

        assert_eq!(
            vec![verb("overflow"), verb("overfly")],
            stem("overflown").collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn stem_adjective_with_exception() {
        assert_eq!(vec![adj("bad")], stem("worse").collect::<Vec<_>>());
    }

    #[test]
    pub fn stem_adverb_with_exception() {
        // "good" is the adjective exception, "well" is the adverb.
        assert_eq!(
            vec![adj("good"), adv("well")],
            stem("best").collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn test_slid() {
        // This shows the importance of returning the word type for each result.  Dictionary lookup
        // fails for "slid", and stemming returns "slide" as a base form -- but the default match
        // for "slide" is "a small flat rectangular piece of glass on which specimens can be
        // mounted for microscopic study".  Adding the fact that we stemmed this as a verb means
        // the dictionary can look up "slide" as a verb.
        assert_eq!(vec![verb("slide")], stem("slid").collect::<Vec<_>>());
    }

    #[test]
    pub fn stem_with_rules() {
        assert_eq!(
            vec![
                noun("sandwiche"),
                noun("sandwich"),
                verb("sandwiche"),
                verb("sandwich")
            ],
            stem("sandwiches").collect::<Vec<_>>()
        );

        assert_eq!(vec![noun("kinsman")], stem("kinsmen").collect::<Vec<_>>());
        assert_eq!(
            vec![verb("change"), verb("chang")],
            stem("changing").collect::<Vec<_>>()
        );
        assert_eq!(
            vec![adj("strang"), adj("strange")],
            stem("strangest").collect::<Vec<_>>()
        );
    }
}
