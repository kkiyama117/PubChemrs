# Getting Started

This guide walks you through installing pubchemrs2 and making your first PubChem API request.

## Prerequisites

- **Rust 1.85.0+** (Edition 2024)
- **tokio** async runtime (pulled in automatically by `pubchemrs_tokio`)

## Installation

Add `pubchemrs_tokio` to your `Cargo.toml` for the full async HTTP client:

```toml
[dependencies]
pubchemrs_tokio = { git = "https://github.com/kkiyama117/PubChemrs" }
```

If you only need type definitions and URL construction (no HTTP dependencies):

```toml
[dependencies]
pubchemrs_struct = { git = "https://github.com/kkiyama117/PubChemrs" }
```

`pubchemrs_tokio` re-exports `pubchemrs_struct` for convenience, so you typically only need one dependency.

## First Request

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formula = CompoundQuery::with_name("aspirin")
        .molecular_formula()
        .await?;
    println!("Formula: {formula:?}"); // Some("C9H8O4")
    Ok(())
}
```

## Choosing an API Level

pubchemrs2 offers two ways to interact with the PubChem API:

### Convenience API (recommended for most users)

`CompoundQuery` and `OtherInputsQuery` provide builder-style one-liners for common queries. They handle client creation, URL construction, and response parsing automatically.

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let weight = CompoundQuery::with_cid(2244)
    .molecular_weight()
    .await?;
# Ok(())
# }
```

Best for: property lookups, synonym searches, batch queries, source listings.

See [Convenience API Guide](convenience-api.md) for full documentation.

### Low-Level Client

`PubChemClient` gives you direct control over domains, namespaces, operations, and output formats. Use it when the convenience API doesn't cover your use case.

```rust,no_run
use pubchemrs_tokio::PubChemClient;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PubChemClient::default();
let props = client.get_properties(
    "aspirin",
    CompoundNamespace::Name(),
    &["MolecularWeight".into(), "InChIKey".into()],
    HashMap::new(),
).await?;
# Ok(())
# }
```

Best for: non-compound domains (Substance, Assay, Gene, etc.), custom output formats, advanced query parameters.

See [Low-Level Client Guide](low-level-client.md) for full documentation.

## Next Steps

- [Convenience API](convenience-api.md) — CompoundQuery and OtherInputsQuery full guide
- [Low-Level Client](low-level-client.md) — PubChemClient direct usage
- [Error Handling](error-handling.md) — Error types, retry logic, environment variables
- [Architecture](architecture.md) — Request pipeline, response types, crate structure
- [Python Bindings](python-bindings.md) — Build and use from Python
