# pubchemrs_struct

Strongly-typed data structures for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest).

## Features

- **Type-safe request construction** — Build API URLs with domain, namespace, identifier, and operation enums following the PUG REST pattern `/{domain}/{namespace}/{identifiers}/{operation}/{output}`.
- **Strongly-typed responses** — 40+ compound property fields mapped to correct Rust types (`f64`, `u32`, `i32`) with `Option<T>` for all optional fields.
- **Automatic string-to-f64 coercion** — Fields like `MolecularWeight`, `ExactMass`, and `MonoisotopicMass` that arrive as JSON strings are transparently parsed into `f64`.
- **Structural type conversions** — `TryFrom<&Compound>` for `Vec<Atom>` and `Option<Vec<Bond>>` to extract structured atom/bond data from raw API responses.
- **URL builder** — `UrlBuilder` assembles all request parts into URL path segments with optional POST body, automatically selecting GET vs POST.
- **Optional Python bindings** — Enable `pyo3` feature for `#[pyclass]` derives on major types via PyO3.

## Quick Start

```toml
[dependencies]
pubchemrs_struct = { git = "https://github.com/kkiyama117/PubChemrs" }
```

```rust
use pubchemrs_struct::properties::{PropertyTableResponse, CompoundProperties};

// Deserialize a PubChem PropertyTable API response
let json = r#"{
    "PropertyTable": {
        "Properties": [{
            "CID": 2244,
            "MolecularFormula": "C9H8O4",
            "MolecularWeight": "180.16",
            "IUPACName": "2-acetyloxybenzoic acid"
        }]
    }
}"#;

let response: PropertyTableResponse = serde_json::from_str(json).unwrap();
let aspirin = &response.property_table.properties[0];

assert_eq!(aspirin.cid, 2244);
assert_eq!(aspirin.molecular_formula.as_deref(), Some("C9H8O4"));
// MolecularWeight is automatically parsed from string to f64
assert!((aspirin.molecular_weight.unwrap() - 180.16).abs() < 0.01);
```

## Modules

| Module | Description |
|--------|-------------|
| `error` | Error types (`PubChemError`, `ParseEnumError`) with optional panic-on-error and backtrace support |
| `properties` | `CompoundProperties` struct with 40+ fields and custom deserializers for string-to-f64 coercion |
| `requests` | Request construction: `InputSpecification`, `Operation`, `OutputFormat`, and `UrlBuilder` |
| `response` | Raw API response structs (`PubChemResponse`, `Compound`, `InformationList`, `Fault`, etc.) |
| `structs` | Higher-level types (`Atom`, `Bond`, `Compound`) with `TryFrom` conversions from raw responses |

- `requests` should be same as [`Pubchem API`](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest#section=The-URL-Path) definition

## Feature Flags

| Flag | Description |
|------|-------------|
| `pyo3` | Enables `#[pyclass]` derives for Python bindings via PyO3 (requires Python 3.9+) |

## License

Distributed under the MIT License. See [LICENSE](../LICENSE) for details.
