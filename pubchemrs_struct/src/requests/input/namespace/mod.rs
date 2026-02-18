//! Namespace types that determine how identifiers are interpreted in PubChem API requests.

mod assay;
mod compound;
mod others;
mod substance;
use std::str::FromStr;

use crate::requests::common::{DomainCompatible, UrlParts};
use crate::requests::input::Domain;
pub use assay::*;
pub use compound::*;
pub use others::*;
pub use substance::*;

/// Namespace specifying how identifiers are interpreted in a PubChem API request.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum Namespace {
    /// Compound-specific namespace (CID, name, SMILES, etc.)
    Compound(CompoundNamespace),
    /// Substance-specific namespace (SID, source, etc.)
    Substance(SubstanceNamespace),
    /// Assay-specific namespace (AID, type, target, etc.)
    Assay(AssayNamespace),
    /// Gene-specific namespace (gene ID, symbol, etc.)
    Gene(GeneNamespace),
    /// Protein-specific namespace (accession, GI, etc.)
    Protein(ProteinNamespace),
    /// Pathway-specific namespace (pathway accession)
    PathWay(PathWayNamespace),
    /// Taxonomy-specific namespace (taxonomy ID, etc.)
    Taxonomy(TaxonomyNamespace),
    /// Cell line-specific namespace (cell accession, etc.)
    Cell(CellNamespace),
    /// Empty namespace, used only for `DomainOtherInputs` domains.
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
                // InChI, SMILES, and SDF contain special characters (slashes, equals)
                // that break GET URL paths; PubChem requires POST for these.
                CompoundNamespace::InChI() => true,
                CompoundNamespace::Smiles() => true,
                CompoundNamespace::Sdf() => true,
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

impl DomainCompatible for Namespace {
    fn is_compatible_with_domain(&self, domain: &Domain) -> bool {
        matches!(
            (self, domain),
            (Namespace::Compound(_), Domain::Compound())
                | (Namespace::Substance(_), Domain::Substance())
                | (Namespace::Assay(_), Domain::Assay())
                | (Namespace::Gene(_), Domain::Gene())
                | (Namespace::Protein(_), Domain::Protein())
                | (Namespace::PathWay(_), Domain::PathWay())
                | (Namespace::Taxonomy(_), Domain::Taxonomy())
                | (Namespace::Cell(_), Domain::Cell())
                | (Namespace::None(), Domain::Others(_))
        )
    }

    fn type_label(&self) -> String {
        format!("namespace `{self}`")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::requests::input::DomainOtherInputs;
    use std::str::FromStr;

    #[test]
    fn test_namespace_domain_compatible_valid_pairs() {
        let valid_pairs: Vec<(Namespace, Domain)> = vec![
            (CompoundNamespace::Cid().into(), Domain::Compound()),
            (SubstanceNamespace::Sid().into(), Domain::Substance()),
            (AssayNamespace::Aid().into(), Domain::Assay()),
            (GeneNamespace::GeneID.into(), Domain::Gene()),
            (ProteinNamespace::Accession.into(), Domain::Protein()),
            (PathWayNamespace::Pwacc.into(), Domain::PathWay()),
            (TaxonomyNamespace::TaxID.into(), Domain::Taxonomy()),
            (CellNamespace::CellAcc.into(), Domain::Cell()),
            (
                Namespace::None(),
                Domain::Others(DomainOtherInputs::SourcesSubstances),
            ),
        ];
        for (ns, domain) in &valid_pairs {
            assert!(
                ns.is_compatible_with_domain(domain),
                "expected {ns} compatible with {domain}"
            );
            assert!(ns.validate_with_domain(domain).is_ok());
        }
    }

    #[test]
    fn test_namespace_domain_compatible_invalid_pairs() {
        let compound_ns = Namespace::Compound(CompoundNamespace::Cid());
        assert!(!compound_ns.is_compatible_with_domain(&Domain::Substance()));
        assert!(!compound_ns.is_compatible_with_domain(&Domain::Assay()));

        let err = compound_ns
            .validate_with_domain(&Domain::Substance())
            .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not compatible with domain"), "got: {msg}");
        assert!(msg.contains("substance"), "got: {msg}");
    }

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
