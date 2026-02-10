//! Common parts and trait for UrlBuilder

pub trait UrlParts {
    fn to_url_parts(&self) -> Vec<String>;
}

/// Xref
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum XRef {
    #[default]
    ResigtryId,
    Rn,
    PubMedId,
    MmdbId,
    /// May not exists
    DbUrl,
    /// May not exists
    SbUrl,
    ProteinGi,
    NucleotideGi,
    TaxonomyId,
    MimId,
    GeneId,
    ProbeId,
    PatentId,
    /// May not exists
    SourceName,
    /// May not exists
    SourceCategory,
}

impl_enum_str!(XRef {
    ResigtryId => "resigtryid",
    Rn => "rn",
    PubMedId => "pubmedid",
    MmdbId => "mmdbid",
    DbUrl => "dburl",
    SbUrl => "sburl",
    ProteinGi => "proteingi",
    NucleotideGi => "nucleotidegi",
    TaxonomyId => "taxonomyid",
    MimId => "mimid",
    GeneId => "geneid",
    ProbeId => "probeid",
    PatentId => "patentid",
    SourceName => "sourcename",
    SourceCategory => "sourcecategory",
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_xref_parse() {
        assert_eq!(XRef::from_str("resigtryid").unwrap(), XRef::ResigtryId);
        assert_eq!(XRef::from_str("rn").unwrap(), XRef::Rn);
        assert_eq!(XRef::from_str("pubmedid").unwrap(), XRef::PubMedId);
        assert_eq!(XRef::from_str("mmdbid").unwrap(), XRef::MmdbId);
        assert_eq!(XRef::from_str("proteingi").unwrap(), XRef::ProteinGi);
        assert_eq!(XRef::from_str("nucleotidegi").unwrap(), XRef::NucleotideGi);
        assert_eq!(XRef::from_str("taxonomyid").unwrap(), XRef::TaxonomyId);
        assert_eq!(XRef::from_str("mimid").unwrap(), XRef::MimId);
        assert_eq!(XRef::from_str("geneid").unwrap(), XRef::GeneId);
        assert_eq!(XRef::from_str("probeid").unwrap(), XRef::ProbeId);
        assert_eq!(XRef::from_str("patentid").unwrap(), XRef::PatentId);
    }
}
