#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ConformerInner {
    #[serde(default)]
    pub style: Option<ConformerInnerStyle>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Option<Vec<f32>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ConformerInnerStyle {
    pub aid1: Vec<u32>,
    pub aid2: Vec<u32>,
    pub annotation: Vec<u32>,
}
