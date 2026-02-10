/// Compound record type.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq,
    serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CompoundIdType {
    Deposited = 0,
    Standardized = 1,
    Component = 2,
    Neutralized = 3,
    Mixture = 4,
    Tautomer = 5,
    Ionized = 6,
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

/// Coordinate Set Type Distinctions (response version).
#[derive(
    Copy, Clone, Debug, PartialEq, Eq,
    serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum ResponseCoordinateType {
    #[serde(rename = "TWOD")]
    TwoD = 1,
    #[serde(rename = "THREED")]
    ThreeD = 2,
    Submitted = 3,
    Experimental = 4,
    Computed = 5,
    Standardized = 6,
    Augmented = 7,
    Aligned = 8,
    Compact = 9,
    #[serde(rename = "UNITS_ANGSTROMS")]
    UnitsAngstroms = 10,
    #[serde(rename = "UNITS_NANOMETERS")]
    UnitsNanometers = 11,
    #[serde(rename = "UNITS_PIXEL")]
    UnitsPixel = 12,
    #[serde(rename = "UNITS_POINTS")]
    UnitsPoints = 13,
    #[serde(rename = "UNITS_STDBONDS")]
    UnitsStdbonds = 14,
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

/// Project Category Distinctions.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq,
    serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum ProjectCategory {
    #[serde(rename = "MLSCN")]
    Mlscn = 1,
    #[serde(rename = "MLPCN")]
    Mlpcn = 2,
    #[serde(rename = "MLSCN_AP")]
    MlscnAp = 3,
    #[serde(rename = "MLPCN_AP")]
    MlpcnAp = 4,
    #[serde(rename = "JOURNAL_ARTICLE")]
    JournalArticle = 5,
    #[serde(rename = "ASSAY_VENDOR")]
    AssayVendor = 6,
    #[serde(rename = "LITERATURE_EXTRACTED")]
    LiteratureExtracted = 7,
    #[serde(rename = "LITERATURE_AUTHOR")]
    LiteratureAuthor = 8,
    #[serde(rename = "LITERATURE_PUBLISHER")]
    LiteraturePublisher = 9,
    #[serde(rename = "RNAIGI")]
    Rnaigi = 10,
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
