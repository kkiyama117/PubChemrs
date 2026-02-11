use crate::requests::{common::UrlParts, input::Namespace};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayNamespace {
    Aid(),
    ListKey(),
    Type(AssayType),
    /// Source Name is `any valid PubChem depositor name`
    SourceAll(String),
    Target(AssayTarget),
    /// Put an argument as Activity Column Name
    Activity(String),
}

impl Display for AssayNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssayNamespace::Aid() => write!(f, "aid"),
            AssayNamespace::ListKey() => write!(f, "listkey"),
            AssayNamespace::Type(t) => write!(f, "type/{}", t),
            AssayNamespace::SourceAll(s) => write!(f, "sourceall/{}", s),
            AssayNamespace::Target(t) => write!(f, "target/{}", t),
            AssayNamespace::Activity(a) => write!(f, "activity/{}", a),
        }
    }
}

impl From<AssayNamespace> for Namespace {
    fn from(value: AssayNamespace) -> Self {
        Self::Assay(value)
    }
}

// Complex Enum does not yet support Unit Variant
impl Default for AssayNamespace {
    fn default() -> Self {
        Self::Aid()
    }
}

impl UrlParts for AssayNamespace {
    fn to_url_parts(&self) -> Vec<String> {
        match self {
            AssayNamespace::Type(id) => vec!["type".to_string(), id.to_string()],
            AssayNamespace::SourceAll(id) => vec!["sourceall".to_string(), id.to_string()],
            AssayNamespace::Target(id) => vec!["target".to_string(), id.to_string()],
            AssayNamespace::Activity(id) => vec!["activity".to_string(), id.to_string()],
            _ => vec![self.to_string()],
        }
    }
}

/// Overwrap strum::EnumString
impl FromStr for AssayNamespace {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if s.starts_with("type/") {
            let inner = s.trim_start_matches("type/");
            AssayNamespace::Type(AssayType::from_str(inner)?)
        } else if s.starts_with("sourceall/") {
            let inner = s.trim_start_matches("sourceall/");
            AssayNamespace::SourceAll(inner.into())
        } else if s.starts_with("target/") {
            let inner = s.trim_start_matches("target/");
            AssayNamespace::Target(AssayTarget::from_str(inner)?)
        } else if s.starts_with("activity/") {
            let inner = s.trim_start_matches("activity/");
            AssayNamespace::Activity(inner.into())
        } else {
            match s {
                "aid" => AssayNamespace::Aid(),
                "listkey" => AssayNamespace::ListKey(),
                // Invalid pattern
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        };
        Ok(result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayType {
    #[default]
    All,
    Confirmatory,
    Doseresponse,
    OnHold,
    Panel,
    Rnai,
    Screening,
    Summary,
    CellBased,
    Biochemical,
    Invivo,
    Invitro,
    ActiveConcentrationSpecified,
}

impl_enum_str!(AssayType {
    All => "all",
    Confirmatory => "confirmatory",
    Doseresponse => "doseresponse",
    OnHold => "onhold",
    Panel => "panel",
    Rnai => "rnai",
    Screening => "screening",
    Summary => "summary",
    CellBased => "cellbased",
    Biochemical => "biochemical",
    Invivo => "invivo",
    Invitro => "invitro",
    ActiveConcentrationSpecified => "activeconcentrationspecified",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayTarget {
    #[default]
    Gi,
    ProteinName,
    GeneID,
    GeneSymbol,
    Accession,
}

impl_enum_str!(AssayTarget {
    Gi => "gi",
    ProteinName => "proteinname",
    GeneID => "geneid",
    GeneSymbol => "genesymbol",
    Accession => "accession",
});

#[cfg(test)]
mod tests {
    use super::*;

    // AssayNamespace tests
    #[test]
    fn test_assay_namespace_parse_basic() {
        assert_eq!(
            AssayNamespace::from_str("aid").unwrap(),
            AssayNamespace::Aid()
        );
        assert_eq!(
            AssayNamespace::from_str("listkey").unwrap(),
            AssayNamespace::ListKey()
        );
    }

    #[test]
    fn test_assay_namespace_parse_type() {
        assert_eq!(
            AssayNamespace::from_str("type/all").unwrap(),
            AssayNamespace::Type(AssayType::All)
        );
        assert_eq!(
            AssayNamespace::from_str("type/confirmatory").unwrap(),
            AssayNamespace::Type(AssayType::Confirmatory)
        );
        assert_eq!(
            AssayNamespace::from_str("type/screening").unwrap(),
            AssayNamespace::Type(AssayType::Screening)
        );
        assert_eq!(
            AssayNamespace::from_str("type/cellbased").unwrap(),
            AssayNamespace::Type(AssayType::CellBased)
        );
    }

    #[test]
    fn test_assay_namespace_parse_sourceall() {
        assert_eq!(
            AssayNamespace::from_str("sourceall/PubChem").unwrap(),
            AssayNamespace::SourceAll("PubChem".to_string())
        );
        assert_eq!(
            AssayNamespace::from_str("sourceall/ChEMBL").unwrap(),
            AssayNamespace::SourceAll("ChEMBL".to_string())
        );
    }

    #[test]
    fn test_assay_namespace_parse_target() {
        assert_eq!(
            AssayNamespace::from_str("target/gi").unwrap(),
            AssayNamespace::Target(AssayTarget::Gi)
        );
        assert_eq!(
            AssayNamespace::from_str("target/proteinname").unwrap(),
            AssayNamespace::Target(AssayTarget::ProteinName)
        );
        assert_eq!(
            AssayNamespace::from_str("target/geneid").unwrap(),
            AssayNamespace::Target(AssayTarget::GeneID)
        );
        assert_eq!(
            AssayNamespace::from_str("target/genesymbol").unwrap(),
            AssayNamespace::Target(AssayTarget::GeneSymbol)
        );
        assert_eq!(
            AssayNamespace::from_str("target/accession").unwrap(),
            AssayNamespace::Target(AssayTarget::Accession)
        );
    }

    #[test]
    fn test_assay_namespace_parse_activity() {
        assert_eq!(
            AssayNamespace::from_str("activity/IC50").unwrap(),
            AssayNamespace::Activity("IC50".to_string())
        );
        assert_eq!(
            AssayNamespace::from_str("activity/EC50").unwrap(),
            AssayNamespace::Activity("EC50".to_string())
        );
    }

    #[test]
    fn test_assay_namespace_parse_invalid() {
        assert!(AssayNamespace::from_str("invalid").is_err());
        assert!(AssayNamespace::from_str("AID").is_err()); // Case sensitive
        assert!(AssayNamespace::from_str("").is_err());
        assert!(AssayNamespace::from_str("type/").is_err()); // Empty inner value
        assert!(AssayNamespace::from_str("type/invalid").is_err()); // Invalid AssayType
    }

    #[test]
    fn test_assay_namespace_display() {
        assert_eq!(AssayNamespace::Aid().to_string(), "aid");
        assert_eq!(AssayNamespace::ListKey().to_string(), "listkey");
        assert_eq!(AssayNamespace::Type(AssayType::All).to_string(), "type/all");
        assert_eq!(
            AssayNamespace::Type(AssayType::Screening).to_string(),
            "type/screening"
        );
        assert_eq!(
            AssayNamespace::SourceAll("PubChem".to_string()).to_string(),
            "sourceall/PubChem"
        );
        assert_eq!(
            AssayNamespace::Target(AssayTarget::Gi).to_string(),
            "target/gi"
        );
        assert_eq!(
            AssayNamespace::Target(AssayTarget::ProteinName).to_string(),
            "target/proteinname"
        );
        assert_eq!(
            AssayNamespace::Activity("IC50".to_string()).to_string(),
            "activity/IC50"
        );
    }

    #[test]
    fn test_assay_namespace_default() {
        assert_eq!(AssayNamespace::default(), AssayNamespace::Aid());
    }

    // AssayType tests
    #[test]
    fn test_assay_type_parse() {
        assert_eq!(AssayType::from_str("all").unwrap(), AssayType::All);
        assert_eq!(
            AssayType::from_str("confirmatory").unwrap(),
            AssayType::Confirmatory
        );
        assert_eq!(
            AssayType::from_str("doseresponse").unwrap(),
            AssayType::Doseresponse
        );
        assert_eq!(AssayType::from_str("onhold").unwrap(), AssayType::OnHold);
        assert_eq!(AssayType::from_str("panel").unwrap(), AssayType::Panel);
        assert_eq!(AssayType::from_str("rnai").unwrap(), AssayType::Rnai);
        assert_eq!(
            AssayType::from_str("screening").unwrap(),
            AssayType::Screening
        );
        assert_eq!(AssayType::from_str("summary").unwrap(), AssayType::Summary);
        assert_eq!(
            AssayType::from_str("cellbased").unwrap(),
            AssayType::CellBased
        );
        assert_eq!(
            AssayType::from_str("biochemical").unwrap(),
            AssayType::Biochemical
        );
        assert_eq!(AssayType::from_str("invivo").unwrap(), AssayType::Invivo);
        assert_eq!(AssayType::from_str("invitro").unwrap(), AssayType::Invitro);
        assert_eq!(
            AssayType::from_str("activeconcentrationspecified").unwrap(),
            AssayType::ActiveConcentrationSpecified
        );
    }

    #[test]
    fn test_assay_type_parse_invalid() {
        assert!(AssayType::from_str("invalid").is_err());
        assert!(AssayType::from_str("ALL").is_err()); // Case sensitive
    }

    #[test]
    fn test_assay_type_display() {
        assert_eq!(AssayType::All.to_string(), "all");
        assert_eq!(AssayType::Confirmatory.to_string(), "confirmatory");
        assert_eq!(AssayType::CellBased.to_string(), "cellbased");
    }

    #[test]
    fn test_assay_type_default() {
        assert_eq!(AssayType::default(), AssayType::All);
    }

    // AssayTarget tests
    #[test]
    fn test_assay_target_parse() {
        assert_eq!(AssayTarget::from_str("gi").unwrap(), AssayTarget::Gi);
        assert_eq!(
            AssayTarget::from_str("proteinname").unwrap(),
            AssayTarget::ProteinName
        );
        assert_eq!(
            AssayTarget::from_str("geneid").unwrap(),
            AssayTarget::GeneID
        );
        assert_eq!(
            AssayTarget::from_str("genesymbol").unwrap(),
            AssayTarget::GeneSymbol
        );
        assert_eq!(
            AssayTarget::from_str("accession").unwrap(),
            AssayTarget::Accession
        );
    }

    #[test]
    fn test_assay_target_parse_invalid() {
        assert!(AssayTarget::from_str("invalid").is_err());
        assert!(AssayTarget::from_str("GI").is_err()); // Case sensitive
    }

    #[test]
    fn test_assay_target_display() {
        assert_eq!(AssayTarget::Gi.to_string(), "gi");
        assert_eq!(AssayTarget::ProteinName.to_string(), "proteinname");
        assert_eq!(AssayTarget::GeneID.to_string(), "geneid");
    }

    #[test]
    fn test_assay_target_default() {
        assert_eq!(AssayTarget::default(), AssayTarget::Gi);
    }

    // Round-trip tests (parse and display)
    #[test]
    fn test_assay_namespace_roundtrip() {
        let test_cases = vec![
            "aid",
            "listkey",
            "type/all",
            "type/screening",
            "sourceall/TestSource",
            "target/gi",
            "target/proteinname",
            "activity/IC50",
        ];

        for case in test_cases {
            let parsed = AssayNamespace::from_str(case).unwrap();
            assert_eq!(parsed.to_string(), case);
        }
    }
}
