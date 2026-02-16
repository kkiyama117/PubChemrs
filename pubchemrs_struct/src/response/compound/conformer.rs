/// A single conformer with atom coordinates from a PubChem compound record.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct ConformerInner {
    /// Bond display style annotations, if present.
    #[serde(default)]
    pub style: Option<ConformerInnerStyle>,
    /// X coordinates for each atom.
    pub x: Vec<f32>,
    /// Y coordinates for each atom.
    pub y: Vec<f32>,
    /// Z coordinates for each atom (absent for 2D structures).
    pub z: Option<Vec<f32>>,
}

/// Bond style annotations within a conformer (e.g. wedge/dash for stereo bonds).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct ConformerInnerStyle {
    /// First atom IDs for each styled bond.
    pub aid1: Vec<u32>,
    /// Second atom IDs for each styled bond.
    pub aid2: Vec<u32>,
    /// Style annotation values for each bond.
    pub annotation: Vec<u32>,
}
