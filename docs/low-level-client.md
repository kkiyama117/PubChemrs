# Low-Level Client

`PubChemClient` provides direct access to the PubChem PUG REST API with automatic retry, GET/POST selection, and connection pooling.

## When to Use

Use `PubChemClient` directly when you need:

- Non-compound domains (Substance, Assay, Gene, Protein, Pathway, Taxonomy, Cell)
- Custom output formats (SDF, etc.)
- Additional query parameters
- Full control over request construction

For common compound queries, the [Convenience API](convenience-api.md) is simpler.

## Creating a Client

```rust,no_run
use pubchemrs_tokio::{PubChemClient, ClientConfig};
use std::time::Duration;

// Default configuration
let client = PubChemClient::default();

// Custom configuration
let client = PubChemClient::new(ClientConfig {
    timeout: Duration::from_secs(60),
    max_retries: 5,
    retry_delay: Duration::from_secs(1),
}).unwrap();
```

A global default client is shared internally via `OnceLock` for connection pool reuse. The convenience API uses this global client unless overridden with `using_client()`.

## API Methods

### `get_compounds()`

Fetch full compound records (atoms, bonds, coordinates, conformers).

```rust,no_run
use pubchemrs_tokio::PubChemClient;
use pubchemrs_struct::requests::input::CompoundNamespace;
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PubChemClient::default();

// By CID
let compounds = client.get_compounds(
    2244u32,
    CompoundNamespace::Cid(),
    HashMap::new(),
).await?;

// By name
let compounds = client.get_compounds(
    "aspirin",
    CompoundNamespace::Name(),
    HashMap::new(),
).await?;
# Ok(())
# }
```

### `get_properties()`

Fetch specific compound properties via the PropertyTable endpoint.

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

println!("MW: {:?}", props[0].molecular_weight);
# Ok(())
# }
```

### `get_synonyms()`

Fetch synonyms for compounds or substances.

```rust,no_run
use pubchemrs_tokio::PubChemClient;
use pubchemrs_struct::requests::input::{Namespace, CompoundNamespace};
use std::collections::HashMap;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PubChemClient::default();
let info_list = client.get_synonyms(
    "caffeine",
    Namespace::Compound(CompoundNamespace::Name()),
    HashMap::new(),
).await?;
# Ok(())
# }
```

The domain (Compound or Substance) is automatically selected based on the namespace.

### `get_all_sources()`

Fetch all source names for a given domain.

```rust,no_run
use pubchemrs_tokio::PubChemClient;
use pubchemrs_struct::requests::input::Domain;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = PubChemClient::default();

// Substance sources (default)
let sources = client.get_all_sources(None).await?;

// Assay sources
let sources = client.get_all_sources(Some(Domain::Assay())).await?;
# Ok(())
# }
```

### Raw Request Methods

For cases not covered by the high-level methods:

| Method | Return Type | Description |
|--------|-------------|-------------|
| `request()` | `String` | Raw response body |
| `get_and_parse()` | `PubChemResponse` | Parsed response enum |
| `get_json()` | `serde_json::Value` | Raw JSON value |
| `get_sdf()` | `String` | Raw SDF text |

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

## URL Pattern

All requests follow the PubChem PUG REST URL pattern:

```
https://pubchem.ncbi.nlm.nih.gov/rest/pug/{domain}/{namespace}/{identifiers}/{operation}/{output}
```

Each segment is modeled as a Rust type:

- `Domain` + `Namespace` + `Identifiers` → `InputSpecification`
- `Operation` — Record, Property, Synonyms, XRefs, etc.
- `OutputFormat` — JSON, SDF, etc.
- `UrlBuilder` assembles all parts and determines GET vs POST.

### GET vs POST Auto-Selection

The library automatically uses POST for namespaces that require it (SMILES, InChI, SDF, Formula, Structure Search, Fast Search). The POST body is `application/x-www-form-urlencoded`.

See [Architecture](architecture.md) for the full request pipeline.
