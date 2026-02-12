//! Common traits and types shared across request construction.

/// Trait for types that can produce URL path segments.
pub trait UrlParts {
    /// Converts this value into a list of URL path segments.
    fn to_url_parts(&self) -> Vec<String>;
}

/// Cross-reference type for linking PubChem records to external databases.
///
/// String values use PascalCase to match the
/// [official PubChem PUG REST documentation](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest).
/// Note that the PubChem API is case-insensitive for xref path segments.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum XRef {
    /// Registry ID (API value: `RegistryID`)
    #[default]
    #[serde(alias = "registryid")]
    RegistryId,
    /// CAS Registry Number (API value: `RN`)
    #[serde(alias = "rn")]
    Rn,
    /// PubMed article ID (API value: `PubMedID`)
    #[serde(alias = "pubmedid")]
    PubMedId,
    /// MMDB structure ID (API value: `MMDBID`)
    #[serde(alias = "mmdbid")]
    MmdbId,
    /// Depositor database URL (API value: `DBURL`). May not exist for all records.
    #[serde(alias = "dburl")]
    DbUrl,
    /// Substance browser URL (API value: `SBURL`). May not exist for all records.
    #[serde(alias = "sburl")]
    SbUrl,
    /// NCBI protein GI number (API value: `ProteinGI`)
    #[serde(alias = "proteingi")]
    ProteinGi,
    /// NCBI nucleotide GI number (API value: `NucleotideGI`)
    #[serde(alias = "nucleotidegi")]
    NucleotideGi,
    /// NCBI taxonomy ID (API value: `TaxonomyID`)
    #[serde(alias = "taxonomyid")]
    TaxonomyId,
    /// OMIM (Mendelian Inheritance in Man) ID (API value: `MIMID`)
    #[serde(alias = "mimid")]
    MimId,
    /// NCBI gene ID (API value: `GeneID`)
    #[serde(alias = "geneid")]
    GeneId,
    /// NCBI probe ID (API value: `ProbeID`)
    #[serde(alias = "probeid")]
    ProbeId,
    /// Patent ID (API value: `PatentID`)
    #[serde(alias = "patentid")]
    PatentId,
    /// Depositor source name (API value: `SourceName`). May not exist for all records.
    #[serde(alias = "sourcename")]
    SourceName,
    /// Source category (API value: `SourceCategory`). May not exist for all records.
    #[serde(alias = "sourcecategory")]
    SourceCategory,
}

impl_enum_str!(XRef {
    RegistryId => "RegistryID",
    Rn => "RN",
    PubMedId => "PubMedID",
    MmdbId => "MMDBID",
    DbUrl => "DBURL",
    SbUrl => "SBURL",
    ProteinGi => "ProteinGI",
    NucleotideGi => "NucleotideGI",
    TaxonomyId => "TaxonomyID",
    MimId => "MIMID",
    GeneId => "GeneID",
    ProbeId => "ProbeID",
    PatentId => "PatentID",
    SourceName => "SourceName",
    SourceCategory => "SourceCategory",
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_xref_parse() {
        assert_eq!(XRef::from_str("RegistryID").unwrap(), XRef::RegistryId);
        assert_eq!(XRef::from_str("RN").unwrap(), XRef::Rn);
        assert_eq!(XRef::from_str("PubMedID").unwrap(), XRef::PubMedId);
        assert_eq!(XRef::from_str("MMDBID").unwrap(), XRef::MmdbId);
        assert_eq!(XRef::from_str("ProteinGI").unwrap(), XRef::ProteinGi);
        assert_eq!(XRef::from_str("NucleotideGI").unwrap(), XRef::NucleotideGi);
        assert_eq!(XRef::from_str("TaxonomyID").unwrap(), XRef::TaxonomyId);
        assert_eq!(XRef::from_str("MIMID").unwrap(), XRef::MimId);
        assert_eq!(XRef::from_str("GeneID").unwrap(), XRef::GeneId);
        assert_eq!(XRef::from_str("ProbeID").unwrap(), XRef::ProbeId);
        assert_eq!(XRef::from_str("PatentID").unwrap(), XRef::PatentId);
        assert_eq!(XRef::from_str("SourceName").unwrap(), XRef::SourceName);
        assert_eq!(
            XRef::from_str("SourceCategory").unwrap(),
            XRef::SourceCategory
        );
    }

    #[test]
    fn test_xref_display() {
        assert_eq!(XRef::RegistryId.to_string(), "RegistryID");
        assert_eq!(XRef::Rn.to_string(), "RN");
        assert_eq!(XRef::PubMedId.to_string(), "PubMedID");
        assert_eq!(XRef::SourceName.to_string(), "SourceName");
    }

    #[test]
    fn test_xref_serde_roundtrip() {
        let xref = XRef::RegistryId;
        let json = serde_json::to_string(&xref).unwrap();
        assert_eq!(json, "\"RegistryId\"");
        let parsed: XRef = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, xref);
    }

    #[test]
    fn test_xref_serde_deserialize_lowercase_alias() {
        let parsed: XRef = serde_json::from_str("\"registryid\"").unwrap();
        assert_eq!(parsed, XRef::RegistryId);
        let parsed: XRef = serde_json::from_str("\"pubmedid\"").unwrap();
        assert_eq!(parsed, XRef::PubMedId);
    }
}
