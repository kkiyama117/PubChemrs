use super::conformer::ConformerInner;

/// Raw coordinate set from a PubChem compound record.
///
/// Maps atom IDs to one or more conformers containing their spatial coordinates.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct CoordsInner {
    /// Atom IDs that these coordinates apply to.
    pub aid: Vec<u32>,
    /// Conformer data with x/y/z positions.
    pub conformers: Vec<ConformerInner>,
    /// Coordinate type flags (2D, 3D, units, etc.).
    #[serde(rename = "type")]
    _type: Vec<u32>,
}
