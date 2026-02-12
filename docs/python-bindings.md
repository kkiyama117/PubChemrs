# Python Bindings

pubchemrs2 provides optional Python bindings via [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/), exposing the `pubchemrs_struct` type definitions to Python.

## Overview

The `pyo3` feature flag adds `#[pyclass]` derives to all major types in `pubchemrs_struct`, including `CompoundProperties`, response types, and request structures. This allows Python code to work with the same strongly-typed structures used in Rust.

## Prerequisites

- Python 3.9+ (`abi3-py39`)
- [maturin](https://www.maturin.rs/) (`pip install maturin`)
- Rust 1.85.0+

## Installation

### Development Build

Install directly into the current Python environment:

```bash
maturin develop --features pyo3
```

### Release Build

Build optimized wheels:

```bash
maturin build --release --features pyo3
```

The resulting wheel will be in `target/wheels/`.

### From Cargo

To build the Rust library with Python bindings enabled (without maturin):

```bash
cargo build -p pubchemrs_struct --features pyo3
```

## Supported Platforms

CI builds wheels for the following platforms:

| OS | Architectures |
|----|---------------|
| Linux | x86_64, x86, aarch64, armv7 |
| Windows | x64, x86 |

## Type Mappings

| Rust Type | Python Type |
|-----------|-------------|
| `String` | `str` |
| `u32`, `u64` | `int` |
| `f64` | `float` |
| `i32` | `int` |
| `Option<T>` | `T | None` |
| `Vec<T>` | `list[T]` |
| `bool` | `bool` |

All fields on `CompoundProperties` are exposed as read-only Python attributes via `#[pyclass(get_all)]`.

## CI Wheel Builds

Python wheels are built automatically via `.github/workflows/maturin_build.yml`. Wheels are built for all supported platforms on release tags.
