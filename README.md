# pizza-analysis-swedish

Swedish language analysis with light stemmer and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `swedish_stem` | Token Filter | Swedish light stemmer — removes common noun/adjective/verb suffixes |
| `swedish_stop` | Token Filter | Swedish stop words filter (114 words) |
| `swedish` | Analyzer | Full pipeline: lowercase → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "swedish"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["swedish_stem", "swedish_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
