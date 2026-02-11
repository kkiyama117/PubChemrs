# pubchemrs2

Async Rust client for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest) with strongly-typed responses and optional Python bindings.

## Features

- **Strongly-typed responses** — 40+ compound property fields mapped to correct Rust types (`f64`, `u32`, `Option<T>`). Numeric fields returned as JSON strings (`MolecularWeight`, `ExactMass`, `MonoisotopicMass`) are automatically parsed to `f64`.
- **Async HTTP client** — Built on `reqwest` + `tokio` with connection pooling, automatic retry on 429/503/504, and linear backoff.
- **Automatic GET/POST selection** — Searches by InChI, SMILES, SDF, or Formula automatically use POST as required by the PubChem API.
- **Comprehensive API coverage** — Compound, Substance, Assay, Gene, Protein, Pathway, Taxonomy, and Cell domains. Structure search (substructure/superstructure/similarity/identity) and fast search (2D/3D similarity).
- **Convenience free functions** — `get_compounds()`, `get_properties()`, `get_synonyms()`, `get_all_sources()` available at module level using a shared global client.
- **Optional Python bindings** — Enable `pyo3` feature flag for `#[pyclass]` derives on all major types. CI builds maturin wheels for Linux and Windows.

## Quick Start

### Installation

```toml
[dependencies]
pubchemrs_tokio = { git = "https://github.com/your-org/pubchemrs2" }
```

For type definitions only (no HTTP dependencies):

```toml
[dependencies]
pubchemrs_struct = { git = "https://github.com/your-org/pubchemrs2" }
```

### Basic Usage

```rust,no_run
use pubchemrs_tokio::PubChemClient;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PubChemClient::default();
    let props = client.get_properties(
        "aspirin",
        CompoundNamespace::Name(),
        &["MolecularWeight".into(), "InChIKey".into()],
        HashMap::new(),
    ).await?;

    println!("MW: {:?}", props[0].molecular_weight);      // Some(180.16)
    println!("InChIKey: {:?}", props[0].inchikey);         // Some("BSYNRYMUTXBXSQ-...")
    Ok(())
}
```

### Using Free Functions

```rust,no_run
use pubchemrs_tokio::get_properties;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let props = get_properties(
    2244u32,
    CompoundNamespace::Cid(),
    &["MolecularFormula".into()],
    HashMap::new(),
).await?;
# Ok(())
# }
```

## API Examples

### Get Full Compound Records

```rust,no_run
use pubchemrs_tokio::get_compounds;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// By CID
let compounds = get_compounds(2244u32, CompoundNamespace::Cid(), HashMap::new()).await?;

// By name
let compounds = get_compounds("aspirin", CompoundNamespace::Name(), HashMap::new()).await?;

// By SMILES (automatically uses POST)
let compounds = get_compounds(
    "CC(=O)OC1=CC=CC=C1C(=O)O",
    CompoundNamespace::Smiles(),
    HashMap::new(),
).await?;
# Ok(())
# }
```

### Get Compound Properties

```rust,no_run
use pubchemrs_tokio::get_properties;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let props = get_properties(
    "water",
    CompoundNamespace::Name(),
    &[
        "MolecularWeight".into(),
        "MolecularFormula".into(),
        "InChIKey".into(),
        "HBondDonorCount".into(),
    ],
    HashMap::new(),
).await?;

let water = &props[0];
println!("Formula: {:?}", water.molecular_formula);   // Some("H2O")
println!("MW: {:?}", water.molecular_weight);          // Some(18.015)
println!("HBD: {:?}", water.h_bond_donor_count);       // Some(1)
# Ok(())
# }
```

### Get Synonyms

```rust,no_run
use pubchemrs_tokio::get_synonyms;
use pubchemrs_struct::requests::input::*;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let synonyms = get_synonyms(
    "caffeine",
    Namespace::Compound(CompoundNamespace::Name()),
    HashMap::new(),
).await?;

for name in &synonyms[0].synonym {
    println!("{name}");
}
# Ok(())
# }
```

### Get All Sources

```rust,no_run
use pubchemrs_tokio::get_all_sources;
use pubchemrs_struct::requests::input::Domain;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// Substance sources (default)
let sources = get_all_sources(None).await?;

// Assay sources
let sources = get_all_sources(Some(Domain::Assay())).await?;
# Ok(())
# }
```

## Client Configuration

```rust,no_run
use pubchemrs_tokio::{PubChemClient, ClientConfig};
use std::time::Duration;

let client = PubChemClient::new(ClientConfig {
    timeout: Duration::from_secs(60),
    max_retries: 5,
    retry_delay: Duration::from_secs(1),
}).unwrap();
```

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
