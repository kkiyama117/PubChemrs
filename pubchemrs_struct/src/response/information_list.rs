use serde::{Deserialize, Deserializer};

/// A list returned in the `InformationList` field of a PubChem response.
///
/// May contain either source names or detailed information records.
#[derive(Clone, Debug, serde::Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
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

/// Deserialize a JSON value that may be either a single `u32` or a `Vec<u32>`.
///
/// PubChem returns SID/AID as a single integer for some endpoints (e.g. synonyms)
/// and as an array for others (e.g. compound/cid/{cid}/sids).
fn deserialize_u32_or_vec<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum SingleOrVec {
        Single(u32),
        Vec(Vec<u32>),
    }

    match SingleOrVec::deserialize(deserializer)? {
        SingleOrVec::Single(v) => Ok(vec![v]),
        SingleOrVec::Vec(v) => Ok(v),
    }
}

/// A single information record from a PubChem response.
#[derive(Clone, Debug, serde::Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all, from_py_object))]
pub struct PubChemInformation {
    /// Compound ID, if present.
    #[serde(rename = "CID", default)]
    pub cid: Option<u32>,
    /// Substance IDs. May contain a single SID or a list depending on the endpoint.
    #[serde(rename = "SID", default, deserialize_with = "deserialize_u32_or_vec")]
    pub sids: Vec<u32>,
    /// Assay IDs. May contain a single AID or a list depending on the endpoint.
    #[serde(rename = "AID", default, deserialize_with = "deserialize_u32_or_vec")]
    pub aids: Vec<u32>,
    /// List of synonyms for the compound/substance.
    #[serde(rename = "Synonym", default)]
    pub synonym: Vec<String>,
}

impl PubChemInformation {
    /// Get the first SID, if any. Useful for backward compatibility when only a single SID is expected.
    pub fn first_sid(&self) -> Option<u32> {
        self.sids.first().copied()
    }

    /// Get the first CID, if present.
    pub fn first_cid(&self) -> Option<u32> {
        self.cid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_sid() {
        let json = r#"{"CID": 2244, "SID": 12345, "Synonym": ["Aspirin"]}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.cid, Some(2244));
        assert_eq!(info.sids, vec![12345]);
        assert_eq!(info.aids, Vec::<u32>::new());
        assert_eq!(info.first_sid(), Some(12345));
    }

    #[test]
    fn test_deserialize_sid_array() {
        let json = r#"{"CID": 2244, "SID": [1234, 5678, 9012]}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.sids, vec![1234, 5678, 9012]);
        assert_eq!(info.first_sid(), Some(1234));
    }

    #[test]
    fn test_deserialize_no_sid() {
        let json = r#"{"CID": 2244, "Synonym": ["Aspirin"]}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.sids, Vec::<u32>::new());
        assert_eq!(info.first_sid(), None);
    }

    #[test]
    fn test_deserialize_aid_array() {
        let json = r#"{"CID": 2244, "AID": [100, 200, 300]}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.aids, vec![100, 200, 300]);
    }

    #[test]
    fn test_deserialize_single_aid() {
        let json = r#"{"CID": 2244, "AID": 42}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.aids, vec![42]);
    }

    #[test]
    fn test_deserialize_no_aid() {
        let json = r#"{"CID": 2244}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.aids, Vec::<u32>::new());
        assert_eq!(info.sids, Vec::<u32>::new());
    }

    #[test]
    fn test_deserialize_all_fields() {
        let json = r#"{"CID": 2244, "SID": [1, 2], "AID": [3, 4], "Synonym": ["Aspirin", "ASA"]}"#;
        let info: PubChemInformation = serde_json::from_str(json).unwrap();
        assert_eq!(info.cid, Some(2244));
        assert_eq!(info.sids, vec![1, 2]);
        assert_eq!(info.aids, vec![3, 4]);
        assert_eq!(info.synonym, vec!["Aspirin", "ASA"]);
    }
}
