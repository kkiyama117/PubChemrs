# Architecture

## Crate Structure

```
pubchemrs2/
  pubchemrs_struct/   Type definitions only (serde, no HTTP)
  pubchemrs_tokio/    Async HTTP client (reqwest + tokio)
```

### `pubchemrs_struct`

Pure type definitions and URL builder with zero runtime dependencies beyond serde. Contains:

- Request types: `InputSpecification`, `Domain`, `Namespace`, `Identifiers`, `Operation`, `OutputFormat`, `UrlBuilder`
- Response types: `PubChemResponse`, `Compound`, `CompoundProperties`, `PubChemInformation`
- Structural types: `Atom`, `Bond`, `Compound`, `Classification` (higher-level types for converting raw API arrays)
- Structural conversions (`structs/convert.rs`): `TryFrom<&Compound>` for `Vec<Atom>` and `Option<Vec<Bond>>`
- Error types: `PubChemError`, `ErrString`
- Macros: `impl_enum_str!`, `impl_from_repr!`, `impl_variant_array!`

### `pubchemrs_tokio`

Async HTTP client built on `reqwest` + `tokio`. Contains:

- `PubChemClient` — HTTP client with retry logic and connection pooling
- `CompoundQuery` / `OtherInputsQuery` — Convenience API builders
- API methods: `get_compounds`, `get_properties`, `get_synonyms`, `get_all_sources`
- Error type wrapping `pubchemrs_struct` errors with HTTP-specific variants

`pubchemrs_tokio` re-exports `pubchemrs_struct` for convenience.

## Request Pipeline

URL construction follows the PubChem PUG REST pattern:

```
/{domain}/{namespace}/{identifiers}/{operation}/{output}
```

The pipeline consists of these steps:

1. **`InputSpecification`** (`pubchemrs_struct/src/requests/input/`)
   - Combines `Domain`, `Namespace`, and `Identifiers`
   - Validates input and determines GET vs POST
   - POST is used for Formula, InChI, SMILES, SDF searches

2. **`Operation`** (`pubchemrs_struct/src/requests/operation/`)
   - What to fetch: Record, Property, Synonyms, XRefs, Dates, etc.
   - Domain-specific operation enums: `CompoundOperationSpecification` (`compound.rs`), `SubstanceOperationSpecification` (`substance.rs`), `AssayOperationSpecification` (`assay.rs`), and simpler domain operations (`simple.rs`)
   - `CompoundProperty` holds a list of property tags

3. **`OutputFormat`** (`pubchemrs_struct/src/requests/output.rs`)
   - JSON, SDF, etc.

4. **`UrlBuilder`** (`pubchemrs_struct/src/requests/url_builder.rs`)
   - Assembles all parts into URL path segments + optional POST body
   - `build_url_parts()` returns `(Vec<String>, Option<String>)` — path segments and optional body

5. **`PubChemClient`** (`pubchemrs_tokio/src/client.rs`)
   - Builds the full URL from path segments
   - Executes the request with retry logic (linear backoff on 429/503/504)
   - Automatically selects GET or POST based on whether a body is present

6. **API methods** (`pubchemrs_tokio/src/api.rs`)
   - High-level methods that construct `UrlBuilder` and parse responses
   - `get_compounds`, `get_properties`, `get_synonyms`, `get_all_sources`

## Response Types

### `PubChemResponse`

Root enum (`pubchemrs_struct/src/response/mod.rs`) that dispatches to:

- `Compounds` — List of full compound records
- `InformationList` — Synonyms, source names, etc.
- `Fault` — API error response

### `CompoundProperties`

Strongly-typed property struct (`pubchemrs_struct/src/properties.rs`) with 40+ fields organized by category:

| Category | Fields |
|----------|--------|
| **Identifiers** | `cid`, `inchi`, `inchikey`, `iupac_name` |
| **SMILES** | `smiles`, `connectivity_smiles`, `canonical_smiles` (legacy), `isomeric_smiles` (legacy) |
| **Physical** | `molecular_formula`, `molecular_weight`, `exact_mass`, `monoisotopic_mass`, `charge` |
| **Descriptors** | `xlogp`, `tpsa`, `complexity` |
| **Counts** | `h_bond_donor_count`, `h_bond_acceptor_count`, `rotatable_bond_count`, `heavy_atom_count`, `isotope_atom_count`, `covalent_unit_count` |
| **Stereochemistry** | `atom_stereo_count`, `defined_atom_stereo_count`, `undefined_atom_stereo_count`, `bond_stereo_count`, `defined_bond_stereo_count`, `undefined_bond_stereo_count` |
| **Fingerprint** | `fingerprint` (hex-encoded 881-bit PubChem fingerprint) |
| **3D** | `volume_3d`, `conformer_rmsd_3d`, `effective_rotor_count_3d`, `conformer_count_3d`, steric quadrupoles, pharmacophore feature counts |

All fields except `cid` are `Option<T>` — unrequested properties deserialize as `None`.

**Type coercion**: `MolecularWeight`, `ExactMass`, and `MonoisotopicMass` arrive as JSON strings from the PubChem API. A custom deserializer automatically parses them to `f64`.

### `Compound`

Full compound record (`pubchemrs_struct/src/response/compound/`) with atoms, bonds, coordinates, and conformers. Structural types in `pubchemrs_struct/src/structs/` provide higher-level representations.

### Structural Type Conversions

`pubchemrs_struct/src/structs/convert.rs` implements `TryFrom<&Compound>` conversions for extracting structured data from raw API responses:

- **`TryFrom<&Compound> for Vec<Atom>`** — Extracts atom IDs, elements, coordinates (2D/3D), and charges into `Atom` structs.
- **`TryFrom<&Compound> for Option<Vec<Bond>>`** — Extracts bond pairs, orders, and style annotations into `Bond` structs. Returns `None` if no bond data is present. Bonds are sorted by `(aid1, aid2)` and style annotations from conformer data are applied. `Bond::is_same_bond_with_aid()` uses bidirectional matching.

## Key Patterns

### Enum-Based URL Parts

`Domain`, `Namespace`, and `Operation` enums implement a `UrlParts` trait to produce URL path segments. This ensures type-safe URL construction.

### `impl_enum_str!` Macro

(`pubchemrs_struct/src/macros.rs`) Generates `Display`, `FromStr`, and `AsRef<str>` implementations for enums. Related macros include `impl_from_repr!` and `impl_variant_array!`.

### Ergonomic `Identifiers`

The `Identifiers` type accepts various `From` implementations (`u32`, `&str`, `Vec<u32>`, etc.) for flexible API usage:

```rust,no_run
use pubchemrs_struct::requests::input::Identifiers;

let id: Identifiers = 2244u32.into();       // Single CID
let id: Identifiers = "aspirin".into();      // Name string
```

### Global Default Client

`PubChemClient::global_default()` uses `OnceLock` for connection pool reuse across the convenience API's free functions.
