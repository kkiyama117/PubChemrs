use std::{fmt::Display, str::FromStr};

use crate::requests::{
    common::{UrlParts, XRef},
    input::Namespace,
};

/// Namespace for the compound domain, specifying how to look up compounds.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CompoundNamespace {
    /// PubChem Compound ID (API value: `cid`)
    Cid(),
    /// Chemical name lookup (API value: `name`)
    Name(),
    /// SMILES notation (API value: `smiles`). Uses POST.
    Smiles(),
    /// InChI notation (API value: `inchi`). Uses POST.
    InChI(),
    /// SDF (Structure-Data File) input (API value: `sdf`). Uses POST.
    Sdf(),
    /// InChIKey lookup (API value: `inchikey`)
    InchiKey(),
    /// Molecular formula search (API value: `formula`). Uses POST.
    Formula(),
    /// Structure-based search (substructure, superstructure, similarity, identity)
    StructureSearch(StructureSearch),
    /// Cross-reference lookup (API path: `xref/<type>`)
    XRef(XRef),
    /// Mass-based lookup (API value: `mass`). Not fully implemented.
    Mass(),
    /// Async list key for paginated results (API value: `listkey`). Uses POST.
    ListKey(),
    /// PubChem fast search (identity, similarity, substructure, etc.)
    FastSearch(FastSearch),
}

impl Display for CompoundNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundNamespace::Cid() => write!(f, "cid"),
            CompoundNamespace::Name() => write!(f, "name"),
            CompoundNamespace::Smiles() => write!(f, "smiles"),
            CompoundNamespace::InChI() => write!(f, "inchi"),
            CompoundNamespace::Sdf() => write!(f, "sdf"),
            CompoundNamespace::InchiKey() => write!(f, "inchikey"),
            CompoundNamespace::Formula() => write!(f, "formula"),
            CompoundNamespace::StructureSearch(inner) => inner.fmt(f),
            CompoundNamespace::XRef(xref) => write!(f, "xref/{}", xref),
            CompoundNamespace::Mass() => write!(f, "mass"),
            CompoundNamespace::ListKey() => write!(f, "listkey"),
            CompoundNamespace::FastSearch(inner) => inner.fmt(f),
        }
    }
}

impl From<CompoundNamespace> for Namespace {
    fn from(value: CompoundNamespace) -> Self {
        Self::Compound(value)
    }
}

impl Default for CompoundNamespace {
    fn default() -> Self {
        Self::Cid()
    }
}

impl UrlParts for CompoundNamespace {
    fn to_url_parts(&self) -> Vec<String> {
        match self {
            CompoundNamespace::XRef(xref) => vec!["xref".to_string(), xref.to_string()],
            CompoundNamespace::StructureSearch(inner) => inner.to_url_parts(),
            CompoundNamespace::FastSearch(inner) => inner.to_url_parts(),
            _ => vec![self.to_string()],
        }
    }
}

impl FromStr for CompoundNamespace {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("xref/") {
            let inner = s.trim_start_matches("xref/");
            Self::XRef(XRef::from_str(inner)?)
        } else {
            match s {
                "cid" => Self::Cid(),
                "name" => Self::Name(),
                "smiles" => Self::Smiles(),
                "inchi" => Self::InChI(),
                "sdf" => Self::Sdf(),
                "inchikey" => Self::InchiKey(),
                "formula" => Self::Formula(),
                "mass" => Self::Mass(),
                "listkey" => Self::ListKey(),
                // If not matched, try to parse as structualsearch and then fastsearch if error is occured.
                _ => StructureSearch::from_str(s)
                    .map(Self::StructureSearch)
                    .or_else(|_e| FastSearch::from_str(s).map(Self::FastSearch))?,
            }
        })
    }
}

/// Structure search specification combining a search type and input format.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct StructureSearch {
    /// The type of structure search to perform.
    pub key: CompoundDomainStructureSearchKey,
    /// The input format for the structure query.
    pub value: CompoundDomainStructureSearchValue,
}

impl UrlParts for StructureSearch {
    fn to_url_parts(&self) -> Vec<String> {
        vec![self.key.to_string(), self.value.to_string()]
    }
}

impl Display for StructureSearch {
    // TODO: May replace slash to dot or do urlencoding.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.key, self.value)
    }
}

impl FromStr for StructureSearch {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let divided: Vec<_> = s.splitn(2, '/').collect();
        if divided.len() == 2 {
            let key = divided[0];
            let value = divided[1];
            Ok(Self {
                key: CompoundDomainStructureSearchKey::from_str(key)?,
                value: CompoundDomainStructureSearchValue::from_str(value)?,
            })
        } else {
            Err(crate::error::ParseEnumError::VariantNotFound)
        }
    }
}

/// Type of structure search to perform against PubChem.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CompoundDomainStructureSearchKey {
    /// Substructure search (API value: `substructure`)
    #[default]
    Substructure,
    /// Superstructure search (API value: `superstructure`)
    SuperStructure,
    /// Similarity search (API value: `similarity`)
    Similarity,
    /// Identity search (API value: `identity`)
    Identity,
}

impl_enum_str!(CompoundDomainStructureSearchKey {
    Substructure => "substructure",
    SuperStructure => "superstructure",
    Similarity => "similarity",
    Identity => "identity",
});

/// Input format for structure search queries.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CompoundDomainStructureSearchValue {
    /// SMILES notation (API value: `smiles`)
    #[default]
    Smiles,
    /// InChI notation (API value: `inchi`)
    InchI,
    /// SDF format (API value: `sdf`)
    Sdf,
    /// PubChem CID (API value: `cid`)
    Cid,
}

impl_enum_str!(CompoundDomainStructureSearchValue {
    Smiles => "smiles",
    InchI => "inchi",
    Sdf => "sdf",
    Cid => "cid",
});

/// PubChem fast search specification combining a search type and input format.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct FastSearch {
    /// The type of fast search to perform.
    pub key: CompoundDomainFastSearchKey,
    /// The input format for the fast search query.
    pub value: CompoundDomainFastSearchValue,
}

impl UrlParts for FastSearch {
    fn to_url_parts(&self) -> Vec<String> {
        vec![self.key.to_string(), self.value.to_string()]
    }
}

impl Display for FastSearch {
    // TODO: May replace slash to dot or do urlencoding.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.key {
            CompoundDomainFastSearchKey::FastFormula => self.key.fmt(f),
            _ => {
                write!(f, "{}/{}", self.key, self.value)
            }
        }
    }
}

impl FromStr for FastSearch {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let divided: Vec<_> = s.splitn(2, '/').collect();
        if divided.len() == 2 {
            let key = divided[0];
            let value = divided[1];
            Ok(Self {
                key: CompoundDomainFastSearchKey::from_str(key)?,
                value: CompoundDomainFastSearchValue::from_str(value)?,
            })
        } else {
            Err(crate::error::ParseEnumError::VariantNotFound)
        }
    }
}

/// Type of PubChem fast search operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CompoundDomainFastSearchKey {
    /// Fast identity search (API value: `fastidentity`)
    #[default]
    FastIdentity,
    /// Fast 2D similarity search (API value: `fastsimilarity_2d`)
    FastSimilarity2D,
    /// Fast 3D similarity search (API value: `fastsimilarity_3d`)
    FastSimilarity3D,
    /// Fast substructure search (API value: `fastsubstructure`)
    FastSubstructure,
    /// Fast superstructure search (API value: `fastsuperstructure`)
    FastSuperStructure,
    /// Fast formula search (API value: `fastformula`)
    FastFormula,
}

impl_enum_str!(CompoundDomainFastSearchKey {
    FastIdentity => "fastidentity",
    FastSimilarity2D => "fastsimilarity_2d",
    FastSimilarity3D => "fastsimilarity_3d",
    FastSubstructure => "fastsubstructure",
    FastSuperStructure => "fastsuperstructure",
    FastFormula => "fastformula",
});

/// Input format for PubChem fast search queries.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CompoundDomainFastSearchValue {
    /// SMILES notation (API value: `smiles`)
    #[default]
    Smiles,
    /// SMARTS pattern (API value: `smarts`)
    Smarts,
    /// InChI notation (API value: `inchi`)
    InchI,
    /// SDF format (API value: `sdf`)
    Sdf,
    /// PubChem CID (API value: `cid`)
    Cid,
    /// No input format, used only with `FastFormula` (API value: `none`)
    None,
}

impl_enum_str!(CompoundDomainFastSearchValue {
    Smiles => "smiles",
    Smarts => "smarts",
    InchI => "inchi",
    Sdf => "sdf",
    Cid => "cid",
    None => "none",
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // CompoundNamespace tests
    #[test]
    fn test_compound_namespace_parse() {
        assert_eq!(
            CompoundNamespace::from_str("cid").unwrap(),
            CompoundNamespace::Cid()
        );
        assert_eq!(
            CompoundNamespace::from_str("name").unwrap(),
            CompoundNamespace::Name()
        );
        assert_eq!(
            CompoundNamespace::from_str("smiles").unwrap(),
            CompoundNamespace::Smiles()
        );
        assert_eq!(
            CompoundNamespace::from_str("inchi").unwrap(),
            CompoundNamespace::InChI()
        );
        assert_eq!(
            CompoundNamespace::from_str("sdf").unwrap(),
            CompoundNamespace::Sdf()
        );
        assert_eq!(
            CompoundNamespace::from_str("inchikey").unwrap(),
            CompoundNamespace::InchiKey()
        );
        assert_eq!(
            CompoundNamespace::from_str("formula").unwrap(),
            CompoundNamespace::Formula()
        );
        assert_eq!(
            CompoundNamespace::from_str("mass").unwrap(),
            CompoundNamespace::Mass()
        );
        assert_eq!(
            CompoundNamespace::from_str("listkey").unwrap(),
            CompoundNamespace::ListKey()
        );
    }

    // CompoundDomainStructureSearchKey tests
    #[test]
    fn test_structure_search_key_parse() {
        assert_eq!(
            CompoundDomainStructureSearchKey::from_str("substructure").unwrap(),
            CompoundDomainStructureSearchKey::Substructure
        );
        assert_eq!(
            CompoundDomainStructureSearchKey::from_str("superstructure").unwrap(),
            CompoundDomainStructureSearchKey::SuperStructure
        );
        assert_eq!(
            CompoundDomainStructureSearchKey::from_str("similarity").unwrap(),
            CompoundDomainStructureSearchKey::Similarity
        );
        assert_eq!(
            CompoundDomainStructureSearchKey::from_str("identity").unwrap(),
            CompoundDomainStructureSearchKey::Identity
        );
    }

    // CompoundDomainStructureSearchValue tests
    #[test]
    fn test_structure_search_value_parse() {
        assert_eq!(
            CompoundDomainStructureSearchValue::from_str("smiles").unwrap(),
            CompoundDomainStructureSearchValue::Smiles
        );
        assert_eq!(
            CompoundDomainStructureSearchValue::from_str("inchi").unwrap(),
            CompoundDomainStructureSearchValue::InchI
        );
        assert_eq!(
            CompoundDomainStructureSearchValue::from_str("sdf").unwrap(),
            CompoundDomainStructureSearchValue::Sdf
        );
        assert_eq!(
            CompoundDomainStructureSearchValue::from_str("cid").unwrap(),
            CompoundDomainStructureSearchValue::Cid
        );
    }

    // CompoundDomainFastSearchKey tests
    #[test]
    fn test_fast_search_key_parse() {
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastidentity").unwrap(),
            CompoundDomainFastSearchKey::FastIdentity
        );
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastsimilarity_2d").unwrap(),
            CompoundDomainFastSearchKey::FastSimilarity2D
        );
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastsimilarity_3d").unwrap(),
            CompoundDomainFastSearchKey::FastSimilarity3D
        );
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastsubstructure").unwrap(),
            CompoundDomainFastSearchKey::FastSubstructure
        );
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastsuperstructure").unwrap(),
            CompoundDomainFastSearchKey::FastSuperStructure
        );
        assert_eq!(
            CompoundDomainFastSearchKey::from_str("fastformula").unwrap(),
            CompoundDomainFastSearchKey::FastFormula
        );
    }

    // CompoundDomainFastSearchValue tests
    #[test]
    fn test_fast_search_value_parse() {
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("smiles").unwrap(),
            CompoundDomainFastSearchValue::Smiles
        );
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("smarts").unwrap(),
            CompoundDomainFastSearchValue::Smarts
        );
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("inchi").unwrap(),
            CompoundDomainFastSearchValue::InchI
        );
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("sdf").unwrap(),
            CompoundDomainFastSearchValue::Sdf
        );
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("cid").unwrap(),
            CompoundDomainFastSearchValue::Cid
        );
        assert_eq!(
            CompoundDomainFastSearchValue::from_str("none").unwrap(),
            CompoundDomainFastSearchValue::None
        );
    }
}
