use serde::Serialize;
use wordfun::{Popularity, Ranked, SearchKey};

/// Builder for preview responses.
pub struct Preview<'a> {
    max_results: usize,
    wiggle: usize,
    popularity: &'a Popularity,
}

/// Data format of a response to a preview request.  The preview trims the result set down to a
/// manageable size, and moves the most common English words to the front.
#[derive(Serialize)]
pub struct PreviewResponse {
    pub full_count: usize,
    pub query: String,
    pub lengths: String,
    pub words: Vec<PreviewWord>,
}

/// One word in a preview response. The `ranked` field indicates whether or not the word was in the
/// "most common English words" list.
#[derive(Serialize)]
pub struct PreviewWord {
    pub text: String,
    pub ranked: bool,
}

impl<'a> Preview<'a> {
    pub fn new(max_results: usize, wiggle: usize, popularity: &'a Popularity) -> Self {
        Self {
            max_results,
            wiggle,
            popularity,
        }
    }

    pub fn build(&self, key: &SearchKey, words: &[&str]) -> PreviewResponse {
        let mut words: Vec<_> = words.iter().map(|w| self.popularity.to_ranked(w)).collect();
        words.sort();
        let full_count = words.len();
        words.truncate(self.display_count(words.len()));

        let words: Vec<_> = words
            .into_iter()
            .map(|w| match w {
                Ranked::Ranked(text, _) => (text.to_string(), true),
                Ranked::Unranked(text) => (text.to_string(), false),
            })
            .map(|(text, ranked)| PreviewWord { text, ranked })
            .collect();

        PreviewResponse {
            full_count,
            query: key.search_string().to_string(),
            lengths: key.search_len().to_string(),
            words,
        }
    }

    fn display_count(&self, count: usize) -> usize {
        display_count(count, self.max_results, self.wiggle)
    }
}

/// Decide how many preview items to display.
///
/// Normally, we display `count` items up to a maximum of `max`.  But it's annoying when the count
/// is only just above the limit; we see "5 matches (one, two, three, four, ...)".  It should just
/// bend the rules and show me that 5th match!  And that's the wiggle parameter: it controls how
/// many extra results can be crammed into the preview before it snaps back to the maximum.
///
/// Suppose max = 10 and wiggle = 2. Then as count increases, we get:
///   * count = 10 -> 10 results
///   * count = 11 -> 11 results
///   * count = 12 -> 12 results
///   * count = 13 -> 10 results
///   * count = 14 -> 10 results
fn display_count(count: usize, max: usize, wiggle: usize) -> usize {
    if count <= max + wiggle {
        count
    } else {
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wordfun::plural;
    use wordfun::Lexicon;

    // This does roughly what the UI is going to do.
    pub fn format_preview(
        search: &SearchKey,
        results: &[&str],
        max_count: usize,
        wiggle: usize,
    ) -> String {
        let popularity = Popularity::default();
        let preview = Preview::new(max_count, wiggle, &popularity).build(search, results);
        // rats (4): 5 matches (arts, rats, star, tars, tsar)
        let search = format!("{} ({})", preview.query, preview.lengths);

        let how_many = plural(preview.full_count, "match", "matches");

        let mut word_list = preview
            .words
            .iter()
            .map(|pw| {
                if pw.ranked {
                    format!("{}*", pw.text)
                } else {
                    pw.text.to_string()
                }
            })
            .collect::<Vec<_>>();

        if preview.full_count > word_list.len() {
            word_list.push("...".to_string());
        }

        format!("{}: {} ({})", &search, &how_many, &word_list.join(", "))
    }

    #[test]
    pub fn test_format_preview() {
        let lex = Lexicon::new(vec!["abc", "def", "ghij"].into_iter());
        let results = lex.find_word("...");

        assert_eq!(
            "... (3): 2 matches (abc, def)",
            format_preview(&results.key, &results.words, 10, 0)
        )
    }

    #[test]
    pub fn format_preview_two_words() {
        let lex = Lexicon::new(vec!["oneword", "two words", "three whole words"].into_iter());
        let results = lex.find_word(".../W.R..");
        assert_eq!(
            ".../w.r.. (3,5): 1 match (two words)",
            format_preview(&results.key, &results.words, 10, 0)
        )
    }

    #[test]
    pub fn format_preview_max_len_results() {
        let lex = Lexicon::new(vec!["aaa", "aab", "aac"].into_iter());

        let results = lex.find_word("...");
        assert_eq!(
            "... (3): 3 matches (aaa, aab, aac)",
            format_preview(&results.key, &results.words, 3, 0)
        )
    }

    mod preview_wiggle {
        use super::*;

        fn results(search: &str, max: usize, wiggle: usize) -> String {
            let words = vec!["aaa", "aab", "aac", "aad", "aae", "aaf", "aag", "aah"];
            let lex = Lexicon::new(words.into_iter());

            let results = lex.find_word(search);
            format_preview(&results.key, &results.words, max, wiggle)
        }

        #[test]
        pub fn wiggle_no_room() {
            assert_eq!(
                "aa. (3): 8 matches (aaa, aab, aac, aad, aae, ...)",
                results("aa.", 5, 0)
            );
        }

        #[test]
        pub fn wiggle_not_enough_room() {
            assert_eq!(
                "aa. (3): 8 matches (aaa, aab, aac, aad, aae, ...)",
                results("aa.", 5, 2)
            );
        }

        #[test]
        pub fn wiggle_enough_room() {
            assert_eq!(
                "aa. (3): 8 matches (aaa, aab, aac, aad, aae, aaf, aag, aah)",
                results("aa.", 5, 3)
            );
        }

        #[test]
        pub fn wiggle_plenty_of_room() {
            assert_eq!(
                "aa. (3): 8 matches (aaa, aab, aac, aad, aae, aaf, aag, aah)",
                results("aa.", 5, 5)
            );
        }
    }

    mod wiggle_props {
        use super::*;
        use proptest::prelude::*;

        const MAX: usize = 1_000_000;

        fn arb_count() -> impl Strategy<Value = usize> {
            0..MAX
        }

        fn arb_range() -> impl Strategy<Value = (usize, usize)> {
            (0..MAX, 0..MAX)
        }

        proptest! {
            #[test]
            fn count_below_max(count in arb_count(), (max, wiggle) in arb_range()) {
                prop_assume!(count < max);
                let dc = display_count(count, max, wiggle);
                prop_assert_eq!(dc, count);
            }

            #[test]
            fn count_is_max((max, wiggle) in arb_range()) {
                let dc = display_count(max, max, wiggle);
                prop_assert_eq!(dc, max);
            }

            #[test]
            fn count_inside_range(count in arb_count(), (max, wiggle) in arb_range()) {
                prop_assume!(max <= count && count <= max + wiggle);
                let dc = display_count(count, max, wiggle);
                prop_assert_eq!(dc, count);
            }

            #[test]
            fn count_is_max_plus_wiggle((max, wiggle) in arb_range()) {
                let count = max + wiggle;
                let dc = display_count(count, max, wiggle);
                prop_assert_eq!(dc, count);
            }

            #[test]
            fn count_above_max_plus_wiggle(delta in arb_count(), (max, wiggle) in arb_range()) {
                let count = max + wiggle + delta;
                let dc = display_count(count, max, wiggle);
                prop_assert_eq!(dc, max);
            }

            #[test]
            fn never_above_max_plus_wiggle(count in arb_count(), (max, wiggle) in arb_range()) {
                let dc = display_count(count, max, wiggle);
                prop_assert!(dc <= max + wiggle);
            }
        }
    }
}
