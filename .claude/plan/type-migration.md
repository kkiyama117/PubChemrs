# Plan: pubchemrs_struct Type Migration with macro_rules!

## Approach

Replace strum derives with 3 custom `macro_rules!` macros. Migrate types from pubchemrs in 5 waves.

## Macros to Define

### 1. `impl_enum_str!` — Display + FromStr + AsRef<str> for simple enums
For enums with straightforward variant-name-to-string mapping.

### 2. `impl_from_repr!` — from_repr(u8/u32) for repr enums
For Element and BondType only.

### 3. `define_element!` — All-in-one macro for Element (122 variants)
Generates enum definition + Display + FromStr + AsRef + from_repr + VARIANTS.

## Migration Waves

### Wave 0: Infrastructure (error types, macros, trait)
- `src/macros.rs` — 3 macro definitions
- `src/error/mod.rs` — PubChemError (core variants only: InvalidInput, ParseResponseError, ParseEnum, Unknown)
- `src/error/err_string.rs` — ErrString

**New dependency**: none (replace `strum::ParseError` with own `ParseEnumError`)

### Wave 1: Core Structs (structs/)
- `src/structs/mod.rs`
- `src/structs/coordinates.rs` — Coordinate, CoordinateType
- `src/structs/atom.rs` — Element (122 variants), Atom
- `src/structs/bond.rs` — BondType, Bond
- `src/structs/classification.rs` — CompoundIdType, CoordinateType (response), ProjectCategory
- `src/structs/compound.rs` — Compound (sync methods only), CompoundCache, CompoundID

**Dependencies**: Wave 0

### Wave 2: Response Types (response/)
- `src/response/mod.rs` — PubChemResponse, PubChemFault
- `src/response/compound/mod.rs` — response::Compound (sync only), CompoundID, Compounds
- `src/response/compound/atom.rs` — AtomInner
- `src/response/compound/bond.rs` — BondInner
- `src/response/compound/charge.rs` — ChargeInner
- `src/response/compound/coordinate.rs` — CoordsInner
- `src/response/compound/conformer.rs` — ConformerInner, ConformaerInnerStyle
- `src/response/compound/others.rs` — CompoundProps, PropsUrn, PropsValue, CompoundTCount, Stereo
- `src/response/information_list.rs` — PubChemInformationList, PubChemInformation

**New dependency**: `itertools` (for response::Compound methods)
**Dependencies**: Wave 0, Wave 1

### Wave 3: Request Builder Types (requests/)
- `src/requests/mod.rs`
- `src/requests/common.rs` — UrlParts trait, XRef
- `src/requests/output.rs` — OutputFormat
- `src/requests/url_builder.rs` — UrlBuilder (sync parts only)
- `src/requests/input/` — Domain, Namespace, all sub-namespaces, Identifiers
- `src/requests/operation/` — Operation, all OperationSpecifications, CompoundProperty, XRefs

**New dependency**: `urlencoding` (for IdentifierValue)
**Dependencies**: Wave 0

### Wave 4: Integration & Tests
- Migrate relevant tests from pubchemrs
- Add round-trip Display/FromStr tests for all enums
- Ensure `cargo test` passes
- Ensure `cargo doc` builds

## Key Decisions

1. **No strum dependency** — all string conversions via macro_rules!
2. **ParseEnumError** replaces `strum::ParseError`
3. **Async methods excluded** — `from_cid()`, `synonyms()` stay in pubchemrs_tokio
4. **pub(crate) → pub** for response inner types (crossing crate boundary)
5. **EnumIs and EnumIter NOT implemented** — zero usage in codebase
6. **itertools** added for response::Compound sync methods
7. **urlencoding** added for IdentifierValue URL encoding

## File Structure (Final)

```
pubchemrs_struct/src/
  lib.rs
  macros.rs               # 3 macro definitions
  properties.rs           # (existing) CompoundProperties
  error/
    mod.rs                # PubChemError, PubChemResult, ParseEnumError
    err_string.rs         # ErrString
  structs/
    mod.rs
    coordinates.rs
    atom.rs               # Element (122 variants)
    bond.rs
    compound.rs           # Compound (sync only)
    classification.rs
  response/
    mod.rs                # PubChemResponse, PubChemFault
    compound/
      mod.rs              # response::Compound (sync only)
      atom.rs
      bond.rs
      charge.rs
      coordinate.rs
      conformer.rs
      others.rs
    information_list.rs
  requests/
    mod.rs
    common.rs             # UrlParts, XRef
    output.rs             # OutputFormat
    url_builder.rs        # UrlBuilder (sync)
    input/
      mod.rs              # InputSpecification
      identifiers.rs
      domain.rs
      namespace/
        mod.rs
        compound.rs
        assay.rs
        substance.rs
        others.rs
    operation/
      mod.rs
      property.rs
      xrefs.rs
```
