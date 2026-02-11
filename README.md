# pubchemrs2

Async Rust client for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest) with strongly-typed responses and optional Python bindings.

## Features

- **Strongly-typed responses** — 40+ compound property fields mapped to correct Rust types (`f64`, `u32`, `Option<T>`). Numeric fields returned as JSON strings (`MolecularWeight`, `ExactMass`, `MonoisotopicMass`) are automatically parsed to `f64`.
- **Async HTTP client** — Built on `reqwest` + `tokio` with connection pooling, automatic retry on 429/503/504, and linear backoff.
- **Automatic GET/POST selection** — Searches by InChI, SMILES, SDF, or Formula automatically use POST as required by the PubChem API.
- **Comprehensive API coverage** — Compound, Substance, Assay, Gene, Protein, Pathway, Taxonomy, and Cell domains. Structure search (substructure/superstructure/similarity/identity) and fast search (2D/3D similarity).
- **Ergonomic convenience API** — `CompoundQuery` and `OtherInputsQuery` builders for common queries with one-liner property accessors.
- **Optional Python bindings** — Enable `pyo3` feature flag for `#[pyclass]` derives on all major types. CI builds maturin wheels for Linux and Windows.

## Quick Start

### Installation

```toml
[dependencies]
pubchemrs_tokio = { git = "https://github.com/kkiyama117/PubChemrs" }
```

For type definitions only (no HTTP dependencies):

```toml
[dependencies]
pubchemrs_struct = { git = "https://github.com/kkiyama117/PubChemrs" }
```

### Convenience API

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Single property
    let formula = CompoundQuery::with_name("aspirin")
        .molecular_formula()
        .await?;
    println!("Formula: {formula:?}"); // Some("C9H8O4")

    // Multiple properties in one request
    let props = CompoundQuery::with_cid(2244)
        .properties(&["MolecularWeight", "InChIKey"])
        .await?;
    println!("MW: {:?}", props[0].molecular_weight);  // Some(180.16)
    println!("InChIKey: {:?}", props[0].inchikey);     // Some("BSYNRYMUTXBXSQ-...")

    // Synonyms
    let synonyms = CompoundQuery::with_name("caffeine")
        .synonyms()
        .await?;
    println!("Found {} synonyms", synonyms.len());

    Ok(())
}
```

### Low-Level Client

For full control over the request, use `PubChemClient` directly:

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

## API Examples

### CompoundQuery

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// Search by name
let formula = CompoundQuery::with_name("aspirin")
    .molecular_formula().await?;

// Search by CID
let weight = CompoundQuery::with_cid(2244)
    .molecular_weight().await?;

// Search by SMILES (automatically uses POST)
let props = CompoundQuery::with_smiles("CC(=O)OC1=CC=CC=C1C(=O)O")
    .properties(&["MolecularFormula", "InChIKey"]).await?;

// Search by InChIKey
let name = CompoundQuery::with_inchikey("BSYNRYMUTXBXSQ-UHFFFAOYSA-N")
    .iupac_name().await?;

// Batch query — multiple CIDs in one request
let batch = CompoundQuery::with_cids(&[2244, 962, 5793])
    .properties(&["MolecularFormula", "MolecularWeight"]).await?;

// Full compound record
let compound = CompoundQuery::with_cid(2244).record().await?;

// Synonyms
let synonyms = CompoundQuery::with_name("caffeine").synonyms().await?;

// Discover CID from name
let cid = CompoundQuery::with_name("aspirin").cid().await?;
# Ok(())
# }
```

Available single-property accessors: `molecular_formula`, `molecular_weight`, `iupac_name`, `inchi`, `inchikey`, `smiles`, `canonical_smiles`, `xlogp`, `exact_mass`, `tpsa`, `charge`.

### OtherInputsQuery

```rust,no_run
use pubchemrs_tokio::OtherInputsQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// List all substance depositors
let sources = OtherInputsQuery::substance_sources().fetch().await?;

// List all assay depositors
let sources = OtherInputsQuery::assay_sources().fetch().await?;

// Periodic table data (raw JSON)
let table = OtherInputsQuery::periodic_table().fetch_json().await?;
# Ok(())
# }
```

### Custom Client

Both query builders accept a custom client via `using_client()`:

```rust,no_run
use pubchemrs_tokio::{CompoundQuery, PubChemClient, ClientConfig};
use std::time::Duration;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PubChemClient::new(ClientConfig {
    timeout: Duration::from_secs(60),
    max_retries: 5,
    retry_delay: Duration::from_secs(1),
})?;

let formula = CompoundQuery::with_name("aspirin")
    .using_client(&client)
    .molecular_formula()
    .await?;
# Ok(())
# }
```

## Client Configuration

| Option | Default | Description |
|--------|---------|-------------|
| `timeout` | 30s | HTTP request timeout |
| `max_retries` | 3 | Retry count on retryable errors |
| `retry_delay` | 500ms | Base delay between retries (linear backoff) |

Retryable status codes: `429 Too Many Requests`, `503 Service Unavailable`, `504 Gateway Timeout`.

## Supported Domains

| Domain | Enum |
|--------|------|
| Compound | `Domain::Compound()` |
| Substance | `Domain::Substance()` |
| Assay | `Domain::Assay()` |
| Gene | `Domain::Gene()` |
| Protein | `Domain::Protein()` |
| Pathway | `Domain::PathWay()` |
| Taxonomy | `Domain::Taxonomy()` |
| Cell | `Domain::Cell()` |

## Compound Namespaces

| Namespace | Enum | HTTP Method |
|-----------|------|-------------|
| CID | `CompoundNamespace::Cid()` | GET |
| Name | `CompoundNamespace::Name()` | GET |
| SMILES | `CompoundNamespace::Smiles()` | POST |
| InChI | `CompoundNamespace::InChI()` | POST |
| SDF | `CompoundNamespace::Sdf()` | POST |
| InChIKey | `CompoundNamespace::InchiKey()` | GET |
| Formula | `CompoundNamespace::Formula()` | POST |
| Structure Search | `CompoundNamespace::StructureSearch(...)` | POST |
| Fast Search | `CompoundNamespace::FastSearch(...)` | POST |

## Error Handling

`pubchemrs_tokio::error::Error` variants:

| Variant | Description |
|---------|-------------|
| `Http` | Network / connection errors (`reqwest::Error`) |
| `HttpStatus` | Non-success HTTP status with response body |
| `ApiFault` | PubChem API fault response (code + message) |
| `Json` | JSON deserialization error |
| `PubChem` | Input validation errors from `pubchemrs_struct` |

### Environment Variables

| Variable | Effect |
|----------|--------|
| `PUBCHEM_PANIC_ON_ERR=1` | Panic on `pubchemrs_struct` errors instead of returning `Err` |
| `PUBCHEM_BACKTRACE_IN_ERR=1` | Include backtrace in error messages |

## Crate Architecture

```
pubchemrs2/
  pubchemrs_struct/   Type definitions only (serde, no HTTP)
  pubchemrs_tokio/    Async HTTP client (reqwest + tokio)
```

`pubchemrs_tokio` re-exports `pubchemrs_struct` for convenience.

URL construction follows the PUG REST pattern:

```
https://pubchem.ncbi.nlm.nih.gov/rest/pug/{domain}/{namespace}/{identifiers}/{operation}/{output}
```

Each segment is modeled as a Rust type:

- `Domain` + `Namespace` + `Identifiers` → `InputSpecification`
- `Operation` — Record, Property, Synonyms, XRefs, etc.
- `OutputFormat` — JSON, SDF, etc.
- `UrlBuilder` assembles all parts and determines GET vs POST.

## SMILES Field Naming

PubChem renamed its SMILES fields. `CompoundProperties` includes both current and legacy fields for compatibility:

| Current Name | JSON Key | Rust Field | Content |
|-------------|----------|------------|---------|
| SMILES | `SMILES` | `smiles` | Full SMILES with stereochemistry and isotopes |
| Connectivity SMILES | `ConnectivitySMILES` | `connectivity_smiles` | Connectivity only |
| *(legacy)* Canonical SMILES | `CanonicalSMILES` | `canonical_smiles` | Same as Connectivity SMILES |
| *(legacy)* Isomeric SMILES | `IsomericSMILES` | `isomeric_smiles` | Same as SMILES |

Prefer `smiles` and `connectivity_smiles` for new code.

## Python Bindings

Enable the `pyo3` feature to derive `#[pyclass]` on all major types:

```bash
# Build and install into current Python environment
maturin develop --features pyo3

# Build release wheels
maturin build --release --features pyo3
```

Requires Python 3.9+ (`abi3-py39`). CI builds wheels for Linux (x86_64, x86, aarch64, armv7) and Windows (x64, x86).

## Minimum Supported Rust Version

Both crates use Rust Edition 2024, requiring **Rust 1.85.0** or later.

## Alternatives

### [pubchem](https://crates.io/crates/pubchem)

Synchronous PubChem client using `ureq` and XML parsing. Simple API like `Compound::with_name("aspirin").title()`.

- **Inspired by:** The ergonomic one-liner API design. `CompoundQuery` was modeled after this crate's `Compound` builder pattern.
- **What pubchemrs2 adds:** Async/await support, automatic retry with backoff, POST method selection for SMILES/InChI/SDF queries, coverage beyond the Compound domain (Substance, Assay, Gene, Protein, etc.), batch queries, and optional Python bindings.

## References

- [PubChem PUG REST Documentation](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest)
- [PubChem Glossary — SMILES](https://pubchem.ncbi.nlm.nih.gov/docs/glossary#section=SMILES)

## Contributing

```bash
# Run all tests
cargo test --workspace

# Run single crate tests
cargo test -p pubchemrs_struct
cargo test -p pubchemrs_tokio

# Run integration tests (requires network)
cargo test -p pubchemrs_tokio -- --ignored

# Format and lint
cargo fmt --check
cargo clippy
```
