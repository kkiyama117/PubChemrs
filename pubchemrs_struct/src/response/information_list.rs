#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum PubChemInformationList {
    SourceName(Vec<String>),
    Information(Vec<PubChemInformation>),
}

impl PubChemInformationList {
    /// Utility function to get first information.
    pub fn get_information(&self) -> Option<PubChemInformation> {
        if let PubChemInformationList::Information(info) = self {
            info.first().cloned()
        } else {
            None
        }
    }

    /// Utility function to get first information list.
    pub fn get_information_list(self) -> Vec<PubChemInformation> {
        if let PubChemInformationList::Information(info) = self {
            info
        } else {
            vec![]
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct PubChemInformation {
    #[serde(rename = "CID", default)]
    pub cid: Option<u32>,
    #[serde(rename = "SID", default)]
    pub sid: Option<u32>,
    #[serde(rename = "Synonym", default)]
    pub synonym: Vec<String>,
}
