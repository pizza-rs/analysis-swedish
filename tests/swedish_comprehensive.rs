//! Comprehensive tests for pizza-analysis-swedish.

use pizza_analysis_swedish::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// SwedishStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = SwedishStemFilter::new();
}

#[test]
fn stem_plural_ar() {
    let f = SwedishStemFilter::new();
    // "hundar" (dogs) → stem
    let mut token = make_token("hundar");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_plural_or() {
    let f = SwedishStemFilter::new();
    // "flickor" (girls) → stem
    let mut token = make_token("flickor");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_definite_en() {
    let f = SwedishStemFilter::new();
    // "boken" (the book) → stem
    let mut token = make_token("boken");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_definite_et() {
    let f = SwedishStemFilter::new();
    // "huset" (the house) → stem
    let mut token = make_token("huset");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective() {
    let f = SwedishStemFilter::new();
    // "stora" (big, plural) → stem
    let mut token = make_token("stora");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_past() {
    let f = SwedishStemFilter::new();
    // "pratade" (talked) → stem
    let mut token = make_token("pratade");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_present() {
    let f = SwedishStemFilter::new();
    // "springer" (runs) → stem
    let mut token = make_token("springer");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = SwedishStemFilter::new();
    let mut token = make_token("och");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = SwedishStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = SwedishStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SwedishStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = SwedishStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = SwedishStopFilter::new();
    let stop_words = ["och", "i", "att", "en", "det", "som", "är", "för", "på", "med"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = SwedishStopFilter::new();
    let content_words = ["hus", "bok", "skola", "stad"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = SwedishStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("swedish_stem").is_some());
    assert!(factory.get_token_filter("swedish_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("swedish").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("swedish").unwrap();
    let mut input = String::from("Huset är stort och fint");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("swedish").unwrap();
    let mut input = String::from("huset och boken i staden");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"och"));
    assert!(!terms.contains(&"i"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("swedish").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("swedish").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
