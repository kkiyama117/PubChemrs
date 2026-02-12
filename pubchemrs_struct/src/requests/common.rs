//! Common traits and types shared across request construction.

/// Trait for types that can produce URL path segments.
pub trait UrlParts {
    /// Converts this value into a list of URL path segments.
    fn to_url_parts(&self) -> Vec<String>;
}

/// Cross-reference type for linking PubChem records to external databases.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum XRef {
    /// Registry ID (API value: `registryid`)
    #[default]
    RegistryId,
    /// CAS Registry Number (API value: `rn`)
    Rn,
    /// PubMed article ID (API value: `pubmedid`)
    PubMedId,
    /// MMDB structure ID (API value: `mmdbid`)
    MmdbId,
    /// Depositor database URL (API value: `dburl`). May not exist for all records.
    DbUrl,
    /// Substance browser URL (API value: `sburl`). May not exist for all records.
    SbUrl,
    /// NCBI protein GI number (API value: `proteingi`)
    ProteinGi,
    /// NCBI nucleotide GI number (API value: `nucleotidegi`)
    NucleotideGi,
    /// NCBI taxonomy ID (API value: `taxonomyid`)
    TaxonomyId,
    /// OMIM (Mendelian Inheritance in Man) ID (API value: `mimid`)
    MimId,
    /// NCBI gene ID (API value: `geneid`)
    GeneId,
    /// NCBI probe ID (API value: `probeid`)
    ProbeId,
    /// Patent ID (API value: `patentid`)
    PatentId,
    /// Depositor source name (API value: `sourcename`). May not exist for all records.
    SourceName,
    /// Source category (API value: `sourcecategory`). May not exist for all records.
    SourceCategory,
}

impl_enum_str!(XRef {
    RegistryId => "registryid",
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
        assert_eq!(XRef::from_str("registryid").unwrap(), XRef::RegistryId);
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
