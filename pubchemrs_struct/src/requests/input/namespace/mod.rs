mod assay;
mod compound;
mod others;
mod substance;
use std::str::FromStr;

use crate::requests::common::UrlParts;
pub use assay::*;
pub use compound::*;
pub use others::*;
pub use substance::*;

/// Search namespace (how to interpret the identifier)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Namespace {
    Compound(CompoundNamespace),
    Substance(SubstanceNamespace),
    Assay(AssayNamespace),
    Gene(GeneNamespace),
    Protein(ProteinNamespace),
    PathWay(PathWayNamespace),
    Taxonomy(TaxonomyNamespace),
    Cell(CellNamespace),
    /// Only for `<other inputs>`
    None(),
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Namespace::Compound(inner) => inner.fmt(f),
            Namespace::Substance(inner) => inner.fmt(f),
            Namespace::Assay(inner) => inner.fmt(f),
            Namespace::Gene(inner) => inner.fmt(f),
            Namespace::Protein(inner) => inner.fmt(f),
            Namespace::PathWay(inner) => inner.fmt(f),
            Namespace::Taxonomy(inner) => inner.fmt(f),
            Namespace::Cell(inner) => inner.fmt(f),
            Namespace::None() => write!(f, ""),
        }
    }
}

// Complex Enum does not yet support Unit Variant
impl Default for Namespace {
    fn default() -> Self {
        Self::Compound(CompoundNamespace::Cid())
    }
}

impl FromStr for Namespace {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CompoundNamespace::from_str(s)
            .map(Self::Compound)
            .or(SubstanceNamespace::from_str(s).map(Self::Substance))
            .or(AssayNamespace::from_str(s).map(Self::Assay))
            .or(GeneNamespace::from_str(s).map(Self::Gene))
            .or(ProteinNamespace::from_str(s).map(Self::Protein))
            .or(PathWayNamespace::from_str(s).map(Self::PathWay))
            .or(TaxonomyNamespace::from_str(s).map(Self::Taxonomy))
            .or(CellNamespace::from_str(s).map(Self::Cell))
    }
}

impl TryFrom<&str> for Namespace {
    type Error = crate::error::ParseEnumError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl UrlParts for Namespace {
    fn to_url_parts(&self) -> Vec<String> {
        match self {
            Namespace::None() => vec![],
            Namespace::Compound(inner) => inner.to_url_parts(),
            Namespace::Substance(inner) => inner.to_url_parts(),
            Namespace::Assay(inner) => inner.to_url_parts(),
            _ => vec![self.to_string()],
        }
    }
}

impl Namespace {
    /// This is same check as [`pubchempy`](https://github.com/mcs07/PubChemPy/blob/9935a14e7fdb4a88d27a99fedce69ca99f004698/pubchempy.py#L360)
    pub fn is_search(&self) -> bool {
        match self {
            Namespace::Compound(cn) => match cn {
                CompoundNamespace::XRef(_) => true,
                CompoundNamespace::StructureSearch(_) => true,
                CompoundNamespace::FastSearch(_) => true,
                CompoundNamespace::Formula() => true,
                CompoundNamespace::ListKey() => true,
                // TODO: Check fastsearch or structure search with cid
                _ => false,
            },
            Namespace::Substance(sn) => matches!(
                sn,
                SubstanceNamespace::XRef(_)
                    | SubstanceNamespace::SourcdId(_)
                    | SubstanceNamespace::ListKey()
            ),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    // DomainOtherInputs tests
    #[test]
    fn test_namespace_parse() {
        assert_eq!(
            Namespace::from_str("cid").unwrap(),
            Namespace::Compound(CompoundNamespace::Cid())
        );
    }

    #[test]
    fn test_namespace_parse_invalid() {
        assert!(Namespace::from_str("invalid").is_err());
        assert!(Namespace::from_str("CID").is_err()); // Case sensitive
        assert!(Namespace::from_str("").is_err());
    }
}
