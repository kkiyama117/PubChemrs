//! Operation types specifying what action to perform on matched PubChem records.

mod assay;
mod compound;
mod property;
mod simple;
mod substance;
mod xrefs;

use std::{borrow::Cow, str::FromStr};

pub use assay::*;
pub use compound::*;
pub use property::*;
pub use simple::*;
pub use substance::*;
pub use xrefs::*;

use crate::requests::common::UrlParts;
use crate::requests::input::DomainOtherInputs;
use crate::{error::PubChemResult, requests::input::Domain};

/// API operation specifying what to do with matched PubChem records.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Operation {
    /// Operation for the compound domain.
    Compound(CompoundOperationSpecification),
    /// Operation for the substance domain.
    Substance(SubstanceOperationSpecification),
    /// Operation for the assay domain.
    Assay(AssayOperationSpecification),
    /// Operation for the gene domain.
    Gene(GeneOperationSpecification),
    /// Operation for the protein domain.
    Protein(ProteinOperationSpecification),
    /// Operation for the pathway domain.
    PathWay(PathWayOperationSpecification),
    /// Operation for the taxonomy domain.
    Taxonomy(TaxonomyOperationSpecification),
    /// Operation for the cell line domain.
    Cell(CellOperationSpecification),
    /// No operation, used for `DomainOtherInputs` domains.
    OtherInput(),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Compound(inner) => inner.fmt(f),
            Operation::Substance(inner) => inner.fmt(f),
            Operation::Assay(inner) => inner.fmt(f),
            Operation::Gene(inner) => inner.fmt(f),
            Operation::Protein(inner) => inner.fmt(f),
            Operation::PathWay(inner) => inner.fmt(f),
            Operation::Taxonomy(inner) => inner.fmt(f),
            Operation::Cell(inner) => inner.fmt(f),
            Operation::OtherInput() => write!(f, ""),
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self::Compound(Default::default())
    }
}

impl UrlParts for Operation {
    fn to_url_parts(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

/// Parses an operation string without domain context.
///
/// When a string matches multiple domain-specific operations (e.g., "record"
/// exists in compound, substance, and assay), the compound variant is
/// preferred. Use [`Operation::from_str_with_domain`] for unambiguous parsing.
impl FromStr for Operation {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CompoundOperationSpecification::from_str(s)
            .map(Self::Compound)
            .or(SubstanceOperationSpecification::from_str(s).map(Self::Substance))
            .or(AssayOperationSpecification::from_str(s).map(Self::Assay))
            .or(GeneOperationSpecification::from_str(s).map(Self::Gene))
            .or(ProteinOperationSpecification::from_str(s).map(Self::Protein))
            .or(PathWayOperationSpecification::from_str(s).map(Self::PathWay))
            .or(TaxonomyOperationSpecification::from_str(s).map(Self::Taxonomy))
            .or(CellOperationSpecification::from_str(s).map(Self::Cell))
    }
}

impl Operation {
    /// Returns the default operation for the given domain.
    pub fn default_with_domain(domain: &Domain) -> Self {
        match domain {
            Domain::Compound() => CompoundOperationSpecification::default().into(),
            Domain::Substance() => SubstanceOperationSpecification::default().into(),
            Domain::Assay() => AssayOperationSpecification::default().into(),
            Domain::Gene() => GeneOperationSpecification::default().into(),
            Domain::Protein() => ProteinOperationSpecification::default().into(),
            Domain::PathWay() => PathWayOperationSpecification::default().into(),
            Domain::Taxonomy() => TaxonomyOperationSpecification::default().into(),
            Domain::Cell() => CellOperationSpecification::default().into(),
            Domain::Others(_) => Self::OtherInput(),
        }
    }

    /// Parses an operation string in the context of a specific domain.
    pub fn from_str_with_domain<'a, S>(domain: &Domain, s: S) -> PubChemResult<Self>
    where
        S: Into<Cow<'a, str>>,
    {
        let s = s.into();
        let s_ref: &str = s.as_ref();
        match domain {
            Domain::Compound() => CompoundOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Substance() => SubstanceOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Assay() => AssayOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Gene() => GeneOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Protein() => ProteinOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::PathWay() => PathWayOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Taxonomy() => TaxonomyOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Cell() => CellOperationSpecification::from_str(s_ref)
                .map(Self::from)
                .map_err(|e| e.into()),
            Domain::Others(domain_other_inputs) => {
                match domain_other_inputs {
                    DomainOtherInputs::SourcesSubstances | DomainOtherInputs::SourcesAssays => {
                        Ok(Self::OtherInput())
                    }
                    // TODO: Validate operation compatibility for each DomainOtherInputs variant
                    // (e.g. Conformers, Annotations, Classification may have restricted operations)
                    _ => Self::from_str(s_ref).map_err(|e| e.into()),
                }
            }
        }
    }
}

impl From<CompoundOperationSpecification> for Operation {
    fn from(value: CompoundOperationSpecification) -> Self {
        Self::Compound(value)
    }
}

impl From<SubstanceOperationSpecification> for Operation {
    fn from(value: SubstanceOperationSpecification) -> Self {
        Self::Substance(value)
    }
}

impl From<AssayOperationSpecification> for Operation {
    fn from(value: AssayOperationSpecification) -> Self {
        Self::Assay(value)
    }
}

impl From<GeneOperationSpecification> for Operation {
    fn from(value: GeneOperationSpecification) -> Self {
        Self::Gene(value)
    }
}

impl From<ProteinOperationSpecification> for Operation {
    fn from(value: ProteinOperationSpecification) -> Self {
        Self::Protein(value)
    }
}

impl From<PathWayOperationSpecification> for Operation {
    fn from(value: PathWayOperationSpecification) -> Self {
        Self::PathWay(value)
    }
}

impl From<TaxonomyOperationSpecification> for Operation {
    fn from(value: TaxonomyOperationSpecification) -> Self {
        Self::Taxonomy(value)
    }
}

impl From<CellOperationSpecification> for Operation {
    fn from(value: CellOperationSpecification) -> Self {
        Self::Cell(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_from_str() {
        assert_eq!(
            Operation::from_str("record").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str("concise").unwrap(),
            Operation::Assay(AssayOperationSpecification::Concise())
        );
    }

    #[test]
    fn test_operation_from_str_with_domain() {
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Compound(), "record").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Compound(), "synonyms").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Synonyms())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Substance(), "record").unwrap(),
            Operation::Substance(SubstanceOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Assay(), "record").unwrap(),
            Operation::Assay(AssayOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Assay(), "concise").unwrap(),
            Operation::Assay(AssayOperationSpecification::Concise())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Gene(), "summary").unwrap(),
            Operation::Gene(GeneOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Protein(), "summary").unwrap(),
            Operation::Protein(ProteinOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::PathWay(), "summary").unwrap(),
            Operation::PathWay(PathWayOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Taxonomy(), "summary").unwrap(),
            Operation::Taxonomy(TaxonomyOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Cell(), "summary").unwrap(),
            Operation::Cell(CellOperationSpecification::Summary)
        );
    }

    #[test]
    fn test_operation_default() {
        assert_eq!(
            Operation::default(),
            Operation::Compound(CompoundOperationSpecification::Record())
        );
    }

    #[test]
    fn test_operation_from_conversions() {
        assert_eq!(
            Operation::from(CompoundOperationSpecification::Record()),
            Operation::Compound(CompoundOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from(SubstanceOperationSpecification::Record()),
            Operation::Substance(SubstanceOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from(AssayOperationSpecification::Record()),
            Operation::Assay(AssayOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from(GeneOperationSpecification::Summary),
            Operation::Gene(GeneOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from(ProteinOperationSpecification::Summary),
            Operation::Protein(ProteinOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from(PathWayOperationSpecification::Summary),
            Operation::PathWay(PathWayOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from(TaxonomyOperationSpecification::Summary),
            Operation::Taxonomy(TaxonomyOperationSpecification::Summary)
        );
        assert_eq!(
            Operation::from(CellOperationSpecification::Summary),
            Operation::Cell(CellOperationSpecification::Summary)
        );
    }
}
