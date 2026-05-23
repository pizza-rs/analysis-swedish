use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Swedish Snowball-style stemmer. Removes common suffixes for nouns,
/// adjectives, verbs, and adverbs.
#[derive(Clone, Debug, Default)]
pub struct SwedishStemFilter;

impl SwedishStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for SwedishStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 4 {
            return (false, None);
        }

        let stemmed = stem_swedish(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_swedish(word: &str) -> String {
    let suffixes: &[&str] = &[
        // Longest first
        "igheter", "igheten", "ighet",
        "elserna", "elser", "elsen",
        "ingarna", "ningar",
        "ingar", "ingen", "ande", "arna", "erna", "aren",
        "aste", "ades",
        "ning", "ling",
        "ing", "ens", "are", "ast",
        "ade", "arn", "ern",
        "iga", "igt", "isk",
        "het", "dom",
        "ad", "an", "ar", "at", "en", "er", "es", "et",
        "or", "os",
        "a", "e", "s",
    ];

    for suffix in suffixes {
        if word.ends_with(suffix) {
            let stem_len = word.len() - suffix.len();
            if stem_len >= 3 {
                return word[..stem_len].to_string();
            }
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_noun() {
        let filter = SwedishStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("flickorna"),
            start_offset: 0,
            end_offset: 8,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "flick");
    }
}
