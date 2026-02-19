//! Structs of API response. Almost all of them are structure of response to parse with serde.
//!
//! API returns some information as array like `atoms`, only contain the list of id and element as number,
//! and we get it as `inner` record and convert it into better struct to use.
//! Recommend to transform structs below to useful structs with `into()` or `try_into()` when you use.

/// Raw compound record types from the PubChem API.
pub mod compound;
/// Information list response types (synonyms, source names, etc.).
pub mod information_list;

pub use self::compound::{Compound, Compounds};
pub use self::information_list::*;

/// Root response envelope from the PubChem PUG REST API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PubChemResponse {
    /// Full compound records (`PC_Compounds`).
    #[serde(rename = "PC_Compounds")]
    Compounds(Compounds),
    /// Compound property table (not yet fully typed).
    // TODO: Implement
    CompoundProperties(serde_json::Value),
    /// Information list (synonyms, source names, etc.).
    InformationList(PubChemInformationList),
    /// Async waiting response with a ListKey for polling.
    Waiting(PubChemWaiting),
    /// API error / fault response.
    Fault(PubChemFault),
    /// Unrecognized response shape.
    Unknown(serde_json::Value),
}

/// Async waiting response returned by PubChem for long-running queries (e.g. formula search).
///
/// Contains a `ListKey` that must be polled until results are ready.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PubChemWaiting {
    /// List key identifier for polling the async result.
    #[serde(rename = "ListKey")]
    pub list_key: u64,
}

/// API fault/error response returned by PubChem when a request fails.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct PubChemFault {
    /// Machine-readable error code (e.g. `"PUGREST.BadRequest"`).
    #[serde(rename = "Code")]
    pub code: String,
    /// Human-readable error message.
    #[serde(rename = "Message")]
    pub message: String,
    /// Additional detail strings, if any.
    #[serde(rename = "Details", default)]
    pub details: Vec<String>,
}
