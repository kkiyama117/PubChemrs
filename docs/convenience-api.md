# Convenience API

The convenience API provides `CompoundQuery` and `OtherInputsQuery` as high-level entry points for common PubChem queries. They build on the lower-level `PubChemClient` methods and handle client creation, URL construction, and response parsing automatically.

## CompoundQuery

### Constructors

| Constructor | Input | HTTP Method |
|-------------|-------|-------------|
| `CompoundQuery::with_name("aspirin")` | Compound name | GET |
| `CompoundQuery::with_cid(2244)` | PubChem Compound ID | GET |
| `CompoundQuery::with_cids(&[2244, 962])` | Multiple CIDs (batch) | GET |
| `CompoundQuery::with_smiles("CC(=O)O")` | SMILES string | POST |
| `CompoundQuery::with_inchikey("BSYN...")` | InChIKey | GET |
| `CompoundQuery::with_formula("C9H8O4")` | Molecular formula | POST |

### Single-Property Accessors

Each accessor makes one HTTP request and returns the property value for the first matching compound.

| Method | Return Type | Property Tag |
|--------|-------------|--------------|
| `molecular_formula()` | `Option<String>` | MolecularFormula |
| `molecular_weight()` | `Option<f64>` | MolecularWeight |
| `iupac_name()` | `Option<String>` | IUPACName |
| `inchi()` | `Option<String>` | InChI |
| `inchikey()` | `Option<String>` | InChIKey |
| `smiles()` | `Option<String>` | IsomericSMILES |
| `canonical_smiles()` | `Option<String>` | CanonicalSMILES |
| `xlogp()` | `Option<f64>` | XLogP |
| `exact_mass()` | `Option<f64>` | ExactMass |
| `tpsa()` | `Option<f64>` | TPSA |
| `charge()` | `Option<i32>` | Charge |

All accessors are `async` and return `Result<Option<T>>`.

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let formula = CompoundQuery::with_name("aspirin")
    .molecular_formula()
    .await?;
println!("Formula: {formula:?}"); // Some("C9H8O4")
# Ok(())
# }
```

### Multiple Properties

When you need multiple properties, use `properties()` to fetch them in a single HTTP request instead of calling individual accessors.

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let props = CompoundQuery::with_cid(2244)
    .properties(&["MolecularWeight", "InChIKey", "MolecularFormula"])
    .await?;
println!("MW: {:?}", props[0].molecular_weight);   // Some(180.16)
println!("InChIKey: {:?}", props[0].inchikey);      // Some("BSYNRYMUTXBXSQ-...")
println!("Formula: {:?}", props[0].molecular_formula); // Some("C9H8O4")
# Ok(())
# }
```

Returns `Vec<CompoundProperties>` — one element per matched compound (typically one for single-compound queries).

### Synonyms

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let synonyms = CompoundQuery::with_name("caffeine")
    .synonyms()
    .await?;
println!("Found {} synonyms", synonyms.len());
# Ok(())
# }
```

### Compound Records

Fetch the full compound record (atoms, bonds, coordinates, conformers):

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// Single compound
let compound = CompoundQuery::with_cid(2244).record().await?;

// Multiple compounds
let compounds = CompoundQuery::with_cids(&[2244, 962]).records().await?;
# Ok(())
# }
```

### Discovering CIDs

Find the PubChem Compound ID from a name, SMILES, or InChIKey:

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let cid = CompoundQuery::with_name("aspirin").cid().await?;
println!("CID: {cid:?}"); // Some(2244)
# Ok(())
# }
```

### Batch Queries

Query multiple compounds in a single HTTP request:

```rust,no_run
use pubchemrs_tokio::CompoundQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let batch = CompoundQuery::with_cids(&[2244, 962, 5793])
    .properties(&["MolecularFormula", "MolecularWeight"])
    .await?;

for props in &batch {
    println!("CID {}: {:?}", props.cid, props.molecular_formula);
}
# Ok(())
# }
```

## OtherInputsQuery

For PubChem endpoints that don't deal with compound/substance/assay identifiers.

### Source Listings

```rust,no_run
use pubchemrs_tokio::OtherInputsQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// List all substance depositors
let sources = OtherInputsQuery::substance_sources().fetch().await?;

// List all assay depositors
let sources = OtherInputsQuery::assay_sources().fetch().await?;
# Ok(())
# }
```

### Periodic Table

```rust,no_run
use pubchemrs_tokio::OtherInputsQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
// Returns raw JSON (response type not yet modeled)
let table = OtherInputsQuery::periodic_table().fetch_json().await?;
# Ok(())
# }
```

### `fetch()` vs `fetch_json()`

| Method | Use Case | Return Type |
|--------|----------|-------------|
| `fetch()` | Source listings (substance/assay sources) | `Vec<String>` |
| `fetch_json()` | Any endpoint, returns raw JSON | `serde_json::Value` |

`fetch()` returns an error if called on non-source endpoints (e.g. periodic table).

## Custom Client

Both query builders use a global default `PubChemClient` by default. To use a custom client with different configuration:

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

### ClientConfig

| Option | Default | Description |
|--------|---------|-------------|
| `timeout` | 30s | HTTP request timeout |
| `max_retries` | 3 | Retry count on retryable errors |
| `retry_delay` | 500ms | Base delay between retries (linear backoff) |

See [Error Handling](error-handling.md) for details on retry behavior.

## SMILES Field Naming

PubChem renamed its SMILES fields. `CompoundProperties` includes both current and legacy fields for compatibility:

| Current Name | JSON Key | Rust Field | Content |
|-------------|----------|------------|---------|
| SMILES | `SMILES` | `smiles` | Full SMILES with stereochemistry and isotopes |
| Connectivity SMILES | `ConnectivitySMILES` | `connectivity_smiles` | Connectivity only |
| *(legacy)* Canonical SMILES | `CanonicalSMILES` | `canonical_smiles` | Same as Connectivity SMILES |
| *(legacy)* Isomeric SMILES | `IsomericSMILES` | `isomeric_smiles` | Same as SMILES |

The `smiles()` accessor falls back to the legacy `IsomericSMILES` field if the current `SMILES` field is absent. Similarly, `canonical_smiles()` falls back to `CanonicalSMILES`.

Prefer `smiles` and `connectivity_smiles` for new code.
