//! Structs of API response. Almost all of them are structure of response to parse with serde.
//!
//! API returns some information as array like `atoms`, only contain the list of id and element as number,
//! and we get it as `inner` record and convert it into better struct to use.
//! Recommend to transform structs below to useful structs with `into()` or `try_into()` when you use.

pub mod compound;
pub mod information_list;

pub use self::compound::{Compound, Compounds};
pub use self::information_list::*;

/// Root Response of PubChem API
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PubChemResponse {
    #[serde(rename = "PC_Compounds")]
    Compounds(Compounds),
    // TODO: Implement
    CompoundProperties(serde_json::Value),
    InformationList(PubChemInformationList),
    Fault(PubChemFault),
    /// Maybe this is not implemented
    Unknown(serde_json::Value),
}

/// API Fault/Error response from PubChem
/// TODO: Implement and Fix
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct PubChemFault {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Details", default)]
    pub details: Vec<String>,
}
