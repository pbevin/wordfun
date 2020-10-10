use anyhow::{anyhow, ensure, Context, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use wordfun::{Dictionary, Lexicon, Popularity, Thesaurus};

struct Inner {
    lex: Lexicon,
    thesaurus: Thesaurus,
    dictionary: Dictionary,
    popularity: Popularity,
    assets_dir: PathBuf,
}

#[derive(Clone)]
pub struct Reference {
    inner: Arc<Inner>,
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let p = path.as_ref();
    fs::read_to_string(p).with_context(|| format!("Could not read file {:?}", p))
}

impl Reference {
    pub fn new() -> Result<Self> {
        let assets_dir = env::var_os("ASSETS_DIR").ok_or_else(|| anyhow!("ASSETS_DIR not set"))?;
        ensure!(
            Path::new(&assets_dir).exists(),
            "ASSETS_DIR is set to {:?}, but that directory does not exist.",
            assets_dir
        );
        let lex_text = read_to_string("data/lexicon.txt")?;
        let popular_words = read_to_string("data/popular_words.txt")?;

        let lex = Lexicon::new(lex_text.lines());
        let thesaurus = Thesaurus::init();
        let assets_dir = PathBuf::from(&assets_dir);
        let popularity = Popularity::from(popular_words.lines());
        let dictionary = Dictionary::from_wordnet();

        let inner = Inner {
            lex,
            thesaurus,
            assets_dir,
            popularity,
            dictionary,
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn lexicon(&self) -> &Lexicon {
        &self.inner.lex
    }

    pub fn dictionary(&self) -> &Dictionary {
        &self.inner.dictionary
    }

    pub fn thesaurus(&self) -> &Thesaurus {
        &self.inner.thesaurus
    }

    pub fn assets_dir(&self) -> &Path {
        &self.inner.assets_dir
    }

    pub fn popularity(&self) -> &Popularity {
        &self.inner.popularity
    }
}
