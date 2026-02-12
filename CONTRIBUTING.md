# Contributing to pubchemrs2

Thank you for considering contributing! This document explains how to set up the project for development and submit changes.

## Prerequisites

- **Rust 1.85.0+** (Edition 2024)
- **Git**
- (Optional) Python 3.9+ and [maturin](https://www.maturin.rs/) for Python bindings

## Development Setup

```bash
git clone https://github.com/kkiyama117/PubChemrs.git
cd PubChemrs
cargo build
```

## Build & Test Commands

```bash
# Build workspace
cargo build

# Run all unit tests (offline, no network)
cargo test

# Run tests for a single crate
cargo test -p pubchemrs_struct
cargo test -p pubchemrs_tokio

# Run integration tests (requires network access to PubChem API)
cargo test -p pubchemrs_tokio -- --ignored

# Build with Python bindings
cargo build -p pubchemrs_struct --features pyo3

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy` and fix all warnings.
- Follow existing patterns in the codebase.
- Keep functions small (<50 lines) and files focused.

## Pull Request Workflow

1. Fork the repository.
2. Create a feature branch from `develop`:
   ```bash
   git checkout -b feat/your-feature develop
   ```
3. Make your changes with clear, focused commits.
4. Ensure all checks pass:
   ```bash
   cargo fmt --check && cargo clippy && cargo test
   ```
5. Push your branch and open a Pull Request against `develop`.

### Commit Message Format

```
<type>: <description>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `ci`

### PR Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo test` passes
- [ ] New code has unit tests
- [ ] Documentation updated if API changed

## Reporting Issues

- Use the [bug report template](https://github.com/kkiyama117/PubChemrs/issues/new?labels=bug&template=bug_report.md) for bugs.
- Use the [feature request template](https://github.com/kkiyama117/PubChemrs/issues/new?labels=enhancement&template=feature_request.md) for new ideas.
