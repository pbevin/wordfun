use crate::AsciiString;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use crate::parse_word;

#[derive(Debug)]
pub enum Ranked<T> {
    Ranked(T, u32),
    Unranked(T),
}

impl<T> Ranked<T> {
    pub fn rank(&self) -> Option<u32> {
        match self {
            Self::Ranked(_, rank) => Some(rank.clone()),
            Self::Unranked(_) => None,
        }
    }

    pub fn is_ranked(&self) -> bool {
        match self {
            Self::Ranked(_, _) => true,
            Self::Unranked(_) => false,
        }
    }
}

impl<T> PartialEq for Ranked<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for Ranked<T> {}

impl<T> PartialOrd for Ranked<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Ranked<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Ranked(_, a), Self::Ranked(_, b)) => a.cmp(b),
            (Ranked::Ranked(_, _), Ranked::Unranked(_)) => Ordering::Less,
            (Ranked::Unranked(_), Ranked::Ranked(_, _)) => Ordering::Greater,
            (Self::Unranked(_), Self::Unranked(_)) => Ordering::Equal,
        }
    }
}

impl<'a> AsRef<str> for Ranked<&'a str> {
    fn as_ref(&self) -> &str {
        match self {
            Ranked::Ranked(t, _) => t,
            Ranked::Unranked(t) => t,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Ranked<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ranked::Ranked(t, _) => write!(f, "{}*", t),
            Ranked::Unranked(t) => write!(f, "{}", t),
        }
    }
}

/// Rank words by their popularity. The most popular English words are
/// `you I to the a and that it of me`. After the most popular
/// 1000, the next five are `acting accept blow strange saved`.
pub struct Popularity {
    words: HashMap<AsciiString, u32>,
}

impl Popularity {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    pub fn is_ranked<'a>(&'a self, word: &'a str) -> bool {
        self.words.contains_key(&parse_word(word).0)
    }

    pub fn to_ranked<'a>(&'a self, word: &'a str) -> Ranked<&'a str> {
        match self.words.get(&parse_word(word).0) {
            Some(&rank) => Ranked::Ranked(word, rank),
            None => Ranked::Unranked(word),
        }
    }

    /// Compare strings according to this ranking.
    /// * If both are ranked, nearer the top means smaller.
    /// * If only one is ranked, that's the smaller one.
    /// * If both are unranked, sort them equally.
    pub fn cmp(&self, a: &AsciiString, b: &AsciiString) -> Ordering {
        match (self.words.get(a), self.words.get(b)) {
            (Some(apos), Some(bpos)) => apos.cmp(bpos),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }

    /// Sort strings by this ranking, most popular first. Unranked words
    /// remain in their relative positions at the end of the list.
    pub fn sort(&self, array: &mut [&str]) {
        array.sort_by(|a, b| {
            let akey = parse_word(a).0;
            let bkey = parse_word(b).0;
            self.cmp(&akey, &bkey)
        })
    }
}

impl<'a, I> From<I> for Popularity
where
    I: Iterator<Item = &'a str>,
{
    fn from(iter: I) -> Self {
        let words: HashMap<AsciiString, u32> =
            iter.map(|s| parse_word(s).0).unique().zip(1..).collect();
        Self { words }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    pub fn test_cmp() {
        let ranked = vec!["one", "two", "three", "four", "five"];
        let one = AsciiString::try_from("one").unwrap();
        let two = AsciiString::try_from("two").unwrap();
        let alpha = AsciiString::try_from("alpha").unwrap();
        let beta = AsciiString::try_from("beta").unwrap();

        let popularity = Popularity::from(ranked.into_iter());
        assert_eq!(popularity.cmp(&one, &two), Ordering::Less);
        assert_eq!(popularity.cmp(&two, &one), Ordering::Greater);
        assert_eq!(popularity.cmp(&one, &one), Ordering::Equal);
        assert_eq!(popularity.cmp(&alpha, &two), Ordering::Greater);
        assert_eq!(popularity.cmp(&two, &beta), Ordering::Less);
        assert_eq!(popularity.cmp(&alpha, &beta), Ordering::Equal);
        assert_eq!(popularity.cmp(&alpha, &alpha), Ordering::Equal);
    }

    #[test]
    pub fn test_sort() {
        let ranked = vec!["one", "two", "three", "one hundred"];
        let popularity = Popularity::from(ranked.into_iter());
        let mut list = vec![
            "THREE",
            "One-hundred",
            "four",
            "two",
            "bleep",
            "one",
            "bloop",
        ];
        popularity.sort(&mut list);
        assert_eq!(
            list,
            vec![
                "one",
                "two",
                "THREE",
                "One-hundred",
                "four",
                "bleep",
                "bloop"
            ]
        );
    }

    #[test]
    pub fn test_rank_with_repeats() {
        let ranked = vec!["three", "one", "four", "one", "five"];
        let popularity = Popularity::from(ranked.into_iter());

        assert_eq!(popularity.to_ranked("three"), Ranked::Ranked("three", 1));
        assert_eq!(popularity.to_ranked("one").rank(), Some(2));
        assert_eq!(popularity.to_ranked("four").rank(), Some(3));
        assert_eq!(popularity.to_ranked("five").rank(), Some(4));
        assert_eq!(popularity.to_ranked("nine"), Ranked::Unranked("nine"));
        assert_eq!(popularity.to_ranked("nine").rank(), None);

        assert!(Ranked::Ranked((), 1) < Ranked::Ranked((), 2));

        assert!(popularity.to_ranked("three") < popularity.to_ranked("one"));
        assert!(popularity.to_ranked("one") < popularity.to_ranked("nine"));
        assert!(popularity.to_ranked("three") < popularity.to_ranked("nine"));
        assert!(popularity.to_ranked("xyz") == popularity.to_ranked("zzz"));
    }
}
