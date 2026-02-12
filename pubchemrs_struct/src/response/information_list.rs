/// A list returned in the `InformationList` field of a PubChem response.
///
/// May contain either source names or detailed information records.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum PubChemInformationList {
    /// List of data source names.
    SourceName(Vec<String>),
    /// List of information records (synonyms, cross-references, etc.).
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

/// A single information record from a PubChem response.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct PubChemInformation {
    /// Compound ID, if present.
    #[serde(rename = "CID", default)]
    pub cid: Option<u32>,
    /// Substance ID, if present.
    #[serde(rename = "SID", default)]
    pub sid: Option<u32>,
    /// List of synonyms for the compound/substance.
    #[serde(rename = "Synonym", default)]
    pub synonym: Vec<String>,
}
