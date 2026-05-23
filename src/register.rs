use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::stem::SwedishStemFilter;
use crate::stop::SwedishStopFilter;

pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("swedish_stem", Box::new(SwedishStemFilter::new()));
    factory.register_token_filter("swedish_stop", Box::new(SwedishStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(SwedishStopFilter::new()),
        Box::new(SwedishStemFilter::new()),
    ];

    factory.register_analyzer(
        "swedish",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("swedish_stem").is_some());
        assert!(factory.get_token_filter("swedish_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("swedish").is_some());
    }
}
