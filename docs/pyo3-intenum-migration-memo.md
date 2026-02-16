# pyo3 Enum vs Python IntEnum: Migration Memo

## Goal

Replace Python `IntEnum` classes in `pubchemrs_pyo3/python/pubchemrs/legacy/compound.py`
with Rust `#[pyclass]` enums from `pubchemrs_struct`.

## Target Types

| Python IntEnum | Rust enum | File |
|---|---|---|
| `CompoundIdType` | `pubchemrs_struct::structs::CompoundIdType` | `structs/classification.rs` |
| `BondType` | `pubchemrs_struct::structs::BondType` | `structs/bond.rs` |
| `CoordinateType` | `pubchemrs_struct::structs::Coordinate` | `structs/coordinates.rs` |

## Blockers (as of pyo3 0.28)

### 1. Hash Incompatibility

pyo3's `#[pyclass(hash)]` uses Rust's `Hash` trait, which produces different hash values
than Python's `hash(int)`. This breaks set/dict operations:

```python
# pyo3 enum with eq_int + frozen + hash
BondType.Single == 1        # True  (eq_int works)
1 == BondType.Single        # True  (eq_int works both ways)
hash(BondType.Single)       # 4952851536318644461  (Rust Hash)
hash(1)                     # 1  (Python int hash)
{BondType.Single} == {1}    # False! (hash mismatch)
```

Python's `set.__eq__` and `dict` rely on both `__eq__` AND `__hash__` matching.
Since pyo3 uses Rust's `Hash` implementation, hash values never match Python ints.

### 2. Variant Naming (PascalCase vs SCREAMING_CASE)

Rust enums use `PascalCase` (e.g., `Single`), but legacy Python API uses
`SCREAMING_CASE` (e.g., `SINGLE`). Solutions attempted:

- `#[pyo3(name = "SINGLE")]` on each variant: **does not work with `cfg_attr`**
  because `#[pyo3(...)]` is a proc-macro attribute that requires `#[pyclass]`
  to be directly present (not behind `cfg_attr`).
- `#[pyclass(rename_all = "SCREAMING_SNAKE_CASE")]`: **not supported** on
  `#[pyclass]` as of pyo3 0.28 (only available for `#[derive(FromPyObject)]`).

### 3. `frozen` Requirement

`#[pyclass(hash)]` requires `#[pyclass(frozen)]`, which prevents mutation.
This is fine for enums but adds complexity to the attribute list.

## Migration Steps (when blockers are resolved)

If pyo3 adds `rename_all` support for `#[pyclass]` and integer-compatible hashing,
the migration steps would be:

1. **Rust side** — Add pyclass options to the enum:
   ```rust
   #[cfg_attr(feature = "pyo3", pyo3::pyclass(
       eq, eq_int, frozen, hash, rename_all = "SCREAMING_SNAKE_CASE", from_py_object
   ))]
   pub enum BondType { ... }
   ```

2. **pyo3 module** — Register the type in `lib.rs`:
   ```rust
   m.add_class::<pubchemrs_struct::structs::BondType>()?;
   ```

3. **Python side** — Replace the IntEnum class with an import:
   ```python
   from pubchemrs._pubchemrs import BondType
   ```

4. **Test** — Verify set/dict operations and integer comparisons work.

## Current Decision

Keep Python `IntEnum` classes as-is. The pyo3 enum type is not a drop-in replacement
for `IntEnum` due to hash and naming incompatibilities. Revisit when pyo3 adds
`rename_all` for `#[pyclass]` and integer-compatible `__hash__`.

## References

- [pyo3 rename_all discussion](https://github.com/PyO3/pyo3/discussions/2455)
- [pyo3 rename_all for FromPyObject PR](https://github.com/PyO3/pyo3/pull/4941)
- [pyo3 hash discussion](https://github.com/PyO3/pyo3/discussions/3557)
