# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pubchemrs2 is a Rust library for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest). It consists of a Cargo workspace with two crates:

- **`pubchemrs_struct`** — Pure type definitions and URL builder (zero runtime deps beyond serde). Defines request/response structures, property types, and URL construction logic.
- **`pubchemrs_tokio`** — Async HTTP client using reqwest/tokio. Provides `PubChemClient` with automatic retry, GET/POST selection, and connection pooling.

Optional `pyo3` feature flag on `pubchemrs_struct` enables Python bindings via PyO3/maturin. CI builds Python wheels via `.github/workflows/maturin_build.yml`.

## Build & Test Commands

```bash
# Build workspace
cargo build

# Run all unit tests (offline, no network)
cargo test

# Run tests for a single crate
cargo test -p pubchemrs_struct
cargo test -p pubchemrs_tokio

# Run a single test
cargo test -p pubchemrs_struct -- test_deserialize_partial_properties

# Run integration tests (requires network access to PubChem API)
cargo test -p pubchemrs_tokio -- --ignored

# Build with Python bindings
cargo build -p pubchemrs_struct --features pyo3

# Check formatting / lint
cargo fmt --check
cargo clippy
```

## Architecture

### Request Pipeline

URL construction follows the PubChem PUG REST pattern: `/{domain}/{namespace}/{identifiers}/{operation}/{output}`.

1. **`InputSpecification`** (`pubchemrs_struct/src/requests/input/`) — Combines `Domain`, `Namespace`, and `Identifiers`. Validates input and determines GET vs POST (POST used for formula, InChI, SMILES, SDF searches).
2. **`Operation`** (`pubchemrs_struct/src/requests/operation/`) — What to fetch: Record, Property, Synonyms, XRefs, Dates, etc. Domain-specific operation enums live in separate files (`compound.rs`, `substance.rs`, `assay.rs`, `simple.rs`).
3. **`OutputFormat`** (`pubchemrs_struct/src/requests/output.rs`) — JSON, SDF, etc.
4. **`UrlBuilder`** (`pubchemrs_struct/src/requests/url_builder.rs`) — Assembles all parts into URL path segments + optional POST body via `build_url_parts()`.
5. **`PubChemClient`** (`pubchemrs_tokio/src/client.rs`) — Executes the request with retry logic (linear backoff, retries on 429/503/504).
6. **API methods** (`pubchemrs_tokio/src/api.rs`) — High-level methods: `get_compounds`, `get_properties`, `get_synonyms`, `get_all_sources`.

### Response Types

- **`PubChemResponse`** (`pubchemrs_struct/src/response/mod.rs`) — Root enum dispatching to `Compounds`, `InformationList`, `Fault`, etc.
- **`CompoundProperties`** (`pubchemrs_struct/src/properties.rs`) — Strongly-typed property struct with custom deserializer for string-to-f64 coercion (`MolecularWeight`, `ExactMass`, `MonoisotopicMass` arrive as JSON strings).
- **`Compound`** (`pubchemrs_struct/src/response/compound/`) — Full compound record with atoms, bonds, coordinates, conformers.
- **Structural types** (`pubchemrs_struct/src/structs/`) — Higher-level types (Atom, Bond, Compound, Classification) for converting raw API arrays into usable structs. `structs/convert.rs` implements `TryFrom<&Compound>` for `Vec<Atom>` and `Option<Vec<Bond>>`.

### Key Patterns

- **Enum-based URL parts**: Domain, Namespace, Operation enums implement `UrlParts` trait to produce URL path segments.
- **`impl_enum_str!` macro** (`pubchemrs_struct/src/macros.rs`) — Generates `Display`, `FromStr`, and `AsRef<str>` for enums. Also `impl_from_repr!` and `impl_variant_array!`.
- **`Identifiers`** type accepts various `From` impls (`u32`, `&str`, `Vec<u32>`, etc.) for ergonomic API usage.
- **Global default client**: `PubChemClient::global_default()` uses `OnceLock` for connection pool reuse in free functions.

### SMILES Field Naming

PubChem renamed SMILES fields. The library supports both:
- Current: `SMILES` (full) / `ConnectivitySMILES` (connectivity-only)
- Legacy: `IsomericSMILES` / `CanonicalSMILES`

All four fields exist on `CompoundProperties` for backward compatibility.

## Rust Edition

Both crates use `edition = "2024"`.

## Test Fixtures

Test fixtures live in `pubchemrs_struct/tests/fixtures/` (e.g., `aspirin_properties.json`). Integration tests in `pubchemrs_tokio/tests/integration.rs` are marked `#[ignore]` and require network.
