/// Represents a bond between two atoms in the raw PubChem API response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct BondInner {
    /// ID of first atom
    pub aid1: Vec<u32>,
    /// ID of second atom
    pub aid2: Vec<u32>,
    /// Bond order ("single", "double", "triple", etc.)
    pub order: Vec<u32>,
}
