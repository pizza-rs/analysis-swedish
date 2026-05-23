//! Swedish stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Swedish stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "alla",
    "allt",
    "att",
    "av",
    "blev",
    "bli",
    "blir",
    "blivit",
    "de",
    "dem",
    "den",
    "denna",
    "deras",
    "dess",
    "dessa",
    "det",
    "detta",
    "dig",
    "din",
    "dina",
    "ditt",
    "du",
    "där",
    "då",
    "efter",
    "ej",
    "eller",
    "en",
    "er",
    "era",
    "ert",
    "ett",
    "från",
    "för",
    "ha",
    "hade",
    "han",
    "hans",
    "har",
    "henne",
    "hennes",
    "hon",
    "honom",
    "hur",
    "här",
    "i",
    "icke",
    "ingen",
    "inom",
    "inte",
    "jag",
    "ju",
    "kan",
    "kunde",
    "man",
    "med",
    "mellan",
    "men",
    "mig",
    "min",
    "mina",
    "mitt",
    "mot",
    "mycket",
    "ni",
    "nu",
    "när",
    "någon",
    "något",
    "några",
    "och",
    "om",
    "oss",
    "på",
    "samma",
    "sedan",
    "sig",
    "sin",
    "sina",
    "sitt",
    "själv",
    "skulle",
    "som",
    "så",
    "sådan",
    "sådana",
    "sådant",
    "till",
    "under",
    "upp",
    "ut",
    "utan",
    "vad",
    "var",
    "vara",
    "varför",
    "varit",
    "varje",
    "vars",
    "vart",
    "vem",
    "vi",
    "vid",
    "vilka",
    "vilkas",
    "vilken",
    "vilket",
    "vår",
    "våra",
    "vårt",
    "än",
    "är",
    "åt",
    "över",
    ];
    words.iter().copied().collect()
});

/// Removes Swedish stop words from the token stream.
#[derive(Clone, Debug)]
pub struct SwedishStopFilter {
    stop_words: HashSet<String>,
}

impl Default for SwedishStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl SwedishStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for SwedishStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 114);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = SwedishStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = SwedishStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = SwedishStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
