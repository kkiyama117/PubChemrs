/// Represents the raw atom data from a PubChem API response.
/// Contains atom IDs, element numbers, and optional charge information.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct AtomInner {
    pub aid: Vec<u32>,
    pub element: Vec<u32>,
    pub charge: Option<Vec<ChargeInner>>,
}

/// Represents a charge on a specific atom in the response data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ChargeInner {
    pub aid: u32,
    pub value: i32,
}
