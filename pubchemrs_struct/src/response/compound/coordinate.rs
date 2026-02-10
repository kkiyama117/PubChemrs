use super::conformer::ConformerInner;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct CoordsInner {
    pub aid: Vec<u32>,
    pub conformers: Vec<ConformerInner>,
    #[serde(rename = "type")]
    _type: Vec<u32>,
}
