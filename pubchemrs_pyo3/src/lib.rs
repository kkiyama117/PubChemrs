use pyo3::prelude::*;

mod client;
mod error;

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

    Ok(())
}
