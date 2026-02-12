<!-- PROJECT SHIELDS -->
[![CI][ci-shield]][ci-url]
[![License][license-shield]][license-url]

# pubchemrs2

Async Rust client for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest) with strongly-typed responses and optional Python bindings.

## Features

- **Strongly-typed responses** — 40+ compound property fields mapped to correct Rust types with automatic string-to-f64 coercion.
- **Async HTTP client** — Built on `reqwest` + `tokio` with connection pooling, automatic retry on 429/503/504, and linear backoff.
- **Automatic GET/POST selection** — Searches by InChI, SMILES, SDF, or Formula automatically use POST.
- **Comprehensive API coverage** — Compound, Substance, Assay, Gene, Protein, Pathway, Taxonomy, and Cell domains.
- **Ergonomic convenience API** — `CompoundQuery` and `OtherInputsQuery` builders for common queries.
- **Optional Python bindings** — Enable `pyo3` feature for `#[pyclass]` derives on all major types.

## Quick Start

```toml
[dependencies]
pubchemrs_tokio = { git = "https://github.com/kkiyama117/PubChemrs" }
```

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formula = CompoundQuery::with_name("aspirin")
        .molecular_formula()
        .await?;
    println!("Formula: {formula:?}"); // Some("C9H8O4")

    let props = CompoundQuery::with_cid(2244)
        .properties(&["MolecularWeight", "InChIKey"])
        .await?;
    println!("MW: {:?}", props[0].molecular_weight);  // Some(180.16)

    Ok(())
}
```

## Documentation

| Guide | Description |
|-------|-------------|
| [Getting Started](docs/getting-started.md) | Installation, first request, choosing an API level |
| [Convenience API](docs/convenience-api.md) | CompoundQuery and OtherInputsQuery full guide |
| [Low-Level Client](docs/low-level-client.md) | PubChemClient direct usage, domains, namespaces |
| [Error Handling](docs/error-handling.md) | Error types, retry logic, environment variables |
| [Architecture](docs/architecture.md) | Request pipeline, response types, crate structure |
| [Python Bindings](docs/python-bindings.md) | Build, install, and usage from Python |

## Minimum Supported Rust Version

Both crates use Rust Edition 2024, requiring **Rust 1.85.0** or later.

## Roadmap

- [x] Convenience API (`CompoundQuery`, `OtherInputsQuery`)
- [x] Automatic retry with linear backoff
- [x] Python bindings via PyO3/maturin
- [ ] Typed responses for SourceTable and PeriodicTable endpoints
- [ ] Classification and Standardize endpoint support in `OtherInputsQuery`
- [ ] Response caching layer
- [ ] Publish to crates.io

See the [open issues](https://github.com/kkiyama117/PubChemrs/issues) for more.

## Alternatives

### [pubchem](https://crates.io/crates/pubchem)

Synchronous PubChem client using `ureq` and XML parsing. Simple API like `Compound::with_name("aspirin").title()`.

- **Inspired by:** The ergonomic one-liner API design. `CompoundQuery` was modeled after this crate's `Compound` builder pattern.
- **What pubchemrs2 adds:** Async/await support, automatic retry with backoff, POST method selection for SMILES/InChI/SDF queries, coverage beyond the Compound domain (Substance, Assay, Gene, Protein, etc.), batch queries, and optional Python bindings.

## References

- [PubChem PUG REST Documentation](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest)
- [PubChem Glossary — SMILES](https://pubchem.ncbi.nlm.nih.gov/docs/glossary#section=SMILES)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding guidelines, and the PR workflow.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.

<!-- MARKDOWN LINKS -->
[ci-shield]: https://img.shields.io/github/actions/workflow/status/kkiyama117/PubChemrs/ci.yml?branch=main&style=flat-square&label=CI
[ci-url]: https://github.com/kkiyama117/PubChemrs/actions/workflows/ci.yml
[license-shield]: https://img.shields.io/github/license/kkiyama117/PubChemrs?style=flat-square
[license-url]: https://github.com/kkiyama117/PubChemrs/blob/main/LICENSE
