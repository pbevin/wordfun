use crate::wordnet::{from_wordnet, stem, DefinitionList, DictionaryData, WordType};
use deunicode::deunicode;

#[derive(Default)]
pub struct Dictionary {
    entries: DictionaryData,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_wordnet() -> Self {
        Self {
            entries: from_wordnet(),
        }
    }

    pub fn insert<S: Into<String>, T: Into<String>>(
        &mut self,
        lemma: S,
        defn: T,
        word_type: WordType,
    ) {
        let lemma = lemma.into();
        if let Some(entry) = self.entries.get_mut(&lemma) {
            entry.push((word_type, defn.into()));
        } else {
            self.entries.insert(lemma, vec![(word_type, defn.into())]);
        }
    }

    /// Returns the number of entries in the dictionary
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a definition of the given term, or `None` if no definition was found.
    ///
    /// If the term contains accented characters, they are converted to a rough ASCII
    /// equivalent (see [`deunicode`]). If a definition is not found, the search
    /// continues with variants suggested by Wordnet's stemmer.
    ///
    /// [`deunicode`]: deunicode::deunicode
    pub fn lookup<'a>(&'a self, term: &str) -> Option<&'a str> {
        let search_term = deunicode(term).to_lowercase().replace(" ", "_");
        self.entries
            .get(&search_term)
            .and_then(Definitions::first)
            .map(AsRef::as_ref)
            .or_else(|| self.lookup_stemming(&search_term))
    }

    fn lookup_stemming(&self, term: &str) -> Option<&str> {
        stem(term)
            .filter_map(|stemmed| {
                self.entries
                    .get(&stemmed.base_form)
                    .and_then(|defs| defs.for_type(stemmed.word_type))
            })
            .map(AsRef::as_ref)
            .next()
    }
}

trait Definitions {
    fn first(&self) -> Option<&str>;
    fn for_type(&self, word_type: WordType) -> Option<&str>;
}

impl Definitions for DefinitionList {
    fn first(&self) -> Option<&str> {
        self.get(0).map(|(_, defn)| defn.as_ref())
    }

    fn for_type(&self, word_type: WordType) -> Option<&str> {
        self.iter()
            .find(|(t, _)| *t == word_type)
            .map(|(_, defn)| defn.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn lookup_simple() {
        let mut dict = Dictionary::new();
        dict.insert("dog", "nice animal", WordType::Noun);
        dict.insert("cat", "horrid animal", WordType::Noun);

        assert_eq!(Some("nice animal"), dict.lookup("dog"));
        assert_eq!(Some("horrid animal"), dict.lookup("cat"));
        assert_eq!(None, dict.lookup("hamster"));
    }

    #[test]
    pub fn lookup_multiple_words() {
        let mut dict = Dictionary::new();
        dict.insert("paul_verlaine", "French poet", WordType::Noun);
        dict.insert("jack-o'-lantern", "Pumpkin carving", WordType::Noun);

        assert_eq!(Some("French poet"), dict.lookup("Paul Verlaine"));
        assert_eq!(Some("French poet"), dict.lookup("paul verlaine"));

        assert_eq!(Some("Pumpkin carving"), dict.lookup("Jack-o'-lantern"));
        assert_eq!(Some("Pumpkin carving"), dict.lookup("jack-o'-lantern"));
    }

    #[test]
    pub fn lookup_accented() {
        let defn = "without the intrusion of a third person";
        let mut dict = Dictionary::new();
        dict.insert("tete-a-tete", defn, WordType::Noun);

        assert_eq!(Some(defn), dict.lookup("tête-à-tête"));
    }
}
