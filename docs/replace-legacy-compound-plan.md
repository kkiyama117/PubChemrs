# Plan: Replace Legacy Python Compound with Rust PyO3 Implementation

## Context

The project has two Compound implementations:
- **Rust PyO3 `PyCompound`** (`pubchemrs_pyo3/src/compound.rs`): 44+ property getters, `to_dict()`, `__repr__`, `__eq__`, OnceLock caching for atoms/bonds
- **Legacy Python `Compound`** (`pubchemrs_pyo3/python/pubchemrs/legacy/compound.py`): 1000+ line pure Python wrapper around dict records

Goal: Add missing features to Rust PyCompound so legacy Compound can be replaced. Keep pandas helpers (`to_series`, `compounds_to_frame`) as thin Python wrappers.

## Missing Features in Rust PyCompound

| Feature | Legacy Location | Description |
|---------|----------------|-------------|
| `from_cid(cid)` | L401-416 | Classmethod: fetch Compound by CID |
| `synonyms` | L525-535 | Memoized property, API call to `/compound/cid/{cid}/synonyms` |
| `sids` | L537-547 | Memoized property, API call to `/compound/cid/{cid}/sids` |
| `aids` | L549-559 | Memoized property, API call to `/compound/cid/{cid}/aids` |

## Key Technical Challenge: `PubChemInformation` SID field

The `SID` JSON field has two shapes depending on endpoint:
- Synonyms: `"SID": 12345` (single u32, for substance synonyms)
- Sids: `"SID": [1234, 5678, ...]` (list, for compound related sids)

**Solution**: Custom serde deserializer `deserialize_u32_or_vec` that accepts both single u32 and `Vec<u32>`. Change `sid: Option<u32>` → `sids: Vec<u32>`. Add helper `fn first_sid() -> Option<u32>` for backward compat.

---

## Phase 1: Extend `PubChemInformation` for SID/AID lists

### 1.1 Add custom deserializer + new fields
**File**: `pubchemrs_struct/src/response/information_list.rs`

- Add `deserialize_u32_or_vec` function (handles both `123` and `[123, 456]`)
- Change `sid: Option<u32>` → `sids: Vec<u32>` with `#[serde(rename = "SID", default, deserialize_with = "deserialize_u32_or_vec")]`
- Add `aids: Vec<u32>` with `#[serde(rename = "AID", default, deserialize_with = "deserialize_u32_or_vec")]`
- Add `fn first_sid() -> Option<u32>` and `fn first_cid() -> Option<u32>` helpers

### 1.2 Fix all usages of `info.sid` and `info.cid`
**Files**: Grep for `.sid` and `.cid` on `PubChemInformation` across workspace. Update to use `.sids.first()` or keep if already `Option`.

Note: `cid` field is already `Option<u32>` and stays as-is (CID is always single). Only SID has the dual-shape issue.

### 1.3 Add deserialization tests
- Test JSON with single SID → `sids: vec![12345]`
- Test JSON with SID array → `sids: vec![1, 2, 3]`
- Test JSON without SID → `sids: vec![]`
- Test JSON with AID array → `aids: vec![1, 2, 3]`

---

## Phase 2: Add `get_sids()` / `get_aids()` to Rust API

### 2.1 `pubchemrs_tokio/src/api.rs`
Add methods following `get_synonyms()` pattern (L140-168):

```rust
pub async fn get_sids(...) -> Result<Vec<PubChemInformation>>
pub async fn get_aids(...) -> Result<Vec<PubChemInformation>>
```

Use `CompoundOperationSpecification::Sids()` / `CompoundOperationSpecification::Aids()` (already exist in operation enums).

### 2.2 URL construction tests
Add `test_get_sids_url`, `test_get_aids_url` verifying correct URL paths.

---

## Phase 3: Expose `get_sids_sync` / `get_aids_sync` in PyO3 client

### 3.1 `pubchemrs_pyo3/src/client.rs`
Follow `get_synonyms` / `get_synonyms_sync` pattern (L155-197). Add 4 methods:
- `get_sids` (async) + `get_sids_sync`
- `get_aids` (async) + `get_aids_sync`

---

## Phase 4: Add `from_cid()`, `synonyms`, `sids`, `aids` to PyCompound

### 4.1 Add cache fields to `PyCompound` struct
**File**: `pubchemrs_pyo3/src/compound.rs`

```rust
pub struct PyCompound {
    record: CompoundResponse,
    atoms_cache: OnceLock<Vec<Atom>>,
    bonds_cache: OnceLock<Vec<Bond>>,
    synonyms_cache: OnceLock<Vec<String>>,  // NEW
    sids_cache: OnceLock<Vec<u32>>,         // NEW
    aids_cache: OnceLock<Vec<u32>>,         // NEW
}
```

Update `from_record()` to initialize new fields with `OnceLock::new()`.

### 4.2 Add global runtime helper
**File**: `pubchemrs_pyo3/src/compound.rs`

```rust
fn global_runtime_and_client() -> &'static (tokio::runtime::Runtime, PubChemClient) {
    static GLOBAL: OnceLock<(Runtime, PubChemClient)> = OnceLock::new();
    GLOBAL.get_or_init(|| { /* create runtime + client */ })
}
```

### 4.3 Add `from_cid()` classmethod

```rust
#[classmethod]
fn from_cid(_cls: &Bound<'_, PyType>, cid: u32, py: Python<'_>) -> PyResult<Self> {
    let (rt, client) = global_runtime_and_client();
    let records = py.allow_threads(|| {
        rt.block_on(client.get_compounds(cid, CompoundNamespace::Cid(), HashMap::new()))
    }).map_err(to_pyerr)?;
    records.into_iter().next()
        .map(PyCompound::from_record)
        .ok_or_else(|| NotFoundError::new_err("No compound found"))
}
```

**GIL safety**: Use `py.allow_threads()` to release GIL during `block_on()`.

### 4.4 Add `synonyms` property

```rust
#[getter]
fn synonyms(&self, py: Python<'_>) -> PyResult<Vec<String>> {
    let cid = self.cid().ok_or_else(|| PyValueError::new_err("No CID"))?;
    // OnceLock can't propagate errors, so use try_with pattern
    if let Some(cached) = self.synonyms_cache.get() {
        return Ok(cached.clone());
    }
    let (rt, client) = global_runtime_and_client();
    let result = py.allow_threads(|| {
        rt.block_on(client.get_synonyms(cid, CompoundNamespace::Cid(), HashMap::new()))
    }).map_err(to_pyerr)?;
    let syns = result.first().map(|i| i.synonym.clone()).unwrap_or_default();
    let _ = self.synonyms_cache.set(syns.clone());
    Ok(syns)
}
```

### 4.5 Add `sids` and `aids` properties
Same pattern as synonyms, using `get_sids()` / `get_aids()`. Return `Vec<u32>` from `info.sids` / `info.aids`.

---

## Phase 5: Python-side legacy compatibility

### 5.1 Add `to_series()` and `compounds_to_frame()` as free functions
**File**: `pubchemrs_pyo3/python/pubchemrs/__init__.py`

```python
def compound_to_series(compound, properties=None):
    import pandas as pd
    return pd.Series(compound.to_dict(properties))

def compounds_to_frame(compounds, properties=None):
    import pandas as pd
    if not isinstance(compounds, list):
        compounds = [compounds]
    return pd.DataFrame.from_records([c.to_dict(properties) for c in compounds], index="cid")
```

### 5.2 Update legacy module
**File**: `pubchemrs_pyo3/python/pubchemrs/legacy/__init__.py`

- Re-export Rust `Compound` (from `pubchemrs._pubchemrs`) as `Compound`
- Re-export Rust `Atom`, `Bond` (from `pubchemrs._pubchemrs`)
- Keep `BondType`, `CoordinateType`, `CompoundIdType` as Python IntEnums (or create Rust equivalents later)
- Keep `ELEMENTS` constant
- Keep `get_compounds()` function, updated to return Rust Compound directly
- Keep `compounds_to_frame()`, `memoized_property`, `deprecated` for backward compat

### 5.3 Slim down `legacy/compound.py`
Remove the `Compound` class (800+ lines). Keep:
- `ELEMENTS` dict
- `BondType`, `CoordinateType`, `CompoundIdType` enums
- `get_compounds()` function (updated to return Rust Compound)
- `compounds_to_frame()` function
- `_get_compounds_via_rust()` helper (simplified - no longer wraps in legacy Compound)
- `memoized_property`, `deprecated` decorators (for backward compat)

---

## Phase 6: Update tests

### 6.1 Add Rust Compound tests
**File**: `pubchemrs_pyo3/python/tests/test_compound.py`

- `test_from_cid` — `Compound.from_cid(2244)` returns Compound with cid 2244
- `test_from_cid_not_found` — raises exception for invalid CID
- `test_synonyms` — returns non-empty list, contains expected synonym
- `test_sids` — returns non-empty list of ints
- `test_aids` — returns non-empty list of ints

### 6.2 Migrate legacy tests
**File**: `pubchemrs_pyo3/python/tests/legacy/test_compound.py`

Update `conftest.py` fixtures to use Rust `Compound.from_cid()` instead of legacy. All existing assertions should pass since Rust Compound exposes the same properties.

---

## Files to modify (summary)

| File | Changes |
|------|---------|
| `pubchemrs_struct/src/response/information_list.rs` | Custom deserializer, SID→sids Vec, add AIDs |
| `pubchemrs_tokio/src/api.rs` | Add `get_sids()`, `get_aids()` |
| `pubchemrs_pyo3/src/client.rs` | Add sync/async `get_sids`, `get_aids` |
| `pubchemrs_pyo3/src/compound.rs` | Add cache fields, `from_cid()`, `synonyms`, `sids`, `aids`, global runtime |
| `pubchemrs_pyo3/python/pubchemrs/__init__.py` | Add `compound_to_series()`, `compounds_to_frame()` |
| `pubchemrs_pyo3/python/pubchemrs/legacy/__init__.py` | Re-export Rust types |
| `pubchemrs_pyo3/python/pubchemrs/legacy/compound.py` | Remove Compound class, keep helpers |
| `pubchemrs_pyo3/python/tests/test_compound.py` | Add from_cid, synonyms, sids, aids tests |
| `pubchemrs_pyo3/python/tests/legacy/test_compound.py` | Update fixtures to use Rust Compound |

## Verification

```bash
# Rust build & tests
cargo test
cargo clippy

# Python build & tests
cd pubchemrs_pyo3 && uv run --dev maturin develop
cd pubchemrs_pyo3 && uv run --dev pytest python/tests/ -v

# Manual verification
cd pubchemrs_pyo3 && uv run python -c "
from pubchemrs import Compound
c = Compound.from_cid(2244)
print(f'cid: {c.cid}')
print(f'synonyms: {c.synonyms[:3]}')
print(f'sids count: {len(c.sids)}')
print(f'aids count: {len(c.aids)}')
"

# Legacy compat verification
cd pubchemrs_pyo3 && uv run python -c "
from pubchemrs.legacy import Compound, get_compounds
c = Compound.from_cid(2244)
print(type(c))  # should be pubchemrs._pubchemrs.Compound
cs = get_compounds(2244)
print(type(cs[0]))  # same Rust type
"
```

## Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| GIL deadlock in `block_on()` | HIGH | Use `py.allow_threads()` for all API calls |
| SID deserializer (single vs array) | MEDIUM | Custom deserializer with thorough tests |
| Legacy test breakage | MEDIUM | Run full test suite after each phase |
