//! Operations available for the assay domain.

use std::str::FromStr;

/// Operations available for the assay domain.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayOperationSpecification {
    /// Retrieve the full assay record (API value: `record`). This is the default.
    Record(),
    /// Retrieve concise assay data (API value: `concise`)
    Concise(),
    /// Retrieve assay IDs (API value: `aids`)
    Aids(),
    /// Retrieve associated compound IDs (API value: `cids`)
    Cids(),
    /// Retrieve associated substance IDs (API value: `sids`)
    Sids(),
    /// Retrieve assay description (API value: `description`)
    Description(),
    /// Retrieve assay targets by type (API path: `targets/<type>`)
    Targets(AssayOperationTargetType),
    /// Retrieve dose-response data (API value: `doseresponse/sid`)
    DoseResponse(),
    /// Retrieve assay summary (API value: `summary`)
    Summary(),
    /// Retrieve classification hierarchy (API value: `classification`)
    Classification(),
}

impl std::fmt::Display for AssayOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Record() => write!(f, "record"),
            Self::Concise() => write!(f, "concise"),
            Self::Aids() => write!(f, "aids"),
            Self::Cids() => write!(f, "cids"),
            Self::Sids() => write!(f, "sids"),
            Self::Description() => write!(f, "description"),
            Self::Targets(t) => write!(f, "targets/{}", t),
            Self::DoseResponse() => write!(f, "doseresponse/sid"),
            Self::Summary() => write!(f, "summary"),
            Self::Classification() => write!(f, "classification"),
        }
    }
}

impl Default for AssayOperationSpecification {
    fn default() -> Self {
        Self::Record()
    }
}

impl FromStr for AssayOperationSpecification {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("targets/") {
            let inner = s.trim_start_matches("targets/");
            Self::Targets(AssayOperationTargetType::from_str(inner)?)
        } else {
            match s {
                "record" => Self::Record(),
                "concise" => Self::Concise(),
                "aids" => Self::Aids(),
                "cids" => Self::Cids(),
                "sids" => Self::Sids(),
                "description" => Self::Description(),
                "doseresponse/sid" => Self::DoseResponse(),
                "summary" => Self::Summary(),
                "classification" => Self::Classification(),
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        })
    }
}

/// Target type for assay target retrieval operations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayOperationTargetType {
    /// Protein GI number (API value: `proteingi`)
    #[default]
    ProteinGI,
    /// Protein name (API value: `proteinname`)
    ProteinName,
    /// NCBI gene ID (API value: `geneid`)
    GeneID,
    /// Gene symbol (API value: `genesymbol`)
    GeneSymbol,
}

impl_enum_str!(AssayOperationTargetType {
    ProteinGI => "proteingi",
    ProteinName => "proteinname",
    GeneID => "geneid",
    GeneSymbol => "genesymbol",
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        assert_eq!(
            AssayOperationSpecification::from_str("record").unwrap(),
            AssayOperationSpecification::Record()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("concise").unwrap(),
            AssayOperationSpecification::Concise()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("aids").unwrap(),
            AssayOperationSpecification::Aids()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("cids").unwrap(),
            AssayOperationSpecification::Cids()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("sids").unwrap(),
            AssayOperationSpecification::Sids()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("description").unwrap(),
            AssayOperationSpecification::Description()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("doseresponse/sid").unwrap(),
            AssayOperationSpecification::DoseResponse()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("summary").unwrap(),
            AssayOperationSpecification::Summary()
        );
        assert_eq!(
            AssayOperationSpecification::from_str("classification").unwrap(),
            AssayOperationSpecification::Classification()
        );
    }

    #[test]
    fn test_parse_targets() {
        assert_eq!(
            AssayOperationSpecification::from_str("targets/proteingi").unwrap(),
            AssayOperationSpecification::Targets(AssayOperationTargetType::ProteinGI)
        );
        assert_eq!(
            AssayOperationSpecification::from_str("targets/proteinname").unwrap(),
            AssayOperationSpecification::Targets(AssayOperationTargetType::ProteinName)
        );
        assert_eq!(
            AssayOperationSpecification::from_str("targets/geneid").unwrap(),
            AssayOperationSpecification::Targets(AssayOperationTargetType::GeneID)
        );
        assert_eq!(
            AssayOperationSpecification::from_str("targets/genesymbol").unwrap(),
            AssayOperationSpecification::Targets(AssayOperationTargetType::GeneSymbol)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            AssayOperationSpecification::default(),
            AssayOperationSpecification::Record()
        );
    }

    #[test]
    fn test_target_type_parse() {
        assert_eq!(
            AssayOperationTargetType::from_str("proteingi").unwrap(),
            AssayOperationTargetType::ProteinGI
        );
        assert_eq!(
            AssayOperationTargetType::from_str("proteinname").unwrap(),
            AssayOperationTargetType::ProteinName
        );
        assert_eq!(
            AssayOperationTargetType::from_str("geneid").unwrap(),
            AssayOperationTargetType::GeneID
        );
        assert_eq!(
            AssayOperationTargetType::from_str("genesymbol").unwrap(),
            AssayOperationTargetType::GeneSymbol
        );
    }

    #[test]
    fn test_target_type_display() {
        assert_eq!(AssayOperationTargetType::ProteinGI.to_string(), "proteingi");
        assert_eq!(
            AssayOperationTargetType::ProteinName.to_string(),
            "proteinname"
        );
        assert_eq!(AssayOperationTargetType::GeneID.to_string(), "geneid");
        assert_eq!(
            AssayOperationTargetType::GeneSymbol.to_string(),
            "genesymbol"
        );
    }

    #[test]
    fn test_target_type_default() {
        assert_eq!(
            AssayOperationTargetType::default(),
            AssayOperationTargetType::ProteinGI
        );
    }
}
