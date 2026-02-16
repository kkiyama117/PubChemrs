/// Raw atom data from a PubChem compound record.
///
/// Contains parallel arrays of atom IDs and element numbers,
/// plus optional per-atom charge information.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct AtomInner {
    /// Atom IDs (1-based, unique within the compound).
    pub aid: Vec<u32>,
    /// Element atomic numbers, parallel to `aid`.
    pub element: Vec<u32>,
    /// Per-atom formal charges, if any atoms are charged.
    pub charge: Option<Vec<ChargeInner>>,
}

/// A formal charge on a specific atom.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct ChargeInner {
    /// Atom ID that carries the charge.
    pub aid: u32,
    /// Formal charge value.
    pub value: i32,
}
