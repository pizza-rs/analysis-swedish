<div align="center">

# 🇸🇪 pizza-analysis-swedish

**Swedish text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--swedish-blue)](https://github.com/pizza-rs/analysis-swedish)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Swedish language analysis with Snowball-based stemming and stop words.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `swedish_stem` | Swedish Snowball stemmer |
| TokenFilter | `swedish_stop` | Swedish stop words (114 entries) |
| Analyzer | `swedish` | Full pipeline: lowercase → stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_swedish::register_all(&mut factory);

let analyzer = factory.get_analyzer("swedish").unwrap();
// "flickorna" (the girls) → "flick"
```

## Installation

```toml
[dependencies]
pizza-analysis-swedish = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["swedish"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
