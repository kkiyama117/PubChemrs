# Plan: Implement Query Parameter (kwargs) Support in Rust Client

## Context

PubChem PUG REST API supports query parameters like `?record_type=3d` to fetch 3D compound records. The `UrlBuilder` already has a `kwargs: HashMap<String, String>` field, but it is completely ignored — `build_url_parts()` doesn't use it, and `build_request_parts()` in the client doesn't append query strings to URLs.

Currently, `Compound.from_cid(1234, record_type="3d")` falls back to legacy Python HTTP as a workaround. The goal is to make the Rust client natively support query parameters so 3D records (and other parameterized requests) work through Rust.

## Requirements

1. `kwargs` in `UrlBuilder` should be converted to URL query parameters (e.g. `?record_type=3d`)
2. The client should append these query params when making HTTP requests
3. Python `from_cid` should pass `record_type` through to Rust client instead of falling back to legacy HTTP
4. All existing tests must continue to pass
5. 3D compound tests (`test_compound3d.py`) should work via Rust client

## Files to Modify

| File | Action |
|------|--------|
| `pubchemrs_struct/src/requests/url_builder.rs` | `build_url_parts()` returns query string from kwargs |
| `pubchemrs_tokio/src/client.rs` | `build_request_parts()` appends query string to URL |
| `pubchemrs_pyo3/src/client.rs` | Accept and forward kwargs in all Python-facing methods |
| `pubchemrs_pyo3/python/pubchemrs/legacy/compound.py` | Pass kwargs to Rust client, remove legacy fallback for `record_type` |
| `pubchemrs_pyo3/python/tests/legacy/test_compound3d.py` | Update/verify 3D tests work via Rust |

## Implementation Phases

### Phase 1: `url_builder.rs` — Query String Generation

Modify `build_url_parts()` to return query parameters alongside the URL path.

**Option A** (minimal change): Return `(Vec<String>, Option<String>, Option<String>)` — path parts, POST body, query string.

**Option B** (struct-based): Return a `UrlParts` struct with fields `{ path: Vec<String>, body: Option<String>, query: Option<String> }`.

Recommendation: **Option B** — cleaner, extensible, and avoids tuple positional confusion.

```rust
/// Result of URL building
pub struct UrlParts {
    pub path_segments: Vec<String>,
    pub post_body: Option<String>,
    pub query_string: Option<String>,
}

impl UrlBuilder {
    pub fn build_url_parts(&self) -> Result<UrlParts, PubChemError> {
        // ... existing path/body logic ...

        let query_string = if self.kwargs.is_empty() {
            None
        } else {
            let qs = self.kwargs.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            Some(qs)
        };

        Ok(UrlParts { path_segments, post_body, query_string })
    }
}
```

**Note**: URL-encode keys and values to handle special characters properly. Consider using `urlencoding` crate or manual percent-encoding.

### Phase 2: `client.rs` (tokio) — Append Query String to URL

Update `build_request_parts()` to use the new `UrlParts` struct and append the query string.

```rust
fn build_request_parts(builder: &UrlBuilder) -> Result<(String, Option<String>), PubChemError> {
    let parts = builder.build_url_parts()?;
    let mut url = format!("{}/{}", BASE_URL, parts.path_segments.join("/"));

    if let Some(qs) = parts.query_string {
        url.push('?');
        url.push_str(&qs);
    }

    Ok((url, parts.post_body))
}
```

Also update all callers of `build_url_parts()` in the crate (likely `api.rs` or helper functions).

### Phase 3: `client.rs` (pyo3) — Accept kwargs in Python Methods

Currently all methods pass `HashMap::new()` as kwargs. Update to accept an optional Python `dict` and forward it.

```rust
#[pyo3(signature = (identifier, namespace="cid", **kwargs))]
fn get_compounds_sync(
    &self,
    py: Python<'_>,
    identifier: &Bound<'_, PyAny>,
    namespace: &str,
    kwargs: Option<HashMap<String, String>>,
) -> PyResult<Vec<Compound>> {
    let kw = kwargs.unwrap_or_default();
    // ... pass kw to client instead of HashMap::new() ...
}
```

Apply the same pattern to:
- `get_compounds` / `get_compounds_sync`
- `get_properties` / `get_properties_sync`
- `get_synonyms` / `get_synonyms_sync`

### Phase 4: `compound.py` — Remove Legacy Fallback

Remove the legacy HTTP fallback for `record_type` in `from_cid`:

```python
# Before (current workaround):
if kwargs:
    from pubchemrs.legacy import get_json
    results = get_json(cid, **kwargs)
    ...

# After (pass through Rust client):
results = cls._client.get_compounds_sync(cid, "cid", **kwargs)
```

This makes 3D compound fetching go through the full Rust pipeline.

### Phase 5: Tests & Verification

1. **Unit tests** (`pubchemrs_struct`):
   - Test `build_url_parts()` with kwargs produces correct query string
   - Test empty kwargs produces no query string
   - Test special character URL encoding

2. **Integration tests** (`pubchemrs_tokio`):
   - Add `#[ignore]` test fetching CID with `record_type=3d`

3. **Python tests**:
   - Verify `test_compound3d.py` passes without legacy fallback
   - May need to update test CID if 1234 doesn't have 3D data (check API)

## Risks & Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| Breaking change in `build_url_parts()` return type | HIGH | Use struct, update all callers in same PR |
| URL encoding issues with special characters | MEDIUM | Use proper percent-encoding for values |
| 3D data not available for all CIDs | LOW | Use CID known to have 3D (e.g. 2244 aspirin) |
| `**kwargs` in PyO3 may not map cleanly | MEDIUM | Use `Option<HashMap<String, String>>` with explicit signature |

## Verification Commands

```bash
# Phase 1-2: Rust builds and tests pass
cargo build
cargo test -p pubchemrs_struct
cargo test -p pubchemrs_tokio

# Phase 2: Integration test with 3D (requires network)
cargo test -p pubchemrs_tokio -- --ignored test_get_compound_3d

# Phase 3-4: Python tests pass
cd pubchemrs_pyo3 && uv run maturin develop
cd pubchemrs_pyo3 && uv run pytest python/tests/legacy/test_compound3d.py -v
cd pubchemrs_pyo3 && uv run pytest python/tests/ -v

# Format & lint
cargo fmt --check
cargo clippy
```

## Dependencies

- No new crate dependencies expected (URL encoding can be done manually for simple key=value pairs)
- If complex URL encoding needed, consider `url` crate (already likely a transitive dep of `reqwest`)
