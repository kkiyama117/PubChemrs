/// Raw bond data from a PubChem compound record.
///
/// Contains parallel arrays: each index `i` describes a bond
/// between `aid1[i]` and `aid2[i]` with bond order `order[i]`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct BondInner {
    /// First atom IDs for each bond.
    pub aid1: Vec<u32>,
    /// Second atom IDs for each bond.
    pub aid2: Vec<u32>,
    /// Bond order values (1=single, 2=double, 3=triple, etc.).
    pub order: Vec<u32>,
}
