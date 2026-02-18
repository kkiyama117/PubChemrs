# Refactoring Plan: pubchemrs_struct — SOLID Data Structures

## Requirements Restatement

Refactor `pubchemrs_struct` so that all structs in `requests/`, `response/`, and `structs/` are **simple data structures** with minimal methods, following SOLID principles. Conversion logic (e.g., `response::Compound` → `structs::Atom`) should use `Into`/`TryInto` trait implementations, not methods on the data structs.

At each phase, verify that the implementation matches the PubChem PUG REST API definitions.

## API Context (from PubChem PUG REST docs)

The PUG REST URL pattern is: `/{domain}/{namespace}/{identifiers}/{operation}/{output}`

- **Domains**: compound, substance, assay, gene, protein, pathway, taxonomy, cell + special "other inputs" (sources, sourcetable, classification, periodictable, standardize, conformers, annotations)
- **Operations**: record, property/{tags}, synonyms, sids, cids, aids, xrefs/{types}, description, classification, conformers, assaysummary, summary, concise, targets/{type}, doseresponse/sid, dates
- **Output**: JSON, XML, SDF, CSV, PNG, TXT, ASNT, ASNB, JSONP
- **SMILES naming**: PubChem renamed `IsomericSMILES` → `SMILES` and `CanonicalSMILES` → `ConnectivitySMILES` (glossary confirms both old/new names)
- **XRefs**: RegistryID, RN, PubMedID, MMDBID, DBURL, SBURL, ProteinGI, NucleotideGI, TaxonomyID, MIMID, GeneID, ProbeID, PatentID, SourceName, SourceCategory
- **Properties**: MolecularFormula, MolecularWeight, SMILES, ConnectivitySMILES, InChI, InChIKey, IUPACName, Title, XLogP, ExactMass, MonoisotopicMass, TPSA, Complexity, Charge, HBondDonorCount, HBondAcceptorCount, RotatableBondCount, HeavyAtomCount, IsotopeAtomCount, AtomStereoCount, DefinedAtomStereoCount, UndefinedAtomStereoCount, BondStereoCount, DefinedBondStereoCount, UndefinedBondStereoCount, CovalentUnitCount, PatentCount, PatentFamilyCount, AnnotationTypes, AnnotationTypeCount, SourceCategories, LiteratureCount, Volume3D, XStericQuadrupole3D, YStericQuadrupole3D, ZStericQuadrupole3D, FeatureCount3D, FeatureAcceptorCount3D, FeatureDonorCount3D, FeatureAnionCount3D, FeatureCationCount3D, FeatureRingCount3D, FeatureHydrophobeCount3D, ConformerModelRMSD3D, EffectiveRotorCount3D, ConformerCount3D, Fingerprint2D
- **Assay target types**: ProteinGI, ProteinName, GeneID, GeneSymbol
- **PC_Compounds record**: atoms (aid[], element[], charge[]), bonds (aid1[], aid2[], order[]), coords (aid[], conformers[{x[], y[], z[]}], type[]), props (urn{label, name, ...}, value{ival|fval|sval|binary}), count, stereo, id

---

## Current SOLID Violations

### 1. SRP: `response::Compound` is data struct + converter (CRITICAL)

**File:** `response/compound/mod.rs` (313 LOC)

`Compound` has 5 business-logic methods that don't belong on a data struct:
- `setup_atoms()` — converts raw arrays → `Vec<structs::Atom>` (60+ LOC)
- `setup_bonds()` — converts raw arrays → `Vec<structs::Bond>` (50+ LOC)
- `parse_coords()` — converts raw arrays → `HashMap<u32, Coordinate>` (55+ LOC)
- `parse_prop_by_label()` — searches props array
- `parse_prop_by_label_and_name()` — searches props array
- `as_dataframe()` — `todo!()` dead code

**Fix:** Extract `setup_atoms`/`setup_bonds`/`parse_coords` into `TryFrom<&response::Compound>` impls. Keep `parse_prop_by_label*` as inherent methods (they are lightweight accessors on own data).

### 2. SRP: `InputSpecification` — NO CHANGE NEEDED

**File:** `requests/input/mod.rs` (120 LOC)

`validate()`, `use_post()`, `to_url_parts_with_body()` implement the `UrlParts` trait — this is the struct's core responsibility. The design follows Interface Segregation well.

### 3. SRP: `operation/mod.rs` is 1,079 LOC

**File:** `requests/operation/mod.rs`

Contains 8 domain-specific operation enums + `Operation` wrapper + all `From` impls + 500 LOC tests.

**Fix:** Split into domain-specific files.

### 4. Immutability: `Bond::set_style(&mut self)`

**File:** `structs/bond.rs:31`

**Fix:** Replace with `with_style(self, style) -> Self`.

### 5. Dead code: `Compound::as_dataframe()`

**File:** `response/compound/mod.rs:59-62` — `pub fn as_dataframe() { todo!() }`

**Fix:** Remove entirely.

### 6. Code smell: `Atom::_from_record_data()` underscore prefix

**File:** `structs/atom.rs:51`

**Fix:** Rename to `from_record_data()` (no underscore). Keep `pub(crate)`.

---

## Implementation Plan

### Phase 1: Extract conversion logic from `response::Compound`

**Goal:** `response::Compound` becomes a pure data struct.

1. **Create `structs/convert.rs`** — new module for conversions
2. **Move `parse_coords` logic** into a private helper `fn parse_coords(compound: &response::Compound) -> PubChemResult<Option<HashMap<u32, Coordinate>>>`
3. **Implement `TryFrom<&response::Compound> for Vec<Atom>`** — uses `parse_coords` internally
4. **Implement `TryFrom<&response::Compound> for Option<Vec<Bond>>`** — move `setup_bonds()` logic
5. **Remove** `setup_atoms()`, `setup_bonds()`, `parse_coords()`, `as_dataframe()` from `response::Compound`
6. **Keep** `parse_prop_by_label()` and `parse_prop_by_label_and_name()` on `Compound`
7. **Update `pubchemrs_tokio`** callers if any reference `setup_atoms`/`setup_bonds`

**PubChem API conformance check:**
- [ ] `response::Compound` fields match PC_Compounds JSON: `atoms{aid,element,charge}`, `bonds{aid1,aid2,order}`, `coords{aid,conformers,type}`, `props{urn,value}`, `count`, `stereo`, `id`
- [ ] `structs::Atom` conversion correctly maps: `atoms.aid[i]` → aid, `atoms.element[i]` → Element enum (u8 repr matching PubChem element numbers), `coords.conformers[0].x[i]/y[i]/z[i]` → Coordinate, `atoms.charge[].{aid,value}` → charge
- [ ] `structs::Bond` conversion correctly maps: `bonds.aid1[i]`/`bonds.aid2[i]` → aid1/aid2, `bonds.order[i]` → BondType (1=single,2=double,3=triple per PubChem spec), `coords.conformers[0].style.{aid1,aid2,annotation}` → style
- [ ] Existing test fixtures (e.g. `aspirin_properties.json`) still deserialize correctly

### Phase 2: Fix `Bond` immutability

1. **Replace** `set_style(&mut self, ...)` with `with_style(self, ...) -> Self`
2. **Update** conversion code in `structs/convert.rs` to use `with_style` (map instead of mutate)

**PubChem API conformance check:**
- [ ] Bond style annotation values from `ConformerInnerStyle` still correctly propagated (PubChem encodes stereo wedge/dash as annotation integers in coords.conformers[].style)
- [ ] `BondType` repr values match PubChem: Single=1, Double=2, Triple=3, Quadruple=4, Dative=5, Complex=6, Ionic=7, Unknown=255

### Phase 3: Split `operation/mod.rs`

1. **Create** `operation/compound.rs` — `CompoundOperationSpecification` + Display/FromStr/Default + tests
2. **Create** `operation/substance.rs` — `SubstanceOperationSpecification` + tests
3. **Create** `operation/assay.rs` — `AssayOperationSpecification` + `AssayOperationTargetType` + tests
4. **Create** `operation/simple.rs` — Gene/Protein/PathWay/Taxonomy/Cell specs + tests
5. **Slim down** `operation/mod.rs` to `Operation` enum + `From` impls + re-exports + Operation tests only

**PubChem API conformance check:**
- [ ] Compound operations match API: record, property/{tags}, synonyms, sids, cids, aids, assaysummary, classification, xrefs/{types}, description, conformers
- [ ] Substance operations match API: record, synonyms, sids, cids, aids, assaysummary, classification, xrefs/{types}, description
- [ ] Assay operations match API: record, concise, aids, cids, sids, description, targets/{ProteinGI|ProteinName|GeneID|GeneSymbol}, doseresponse/sid, summary, classification
- [ ] Gene operations match API: summary, aids, concise, pwaccs
- [ ] Protein operations match API: summary, aids, concise, pwaccs
- [ ] PathWay operations match API: summary, cids, concise, pwaccs
- [ ] Taxonomy operations match API: summary, aids
- [ ] Cell operations match API: summary, aids
- [ ] All `Display` outputs produce correct URL path segments (e.g. `"property/MolecularFormula,MolecularWeight"`, `"xrefs/RegistryID"`, `"targets/proteingi"`)
- [ ] All `FromStr` round-trips: `Display` → `FromStr` → same value

### Phase 4: Clean up `Atom` constructor

1. **Rename** `_from_record_data()` → `from_record_data()` (remove underscore prefix)
2. Keep `pub(crate)` visibility

**PubChem API conformance check:**
- [ ] Element enum covers all PubChem element numbers (1-118 standard + 252=Lp, 253=R, 254=Dummy, 255=Unspecified)
- [ ] `Atom.number` (u8) matches PubChem's `atoms.element[]` integer values

---

## Files Modified

| Phase | File | Action |
|-------|------|--------|
| 1 | `structs/mod.rs` | Add `mod convert;` (private) |
| 1 | `structs/convert.rs` | **NEW** — TryFrom impls for Atom/Bond conversion |
| 1 | `response/compound/mod.rs` | Remove `setup_atoms`, `setup_bonds`, `parse_coords`, `as_dataframe`; remove `use itertools` |
| 1 | `pubchemrs_tokio/src/api.rs` | Update callers (if any) |
| 2 | `structs/bond.rs` | `set_style` → `with_style` |
| 3 | `requests/operation/compound.rs` | **NEW** |
| 3 | `requests/operation/substance.rs` | **NEW** |
| 3 | `requests/operation/assay.rs` | **NEW** |
| 3 | `requests/operation/simple.rs` | **NEW** |
| 3 | `requests/operation/mod.rs` | Slim down (~200 LOC from 1,079) |
| 4 | `structs/atom.rs` | Rename `_from_record_data` |

## Risks

- **MEDIUM**: `pubchemrs_tokio` may call `setup_atoms()`/`setup_bonds()` directly — need to update callers to use `TryFrom`
- **LOW**: Splitting `operation/mod.rs` is mechanical but touches tests — need careful re-export to avoid breaking public API
- **LOW**: `with_style` change is minor but internal callers need updating

## Verification (per phase)

1. `cargo test` — all tests pass
2. `cargo clippy` — no new warnings
3. `cargo doc --no-deps` — zero warnings
4. PubChem API conformance checklist (see each phase above)
5. Public API changes: only `setup_atoms`/`setup_bonds` removed (replaced by `TryFrom`), `as_dataframe` removed, `set_style` → `with_style`

---

## TODO: Existing pytest failures

The following 6 Python test failures exist independently of the refactoring work above and need to be addressed separately.

### 3D Compound tests (`test_compound3d.py`) — 5 failures

CID 1234 is used as a 3D compound fixture but the PubChem API currently returns 2D-only data for it.

- [ ] **test_properties_types** — `c3d.volume_3d` returns `None` (expects `float`). The API no longer returns 3D conformer data with `Volume` for this CID.
- [ ] **test_coordinate_type** — `c3d.coordinate_type` returns `"2d"` (expects `"3d"`). The API returns `CoordinateType.TWO_D` instead of `THREE_D`.
- [ ] **test_coordinates** — `a.z` is `None` for all atoms (expects `float | int`). No z-coordinates in the 2D response.
- [ ] **test_coordinates_deprecated** — Same root cause as test_coordinates, via deprecated dict-access API.
- [ ] **test_atoms_deprecated** — `w[0].category` is `DeprecationWarning` (expects `PubChemPyDeprecationWarning`). The Rust `Atom.__getitem__` uses the built-in `DeprecationWarning` instead of the custom warning class.

**Possible fixes:**
1. Replace CID 1234 with a CID that reliably has 3D conformer data, or use a local fixture file.
2. For the deprecation warning mismatch: update Rust `Atom` to emit `PubChemPyDeprecationWarning`, or update the test expectation.

### Formula search (`test_pandas.py`) — 1 failure

- [ ] **test_compounds_dataframe** — `get_compounds("C20H41Br", "formula")` raises `ValueError: unknown variant 'Waiting'`. The Rust `PubChemResponse` enum does not handle the `Waiting` (listkey polling) response from formula searches.

**Possible fix:** Add a `Waiting` variant to `PubChemResponse` and implement listkey polling in the Rust client, or fall back to the legacy HTTP path for formula namespace (as is already done for `searchtype`).
