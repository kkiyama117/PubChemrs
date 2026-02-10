//! # pubchemrs_struct
//!
//! Strongly-typed data structures for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest).
//!
//! This crate provides pure type definitions with zero runtime dependencies (only `serde` for
//! serialization). It is designed to be used alongside an HTTP client crate such as `pubchemrs_tokio`.
//!
//! ## Overview
//!
//! The PubChem PUG REST API returns compound data in a loosely-typed JSON format where some
//! numeric fields arrive as strings and all properties are optional depending on the request.
//! This crate maps those responses into idiomatic Rust types with:
//!
//! - Correct numeric types (`f64`, `u32`, `i32`) instead of `Any` or `String`
//! - `Option<T>` for all property fields (the API only returns requested properties)
//! - Automatic string-to-number coercion for fields like `MolecularWeight`
//! - Serde rename attributes matching the PubChem PascalCase JSON format
//!
//! ## Quick Start
//!
//! ```rust
//! use pubchemrs_struct::properties::{PropertyTableResponse, CompoundProperties};
//!
//! // Deserialize a PubChem PropertyTable API response
//! let json = r#"{
//!     "PropertyTable": {
//!         "Properties": [{
//!             "CID": 2244,
//!             "MolecularFormula": "C9H8O4",
//!             "MolecularWeight": "180.16",
//!             "IUPACName": "2-acetyloxybenzoic acid"
//!         }]
//!     }
//! }"#;
//!
//! let response: PropertyTableResponse = serde_json::from_str(json).unwrap();
//! let aspirin = &response.property_table.properties[0];
//!
//! assert_eq!(aspirin.cid, 2244);
//! assert_eq!(aspirin.molecular_formula.as_deref(), Some("C9H8O4"));
//! // MolecularWeight is automatically parsed from string to f64
//! assert!((aspirin.molecular_weight.unwrap() - 180.16).abs() < 0.01);
//! ```
//!
//! ## PubChem API Reference
//!
//! The PropertyTable endpoint follows this URL pattern:
//!
//! ```text
//! https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/{namespace}/{identifiers}/property/{properties}/JSON
//! ```
//!
//! For example, to fetch molecular weight and InChIKey for aspirin (CID 2244):
//!
//! ```text
//! https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/2244/property/MolecularWeight,InChIKey/JSON
//! ```
//!
//! ## Feature Flags
//!
//! - **`pyo3`** - Enables `#[pyclass]` derives for Python bindings via PyO3.

#[macro_use]
mod macros;
pub mod error;
pub mod properties;
pub mod requests;
pub mod response;
pub mod structs;
