use pyo3::prelude::*;

mod client;
mod error;

/// Serialize a Rust Compound to a JSON string matching the PubChem API format.
///
/// This is used by the legacy compatibility layer to convert Rust Compound objects
/// back into the dict-based record format expected by the pubchempy-compatible API.
#[pyfunction]
fn compound_to_json(compound: &pubchemrs_struct::response::Compound) -> PyResult<String> {
    serde_json::to_string(compound)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Native extension module for the pubchemrs Python package.
///
/// This module is not intended to be imported directly. Use `import pubchemrs` instead.
#[pymodule]
fn _pubchemrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register custom exceptions
    error::register_exceptions(m)?;

    // Register client class
    m.add_class::<client::PyPubChemClient>()?;

    // Re-export key types from pubchemrs_struct
    m.add_class::<pubchemrs_struct::properties::CompoundProperties>()?;
    m.add_class::<pubchemrs_struct::response::Compound>()?;
    m.add_class::<pubchemrs_struct::response::PubChemInformation>()?;

    // Register struct types
    m.add_class::<pubchemrs_struct::structs::Atom>()?;
    m.add_class::<pubchemrs_struct::structs::Element>()?;

    // Register Rust enum types (with eq_int for IntEnum interop)
    m.add_class::<pubchemrs_struct::structs::CompoundIdType>()?;
    m.add_class::<pubchemrs_struct::structs::BondType>()?;
    m.add_class::<pubchemrs_struct::structs::ResponseCoordinateType>()?;

    // Register utility functions
    m.add_function(wrap_pyfunction!(compound_to_json, m)?)?;

    Ok(())
}
