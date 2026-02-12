//! Operations available for the substance domain.

use std::str::FromStr;

use super::xrefs::XRefs;

/// Operations available for the substance domain.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum SubstanceOperationSpecification {
    /// Retrieve the full substance record (API value: `record`). This is the default.
    Record(),
    /// Retrieve substance synonyms (API value: `synonyms`)
    Synonyms(),
    /// Retrieve substance IDs (API value: `sids`)
    Sids(),
    /// Retrieve associated compound IDs (API value: `cids`)
    Cids(),
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids(),
    /// Retrieve assay summary (API value: `assaysummary`)
    AssaySummary(),
    /// Retrieve classification hierarchy (API value: `classification`)
    Classification(),
    /// Retrieve cross-references (API path: `xrefs/<types>`)
    XRefs(XRefs),
    /// Retrieve substance description (API value: `description`)
    Description(),
}

impl std::fmt::Display for SubstanceOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Record() => write!(f, "record"),
            Self::Synonyms() => write!(f, "synonyms"),
            Self::Sids() => write!(f, "sids"),
            Self::Cids() => write!(f, "cids"),
            Self::Aids() => write!(f, "aids"),
            Self::AssaySummary() => write!(f, "assaysummary"),
            Self::Classification() => write!(f, "classification"),
            Self::XRefs(x) => write!(f, "xrefs/{}", x),
            Self::Description() => write!(f, "description"),
        }
    }
}

impl Default for SubstanceOperationSpecification {
    fn default() -> Self {
        Self::Record()
    }
}

impl FromStr for SubstanceOperationSpecification {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("xrefs/") {
            let inner = s.trim_start_matches("xrefs/");
            Self::XRefs(XRefs::from_str(inner)?)
        } else {
            match s {
                "record" => Self::Record(),
                "synonyms" => Self::Synonyms(),
                "sids" => Self::Sids(),
                "aids" => Self::Aids(),
                "cids" => Self::Cids(),
                "assaysummary" => Self::AssaySummary(),
                "classification" => Self::Classification(),
                "description" => Self::Description(),
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        assert_eq!(
            SubstanceOperationSpecification::from_str("record").unwrap(),
            SubstanceOperationSpecification::Record()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("synonyms").unwrap(),
            SubstanceOperationSpecification::Synonyms()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("sids").unwrap(),
            SubstanceOperationSpecification::Sids()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("cids").unwrap(),
            SubstanceOperationSpecification::Cids()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("aids").unwrap(),
            SubstanceOperationSpecification::Aids()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("assaysummary").unwrap(),
            SubstanceOperationSpecification::AssaySummary()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("classification").unwrap(),
            SubstanceOperationSpecification::Classification()
        );
        assert_eq!(
            SubstanceOperationSpecification::from_str("description").unwrap(),
            SubstanceOperationSpecification::Description()
        );
    }

    #[test]
    fn test_parse_xrefs() {
        assert_eq!(
            SubstanceOperationSpecification::from_str("xrefs/RegistryID").unwrap(),
            SubstanceOperationSpecification::XRefs(XRefs::from_str("RegistryID").unwrap())
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            SubstanceOperationSpecification::default(),
            SubstanceOperationSpecification::Record()
        );
    }
}
