# Error Handling

## Error Types

pubchemrs2 uses a two-layer error system:

- **`pubchemrs_struct::error::PubChemError`** — Input validation and parsing errors (no HTTP dependency)
- **`pubchemrs_tokio::error::Error`** — HTTP client errors that wrap `PubChemError`

### `pubchemrs_tokio::error::Error`

| Variant | Description | Source |
|---------|-------------|--------|
| `Http` | Network or connection errors | `reqwest::Error` |
| `HttpStatus` | Non-success HTTP status with response body | HTTP response |
| `ApiFault` | PubChem API fault response (code + message) | API `{"Fault": {...}}` response |
| `Json` | JSON deserialization error | `serde_json::Error` |
| `PubChem` | Input validation or parse errors | `pubchemrs_struct::error::PubChemError` |

### `pubchemrs_struct::error::PubChemError`

| Variant | Description |
|---------|-------------|
| `InvalidInput` | Invalid input parameters (e.g. empty identifiers) |
| `ParseResponseError` | Failed to parse API response into expected type |
| `ParseEnum` | Failed to parse string into enum variant |
| `Unknown` | Unclassified error |

## Pattern Matching

```rust,no_run
use pubchemrs_tokio::error::Error;
use pubchemrs_tokio::CompoundQuery;

# async fn example() {
let result = CompoundQuery::with_name("nonexistent_compound_xyz")
    .molecular_formula()
    .await;

match result {
    Ok(formula) => println!("Formula: {formula:?}"),
    Err(Error::ApiFault { code, message }) => {
        eprintln!("API error {code}: {message}");
    }
    Err(Error::HttpStatus { status, body }) => {
        eprintln!("HTTP {status}: {body}");
    }
    Err(Error::Http(e)) => {
        eprintln!("Network error: {e}");
    }
    Err(Error::Json(e)) => {
        eprintln!("Parse error: {e}");
    }
    Err(Error::PubChem(e)) => {
        eprintln!("Input/validation error: {e}");
    }
}
# }
```

## Retry Behavior

`PubChemClient` automatically retries requests on transient server errors:

- **Retryable status codes**: `429 Too Many Requests`, `503 Service Unavailable`, `504 Gateway Timeout`
- **Backoff**: Linear backoff (`retry_delay * attempt_number`)
- **Max attempts**: `1 + max_retries` (default: 4 total attempts)

Non-retryable errors (4xx other than 429, 500, etc.) are returned immediately. If the response contains a `{"Fault": {...}}` JSON body, it is parsed into an `ApiFault` error.

### Customizing Retry

```rust,no_run
use pubchemrs_tokio::{PubChemClient, ClientConfig};
use std::time::Duration;

let client = PubChemClient::new(ClientConfig {
    timeout: Duration::from_secs(60),
    max_retries: 5,
    retry_delay: Duration::from_secs(1),
}).unwrap();
```

## Environment Variables

These environment variables affect error behavior in `pubchemrs_struct`:

| Variable | Effect |
|----------|--------|
| `PUBCHEM_PANIC_ON_ERR=1` | Panic on `pubchemrs_struct` errors instead of returning `Err` |
| `PUBCHEM_BACKTRACE_IN_ERR=1` | Include backtrace in error messages |

These are evaluated once at first use (via `LazyLock`) and cannot be changed at runtime.

## Common Error Scenarios

### Invalid compound name

The PubChem API returns an `ApiFault` with code `PUGREST.NotFound` when a compound cannot be found.

### Rate limiting

PubChem enforces rate limits. When you receive `429 Too Many Requests`, the client automatically retries with backoff. If you consistently hit rate limits, increase `retry_delay` or reduce request frequency.

### SMILES/InChI parse errors

Invalid SMILES or InChI strings result in an `ApiFault` from the PubChem API. These are not retried.

### Network timeouts

The default timeout is 30 seconds. For slow connections or large batch queries, increase `timeout` in `ClientConfig`.
