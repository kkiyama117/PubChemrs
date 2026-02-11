mod property;
mod xrefs;

use std::{borrow::Cow, str::FromStr};

pub use property::*;
pub use xrefs::*;

use crate::requests::common::UrlParts;
use crate::requests::input::DomainOtherInputs;
use crate::{error::PubChemResult, requests::input::Domain};

/// API operation (what to do with the data)
/// TODO: Check implementation
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Operation {
    Compound(CompoundOperationSpecification),
    Substance(SubstanceOperationSpecification),
    Assay(AssayOperationSpecification),
    Gene(GeneOperationSpecification),
    Protein(ProteinOperationSpecification),
    PathWay(PathWayOperationSpecification),
    Taxonomy(TaxonomyOperationSpecification),
    Cell(CellOperationSpecification),
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
        // TODO: Use inner one
        // self.into().to_url_parts()
        vec![self.to_string()]
    }
}

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
                    // they may not accept operations
                    DomainOtherInputs::SourcesSubstances | DomainOtherInputs::SourcesAssays => {
                        Ok(Self::OtherInput())
                    }
                    // TODO: Check each `other inputs`
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

/// API operation (what to do with the data)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CompoundOperationSpecification {
    /// Get record. This is default operation at now.
    Record(),
    /// Retrieve compound information
    Property(CompoundProperty),
    Synonyms(),
    Sids(),
    Cids(),
    Aids(),
    AssaySummary(),
    Classification(),
    XRefs(XRefs),
    /// Get compound description
    Description(),
    Conformers(),
    /// For source search
    None(),
}

impl std::fmt::Display for CompoundOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundOperationSpecification::Record() => write!(f, "record"),
            CompoundOperationSpecification::Property(p) => write!(f, "property/{}", p),
            CompoundOperationSpecification::Synonyms() => write!(f, "synonyms"),
            CompoundOperationSpecification::Sids() => write!(f, "sids"),
            CompoundOperationSpecification::Cids() => write!(f, "cids"),
            CompoundOperationSpecification::Aids() => write!(f, "aids"),
            CompoundOperationSpecification::AssaySummary() => write!(f, "assaysummary"),
            CompoundOperationSpecification::Classification() => write!(f, "classification"),
            CompoundOperationSpecification::XRefs(x) => write!(f, "xrefs/{}", x),
            CompoundOperationSpecification::Description() => write!(f, "description"),
            CompoundOperationSpecification::Conformers() => write!(f, "conformers"),
            CompoundOperationSpecification::None() => write!(f, ""),
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
                // Invalid pattern
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        })
    }
}

/// API operation (what to do with the data)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum SubstanceOperationSpecification {
    /// Get record. This is default operation at now.
    Record(),
    Synonyms(),
    Sids(),
    Cids(),
    Aids(),
    AssaySummary(),
    Classification(),
    XRefs(XRefs),
    /// Get compound description
    Description(),
}

impl std::fmt::Display for SubstanceOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubstanceOperationSpecification::Record() => write!(f, "record"),
            SubstanceOperationSpecification::Synonyms() => write!(f, "synonyms"),
            SubstanceOperationSpecification::Sids() => write!(f, "sids"),
            SubstanceOperationSpecification::Cids() => write!(f, "cids"),
            SubstanceOperationSpecification::Aids() => write!(f, "aids"),
            SubstanceOperationSpecification::AssaySummary() => write!(f, "assaysummary"),
            SubstanceOperationSpecification::Classification() => write!(f, "classification"),
            SubstanceOperationSpecification::XRefs(x) => write!(f, "xrefs/{}", x),
            SubstanceOperationSpecification::Description() => write!(f, "description"),
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
                // Invalid pattern
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        })
    }
}

/// API operation (what to do with the data)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayOperationSpecification {
    /// Get record. This is default operation at now.
    Record(),
    Concise(),
    Aids(),
    Cids(),
    Sids(),
    Description(),
    Targets(AssayOperationTargetType),
    DoseResponse(),
    Summary(),
    Classification(),
}

impl std::fmt::Display for AssayOperationSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssayOperationSpecification::Record() => write!(f, "record"),
            AssayOperationSpecification::Concise() => write!(f, "concise"),
            AssayOperationSpecification::Aids() => write!(f, "aids"),
            AssayOperationSpecification::Cids() => write!(f, "cids"),
            AssayOperationSpecification::Sids() => write!(f, "sids"),
            AssayOperationSpecification::Description() => write!(f, "description"),
            AssayOperationSpecification::Targets(t) => write!(f, "targets/{}", t),
            AssayOperationSpecification::DoseResponse() => write!(f, "doseresponse/sid"),
            AssayOperationSpecification::Summary() => write!(f, "summary"),
            AssayOperationSpecification::Classification() => write!(f, "classification"),
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
                // Invalid pattern
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum AssayOperationTargetType {
    #[default]
    ProteinGI,
    ProteinName,
    GeneID,
    GeneSymbol,
}

impl_enum_str!(AssayOperationTargetType {
    ProteinGI => "proteingi",
    ProteinName => "proteinname",
    GeneID => "geneid",
    GeneSymbol => "genesymbol",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum GeneOperationSpecification {
    #[default]
    Summary,
    Aids,
    Concise,
    Pwaccs,
}

impl_enum_str!(GeneOperationSpecification {
    Summary => "summary",
    Aids => "aids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum ProteinOperationSpecification {
    #[default]
    Summary,
    Aids,
    Concise,
    Pwaccs,
}

impl_enum_str!(ProteinOperationSpecification {
    Summary => "summary",
    Aids => "aids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum PathWayOperationSpecification {
    #[default]
    Summary,
    Cids,
    Concise,
    Pwaccs,
}

impl_enum_str!(PathWayOperationSpecification {
    Summary => "summary",
    Cids => "cids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum TaxonomyOperationSpecification {
    #[default]
    Summary,
    Aids,
}

impl_enum_str!(TaxonomyOperationSpecification {
    Summary => "summary",
    Aids => "aids",
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CellOperationSpecification {
    #[default]
    Summary,
    Aids,
}

impl_enum_str!(CellOperationSpecification {
    Summary => "summary",
    Aids => "aids",
});

#[cfg(test)]
mod tests {
    use super::*;

    // CompoundOperationSpecification tests
    #[test]
    fn test_compound_operation_parse_basic() {
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
    fn test_compound_operation_parse_property() {
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
    fn test_compound_operation_parse_xrefs() {
        assert_eq!(
            CompoundOperationSpecification::from_str("xrefs/resigtryid").unwrap(),
            CompoundOperationSpecification::XRefs(XRefs::from_str("resigtryid").unwrap())
        );
    }

    #[test]
    fn test_compound_operation_parse_invalid() {
        assert!(CompoundOperationSpecification::from_str("invalid").is_err());
        assert!(CompoundOperationSpecification::from_str("RECORD").is_err()); // Case sensitive
    }

    #[test]
    fn test_compound_operation_display() {
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
    fn test_compound_operation_default() {
        assert_eq!(
            CompoundOperationSpecification::default(),
            CompoundOperationSpecification::Record()
        );
    }

    // SubstanceOperationSpecification tests
    #[test]
    fn test_substance_operation_parse_basic() {
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
    fn test_substance_operation_parse_xrefs() {
        assert_eq!(
            SubstanceOperationSpecification::from_str("xrefs/resigtryid").unwrap(),
            SubstanceOperationSpecification::XRefs(XRefs::from_str("resigtryid").unwrap())
        );
    }

    #[test]
    fn test_substance_operation_default() {
        assert_eq!(
            SubstanceOperationSpecification::default(),
            SubstanceOperationSpecification::Record()
        );
    }

    // AssayOperationSpecification tests
    #[test]
    fn test_assay_operation_parse_basic() {
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
    fn test_assay_operation_parse_targets() {
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
    fn test_assay_operation_default() {
        assert_eq!(
            AssayOperationSpecification::default(),
            AssayOperationSpecification::Record()
        );
    }

    // AssayOperationTargetType tests
    #[test]
    fn test_assay_operation_target_type_parse() {
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
    fn test_assay_operation_target_type_display() {
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
    fn test_assay_operation_target_type_default() {
        assert_eq!(
            AssayOperationTargetType::default(),
            AssayOperationTargetType::ProteinGI
        );
    }

    // GeneOperationSpecification tests
    #[test]
    fn test_gene_operation_parse() {
        assert_eq!(
            GeneOperationSpecification::from_str("summary").unwrap(),
            GeneOperationSpecification::Summary
        );
        assert_eq!(
            GeneOperationSpecification::from_str("aids").unwrap(),
            GeneOperationSpecification::Aids
        );
        assert_eq!(
            GeneOperationSpecification::from_str("concise").unwrap(),
            GeneOperationSpecification::Concise
        );
        assert_eq!(
            GeneOperationSpecification::from_str("pwaccs").unwrap(),
            GeneOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_gene_operation_display() {
        assert_eq!(GeneOperationSpecification::Summary.to_string(), "summary");
        assert_eq!(GeneOperationSpecification::Aids.to_string(), "aids");
        assert_eq!(GeneOperationSpecification::Concise.to_string(), "concise");
        assert_eq!(GeneOperationSpecification::Pwaccs.to_string(), "pwaccs");
    }

    #[test]
    fn test_gene_operation_default() {
        assert_eq!(
            GeneOperationSpecification::default(),
            GeneOperationSpecification::Summary
        );
    }

    // ProteinOperationSpecification tests
    #[test]
    fn test_protein_operation_parse() {
        assert_eq!(
            ProteinOperationSpecification::from_str("summary").unwrap(),
            ProteinOperationSpecification::Summary
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("aids").unwrap(),
            ProteinOperationSpecification::Aids
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("concise").unwrap(),
            ProteinOperationSpecification::Concise
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("pwaccs").unwrap(),
            ProteinOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_protein_operation_default() {
        assert_eq!(
            ProteinOperationSpecification::default(),
            ProteinOperationSpecification::Summary
        );
    }

    // PathWayOperationSpecification tests
    #[test]
    fn test_pathway_operation_parse() {
        assert_eq!(
            PathWayOperationSpecification::from_str("summary").unwrap(),
            PathWayOperationSpecification::Summary
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("cids").unwrap(),
            PathWayOperationSpecification::Cids
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("concise").unwrap(),
            PathWayOperationSpecification::Concise
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("pwaccs").unwrap(),
            PathWayOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_pathway_operation_default() {
        assert_eq!(
            PathWayOperationSpecification::default(),
            PathWayOperationSpecification::Summary
        );
    }

    // TaxonomyOperationSpecification tests
    #[test]
    fn test_taxonomy_operation_parse() {
        assert_eq!(
            TaxonomyOperationSpecification::from_str("summary").unwrap(),
            TaxonomyOperationSpecification::Summary
        );
        assert_eq!(
            TaxonomyOperationSpecification::from_str("aids").unwrap(),
            TaxonomyOperationSpecification::Aids
        );
    }

    #[test]
    fn test_taxonomy_operation_default() {
        assert_eq!(
            TaxonomyOperationSpecification::default(),
            TaxonomyOperationSpecification::Summary
        );
    }

    // CellOperationSpecification tests
    #[test]
    fn test_cell_operation_parse() {
        assert_eq!(
            CellOperationSpecification::from_str("summary").unwrap(),
            CellOperationSpecification::Summary
        );
        assert_eq!(
            CellOperationSpecification::from_str("aids").unwrap(),
            CellOperationSpecification::Aids
        );
    }

    #[test]
    fn test_cell_operation_default() {
        assert_eq!(
            CellOperationSpecification::default(),
            CellOperationSpecification::Summary
        );
    }

    // Operation tests
    #[test]
    fn test_operation_from_str() {
        // Compound
        assert_eq!(
            Operation::from_str("record").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Record())
        );

        // Assay (has unique 'concise')
        assert_eq!(
            Operation::from_str("concise").unwrap(),
            Operation::Assay(AssayOperationSpecification::Concise())
        );
    }

    #[test]
    fn test_operation_from_str_with_domain() {
        // Compound domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Compound(), "record").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Compound(), "synonyms").unwrap(),
            Operation::Compound(CompoundOperationSpecification::Synonyms())
        );

        // Substance domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Substance(), "record").unwrap(),
            Operation::Substance(SubstanceOperationSpecification::Record())
        );

        // Assay domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Assay(), "record").unwrap(),
            Operation::Assay(AssayOperationSpecification::Record())
        );
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Assay(), "concise").unwrap(),
            Operation::Assay(AssayOperationSpecification::Concise())
        );

        // Gene domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Gene(), "summary").unwrap(),
            Operation::Gene(GeneOperationSpecification::Summary)
        );

        // Protein domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Protein(), "summary").unwrap(),
            Operation::Protein(ProteinOperationSpecification::Summary)
        );

        // PathWay domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::PathWay(), "summary").unwrap(),
            Operation::PathWay(PathWayOperationSpecification::Summary)
        );

        // Taxonomy domain
        assert_eq!(
            Operation::from_str_with_domain(&Domain::Taxonomy(), "summary").unwrap(),
            Operation::Taxonomy(TaxonomyOperationSpecification::Summary)
        );

        // Cell domain
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
