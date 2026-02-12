//! Operations available for the compound domain.

use std::str::FromStr;

use super::property::CompoundProperty;
use super::xrefs::XRefs;

/// Operations available for the compound domain.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CompoundOperationSpecification {
    /// Retrieve the full compound record (API value: `record`). This is the default.
    Record(),
    /// Retrieve specific compound properties (API path: `property/<tags>`)
    Property(CompoundProperty),
    /// Retrieve compound synonyms (API value: `synonyms`)
    Synonyms(),
    /// Retrieve associated substance IDs (API value: `sids`)
    Sids(),
    /// Retrieve compound IDs (API value: `cids`)
    Cids(),
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids(),
    /// Retrieve assay summary (API value: `assaysummary`)
    AssaySummary(),
    /// Retrieve classification hierarchy (API value: `classification`)
    Classification(),
    /// Retrieve cross-references (API path: `xrefs/<types>`)
    XRefs(XRefs),
    /// Retrieve compound description (API value: `description`)
    Description(),
    /// Retrieve 3D conformers (API value: `conformers`)
    Conformers(),
    /// No operation, used for source searches.
    None(),
}

impl std::fmt::Display for CompoundOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Record() => write!(f, "record"),
            Self::Property(p) => write!(f, "property/{}", p),
            Self::Synonyms() => write!(f, "synonyms"),
            Self::Sids() => write!(f, "sids"),
            Self::Cids() => write!(f, "cids"),
            Self::Aids() => write!(f, "aids"),
            Self::AssaySummary() => write!(f, "assaysummary"),
            Self::Classification() => write!(f, "classification"),
            Self::XRefs(x) => write!(f, "xrefs/{}", x),
            Self::Description() => write!(f, "description"),
            Self::Conformers() => write!(f, "conformers"),
            Self::None() => write!(f, ""),
        }
    }
}

impl Default for CompoundOperationSpecification {
    fn default() -> Self {
        Self::Record()
    }
}

impl FromStr for CompoundOperationSpecification {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("xrefs/") {
            let inner = s.trim_start_matches("xrefs/");
            Self::XRefs(XRefs::from_str(inner)?)
        } else if s.starts_with("property/") {
            let inner = s.trim_start_matches("property/");
            Self::Property(CompoundProperty::from_str(inner)?)
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
                "conformers" => Self::Conformers(),
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
            CompoundOperationSpecification::from_str("record").unwrap(),
            CompoundOperationSpecification::Record()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("synonyms").unwrap(),
            CompoundOperationSpecification::Synonyms()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("sids").unwrap(),
            CompoundOperationSpecification::Sids()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("cids").unwrap(),
            CompoundOperationSpecification::Cids()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("aids").unwrap(),
            CompoundOperationSpecification::Aids()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("assaysummary").unwrap(),
            CompoundOperationSpecification::AssaySummary()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("classification").unwrap(),
            CompoundOperationSpecification::Classification()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("description").unwrap(),
            CompoundOperationSpecification::Description()
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("conformers").unwrap(),
            CompoundOperationSpecification::Conformers()
        );
    }

    #[test]
    fn test_parse_property() {
        assert_eq!(
            CompoundOperationSpecification::from_str("property/MolecularFormula").unwrap(),
            CompoundOperationSpecification::Property(
                CompoundProperty::from_str("MolecularFormula").unwrap()
            )
        );
        assert_eq!(
            CompoundOperationSpecification::from_str("property/MolecularWeight,IUPACName").unwrap(),
            CompoundOperationSpecification::Property(
                CompoundProperty::from_str("MolecularWeight,IUPACName").unwrap()
            )
        );
    }

    #[test]
    fn test_parse_xrefs() {
        assert_eq!(
            CompoundOperationSpecification::from_str("xrefs/resigtryid").unwrap(),
            CompoundOperationSpecification::XRefs(XRefs::from_str("resigtryid").unwrap())
        );
    }

    #[test]
    fn test_parse_invalid() {
        assert!(CompoundOperationSpecification::from_str("invalid").is_err());
        assert!(CompoundOperationSpecification::from_str("RECORD").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            CompoundOperationSpecification::Record().to_string(),
            "record"
        );
        assert_eq!(
            CompoundOperationSpecification::Synonyms().to_string(),
            "synonyms"
        );
        assert_eq!(
            CompoundOperationSpecification::Property(
                CompoundProperty::from_str("MolecularFormula").unwrap()
            )
            .to_string(),
            "property/MolecularFormula"
        );
        assert_eq!(
            CompoundOperationSpecification::XRefs(XRefs::from_str("resigtryid").unwrap())
                .to_string(),
            "xrefs/resigtryid"
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            CompoundOperationSpecification::default(),
            CompoundOperationSpecification::Record()
        );
    }
}
