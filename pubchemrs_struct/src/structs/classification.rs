/// Compound record type indicating how the compound was processed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
pub enum CompoundIdType {
    /// As deposited by the submitter.
    Deposited = 0,
    /// Standardized form.
    Standardized = 1,
    /// Component of a mixture.
    Component = 2,
    /// Neutralized form.
    Neutralized = 3,
    /// Mixture record.
    Mixture = 4,
    /// Tautomeric form.
    Tautomer = 5,
    /// Ionized form.
    Ionized = 6,
    /// Unknown record type.
    Unknown = 255,
}

impl_enum_str!(CompoundIdType {
    Deposited => "DEPOSITED",
    Standardized => "STANDARDIZED",
    Component => "COMPONENT",
    Neutralized => "NEUTRALIZED",
    Mixture => "MIXTURE",
    Tautomer => "TAUTOMER",
    Ionized => "IONIZED",
    Unknown => "UNKNOWN",
});

/// Coordinate set type flags as returned in PubChem responses.
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
pub enum ResponseCoordinateType {
    /// Two-dimensional coordinates.
    #[serde(rename = "TWOD")]
    TwoD = 1,
    /// Three-dimensional coordinates.
    #[serde(rename = "THREED")]
    ThreeD = 2,
    /// Submitted by the depositor.
    Submitted = 3,
    /// Experimentally determined.
    Experimental = 4,
    /// Computationally generated.
    Computed = 5,
    /// Standardized coordinates.
    Standardized = 6,
    /// Augmented coordinates.
    Augmented = 7,
    /// Aligned coordinates.
    Aligned = 8,
    /// Compact representation.
    Compact = 9,
    /// Units in angstroms.
    #[serde(rename = "UNITS_ANGSTROMS")]
    UnitsAngstroms = 10,
    /// Units in nanometers.
    #[serde(rename = "UNITS_NANOMETERS")]
    UnitsNanometers = 11,
    /// Units in pixels.
    #[serde(rename = "UNITS_PIXEL")]
    UnitsPixel = 12,
    /// Units in points.
    #[serde(rename = "UNITS_POINTS")]
    UnitsPoints = 13,
    /// Units in standard bond lengths.
    #[serde(rename = "UNITS_STDBONDS")]
    UnitsStdbonds = 14,
    /// Unknown units.
    #[serde(rename = "UNITS_UNKNOWN")]
    UnitsUnknown = 255,
}

impl_enum_str!(ResponseCoordinateType {
    TwoD => "TWOD",
    ThreeD => "THREED",
    Submitted => "SUBMITTED",
    Experimental => "EXPERIMENTAL",
    Computed => "COMPUTED",
    Standardized => "STANDARDIZED",
    Augmented => "AUGMENTED",
    Aligned => "ALIGNED",
    Compact => "COMPACT",
    UnitsAngstroms => "UNITS_ANGSTROMS",
    UnitsNanometers => "UNITS_NANOMETERS",
    UnitsPixel => "UNITS_PIXEL",
    UnitsPoints => "UNITS_POINTS",
    UnitsStdbonds => "UNITS_STDBONDS",
    UnitsUnknown => "UNITS_UNKNOWN",
});

/// BioAssay project category.
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum ProjectCategory {
    /// Molecular Libraries Screening Center Network.
    #[serde(rename = "MLSCN")]
    Mlscn = 1,
    /// Molecular Libraries Probe Centers Network.
    #[serde(rename = "MLPCN")]
    Mlpcn = 2,
    /// MLSCN assay provider.
    #[serde(rename = "MLSCN_AP")]
    MlscnAp = 3,
    /// MLPCN assay provider.
    #[serde(rename = "MLPCN_AP")]
    MlpcnAp = 4,
    /// Journal article.
    #[serde(rename = "JOURNAL_ARTICLE")]
    JournalArticle = 5,
    /// Assay vendor.
    #[serde(rename = "ASSAY_VENDOR")]
    AssayVendor = 6,
    /// Literature-extracted data.
    #[serde(rename = "LITERATURE_EXTRACTED")]
    LiteratureExtracted = 7,
    /// Literature author-submitted.
    #[serde(rename = "LITERATURE_AUTHOR")]
    LiteratureAuthor = 8,
    /// Literature publisher-submitted.
    #[serde(rename = "LITERATURE_PUBLISHER")]
    LiteraturePublisher = 9,
    /// RNAi Global Initiative.
    #[serde(rename = "RNAIGI")]
    Rnaigi = 10,
    /// Other / uncategorized.
    #[serde(rename = "OTHER")]
    Other = 255,
}

impl_enum_str!(ProjectCategory {
    Mlscn => "MLSCN",
    Mlpcn => "MLPCN",
    MlscnAp => "MLSCN_AP",
    MlpcnAp => "MLPCN_AP",
    JournalArticle => "JOURNAL_ARTICLE",
    AssayVendor => "ASSAY_VENDOR",
    LiteratureExtracted => "LITERATURE_EXTRACTED",
    LiteratureAuthor => "LITERATURE_AUTHOR",
    LiteraturePublisher => "LITERATURE_PUBLISHER",
    Rnaigi => "RNAIGI",
    Other => "OTHER",
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_compound_id_type_roundtrip() {
        for &v in &[
            CompoundIdType::Deposited,
            CompoundIdType::Standardized,
            CompoundIdType::Component,
            CompoundIdType::Neutralized,
            CompoundIdType::Mixture,
            CompoundIdType::Tautomer,
            CompoundIdType::Ionized,
            CompoundIdType::Unknown,
        ] {
            let s = v.to_string();
            assert_eq!(CompoundIdType::from_str(&s).unwrap(), v);
        }
    }

    #[test]
    fn test_response_coordinate_type_roundtrip() {
        for &v in &[
            ResponseCoordinateType::TwoD,
            ResponseCoordinateType::ThreeD,
            ResponseCoordinateType::Submitted,
            ResponseCoordinateType::Computed,
            ResponseCoordinateType::UnitsAngstroms,
            ResponseCoordinateType::UnitsUnknown,
        ] {
            let s = v.to_string();
            assert_eq!(ResponseCoordinateType::from_str(&s).unwrap(), v);
        }
    }

    #[test]
    fn test_project_category_roundtrip() {
        for &v in &[
            ProjectCategory::Mlscn,
            ProjectCategory::MlscnAp,
            ProjectCategory::JournalArticle,
            ProjectCategory::LiteratureExtracted,
            ProjectCategory::Rnaigi,
            ProjectCategory::Other,
        ] {
            let s = v.to_string();
            assert_eq!(ProjectCategory::from_str(&s).unwrap(), v);
        }
    }

    #[test]
    fn test_compound_id_type_serde() {
        let v = CompoundIdType::Standardized;
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "\"STANDARDIZED\"");
        let de: CompoundIdType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, v);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(CompoundIdType::from_str("INVALID").is_err());
        assert!(ResponseCoordinateType::from_str("INVALID").is_err());
        assert!(ProjectCategory::from_str("INVALID").is_err());
    }
}
