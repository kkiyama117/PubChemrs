use std::fmt::Display;
use std::str::FromStr;

use crate::requests::common::UrlParts;

/// API domain (what type of data to retrieve)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Domain {
    Compound(),
    Substance(),
    Assay(),
    Gene(),
    Protein(),
    PathWay(),
    Taxonomy(),
    Cell(),
    /// TODO: Implement this
    Others(DomainOtherInputs),
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::Compound() => write!(f, "compound"),
            Domain::Substance() => write!(f, "substance"),
            Domain::Assay() => write!(f, "assay"),
            Domain::Gene() => write!(f, "gene"),
            Domain::Protein() => write!(f, "protein"),
            Domain::PathWay() => write!(f, "pathway"),
            Domain::Taxonomy() => write!(f, "taxonomy"),
            Domain::Cell() => write!(f, "cell"),
            Domain::Others(inner) => inner.fmt(f),
        }
    }
}

impl FromStr for Domain {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compound" => Ok(Self::Compound()),
            "substance" => Ok(Self::Substance()),
            "assay" => Ok(Self::Assay()),
            "gene" => Ok(Self::Gene()),
            "protein" => Ok(Self::Protein()),
            "pathway" => Ok(Self::PathWay()),
            "taxonomy" => Ok(Self::Taxonomy()),
            "cell" => Ok(Self::Cell()),
            other => DomainOtherInputs::from_str(other).map(Self::Others),
        }
    }
}

impl UrlParts for Domain {
    fn to_url_parts(&self) -> Vec<String> {
        match &self {
            Domain::Others(inner) => inner.to_url_parts(),
            _ => vec![self.to_string()],
        }
    }
}

// Complex Enum does not yet support Unit Variant
impl Default for Domain {
    fn default() -> Self {
        Self::Compound()
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum DomainOtherInputs {
    #[default]
    SourcesSubstances,
    SourcesAssays,
    SourceTable,
    Conformers,
    // TODO: Implement this
    /// SourceName or heading continues
    Annotations,
    Classification,
    Standardize,
    Periodictable,
}

impl Display for DomainOtherInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainOtherInputs::SourcesSubstances => write!(f, "sources/substance"),
            DomainOtherInputs::SourcesAssays => write!(f, "sources/assay"),
            DomainOtherInputs::SourceTable => write!(f, "sourcetable"),
            DomainOtherInputs::Conformers => write!(f, "conformers"),
            DomainOtherInputs::Annotations => write!(f, "annotations"),
            DomainOtherInputs::Classification => write!(f, "classification"),
            DomainOtherInputs::Standardize => write!(f, "standardize"),
            DomainOtherInputs::Periodictable => write!(f, "periodictable"),
        }
    }
}

impl FromStr for DomainOtherInputs {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sources/substance" => Ok(Self::SourcesSubstances),
            "sources/assay" => Ok(Self::SourcesAssays),
            "sourcetable" => Ok(Self::SourceTable),
            "conformers" => Ok(Self::Conformers),
            "annotations" => Ok(Self::Annotations),
            "classification" => Ok(Self::Classification),
            "standardize" => Ok(Self::Standardize),
            "periodictable" => Ok(Self::Periodictable),
            _ => Err(crate::error::ParseEnumError::VariantNotFound),
        }
    }
}

impl AsRef<str> for DomainOtherInputs {
    fn as_ref(&self) -> &str {
        match self {
            DomainOtherInputs::SourcesSubstances => "sources/substance",
            DomainOtherInputs::SourcesAssays => "sources/assay",
            DomainOtherInputs::SourceTable => "sourcetable",
            DomainOtherInputs::Conformers => "conformers",
            DomainOtherInputs::Annotations => "annotations",
            DomainOtherInputs::Classification => "classification",
            DomainOtherInputs::Standardize => "standardize",
            DomainOtherInputs::Periodictable => "periodictable",
        }
    }
}

impl From<DomainOtherInputs> for Domain {
    fn from(value: DomainOtherInputs) -> Self {
        Domain::Others(value)
    }
}

impl UrlParts for DomainOtherInputs {
    fn to_url_parts(&self) -> Vec<String> {
        match self {
            DomainOtherInputs::SourcesSubstances => {
                vec!["sources".to_string(), "substance".to_string()]
            }
            DomainOtherInputs::SourcesAssays => vec!["sources".to_string(), "assay".to_string()],
            _ => vec![self.to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // Domain tests
    #[test]
    fn test_domain_parse_basic() {
        assert_eq!(Domain::from_str("compound").unwrap(), Domain::Compound());
        assert_eq!(Domain::from_str("substance").unwrap(), Domain::Substance());
        assert_eq!(Domain::from_str("assay").unwrap(), Domain::Assay());
        assert_eq!(Domain::from_str("gene").unwrap(), Domain::Gene());
        assert_eq!(Domain::from_str("protein").unwrap(), Domain::Protein());
        assert_eq!(Domain::from_str("pathway").unwrap(), Domain::PathWay());
        assert_eq!(Domain::from_str("taxonomy").unwrap(), Domain::Taxonomy());
        assert_eq!(Domain::from_str("cell").unwrap(), Domain::Cell());
    }

    #[test]
    fn test_domain_parse_invalid() {
        assert!(Domain::from_str("invalid").is_err());
        assert!(Domain::from_str("COMPOUND").is_err()); // Case sensitive
        assert!(Domain::from_str("").is_err());
    }

    #[test]
    fn test_domain_other_inputs_parse() {
        assert_eq!(
            DomainOtherInputs::from_str("sources/substance").unwrap(),
            DomainOtherInputs::SourcesSubstances
        );
        assert_eq!(
            DomainOtherInputs::from_str("sources/assay").unwrap(),
            DomainOtherInputs::SourcesAssays
        );
        assert_eq!(
            DomainOtherInputs::from_str("sourcetable").unwrap(),
            DomainOtherInputs::SourceTable
        );
        assert_eq!(
            DomainOtherInputs::from_str("conformers").unwrap(),
            DomainOtherInputs::Conformers
        );
        assert_eq!(
            DomainOtherInputs::from_str("annotations").unwrap(),
            DomainOtherInputs::Annotations
        );
        assert_eq!(
            DomainOtherInputs::from_str("classification").unwrap(),
            DomainOtherInputs::Classification
        );
        assert_eq!(
            DomainOtherInputs::from_str("standardize").unwrap(),
            DomainOtherInputs::Standardize
        );
        assert_eq!(
            DomainOtherInputs::from_str("periodictable").unwrap(),
            DomainOtherInputs::Periodictable
        );
    }
}
