use pyo3::prelude::*;
use pyo3::types::PyDict;

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

    // Register struct types
    m.add_class::<pubchemrs_struct::structs::Atom>()?;
    m.add_class::<pubchemrs_struct::structs::Bond>()?;
    m.add_class::<pubchemrs_struct::structs::Element>()?;

    // Register Rust enum types (with eq_int for IntEnum interop)
    m.add_class::<pubchemrs_struct::structs::CompoundIdType>()?;
    m.add_class::<pubchemrs_struct::structs::BondType>()?;
    m.add_class::<pubchemrs_struct::structs::ResponseCoordinateType>()?;

    // Expose PROPERTY_MAP as a Python dict {snake_case: api_name}
    // Derived from CompoundPropertyTag::variants()
    let property_map = PyDict::new(m.py());
    for variant in pubchemrs_struct::requests::operation::CompoundPropertyTag::variants() {
        property_map.set_item(variant.snake_case_name().as_ref(), variant.to_string())?;
    }
    m.add("PROPERTY_MAP", property_map)?;

    Ok(())
}
