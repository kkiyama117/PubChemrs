use std::fmt::Display;
use std::str::FromStr;

use super::super::Namespace;
use crate::requests::common::UrlParts;
use crate::requests::common::XRef;

/// Namespace for the substance domain, specifying how to look up substances.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum SubstanceNamespace {
    /// PubChem Substance ID (API value: `sid`)
    Sid(),
    /// Source-specific ID with depositor ID (API path: `sourceid/<id>`)
    SourcdId(u32),
    /// Search all sources by depositor name (API path: `sourceall/<name>`)
    SourceAll(String),
    /// Substance name lookup (API value: `name`)
    Name(),
    /// Cross-reference lookup (API path: `xref/<type>`)
    XRef(XRef),
    /// Async list key for paginated results (API value: `listkey`)
    ListKey(),
}

impl Display for SubstanceNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubstanceNamespace::Sid() => write!(f, "sid"),
            SubstanceNamespace::SourcdId(id) => write!(f, "sourceid/{}", id),
            SubstanceNamespace::SourceAll(s) => write!(f, "sourceall/{}", s),
            SubstanceNamespace::Name() => write!(f, "name"),
            SubstanceNamespace::XRef(xref) => write!(f, "xref/{}", xref),
            SubstanceNamespace::ListKey() => write!(f, "listkey"),
        }
    }
}

// Complex Enum does not yet support Unit Variant
impl Default for SubstanceNamespace {
    fn default() -> Self {
        Self::Sid()
    }
}

impl From<SubstanceNamespace> for Namespace {
    fn from(value: SubstanceNamespace) -> Self {
        Self::Substance(value)
    }
}

impl UrlParts for SubstanceNamespace {
    fn to_url_parts(&self) -> Vec<String> {
        match self {
            SubstanceNamespace::XRef(xref) => vec!["xref".to_string(), xref.to_string()],
            SubstanceNamespace::SourcdId(id) => vec!["sourceid".to_string(), id.to_string()],
            SubstanceNamespace::SourceAll(id) => vec!["sourceall".to_string(), id.to_string()],
            _ => vec![self.to_string()],
        }
    }
}

impl FromStr for SubstanceNamespace {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if s.starts_with("sourceid/") {
            let inner = s
                .trim_start_matches("sourceid/")
                .parse()
                .map_err(|_| crate::error::ParseEnumError::VariantNotFound)?;
            SubstanceNamespace::SourcdId(inner)
        } else if s.starts_with("sourceall/") {
            let inner = s.trim_start_matches("sourceall/");
            SubstanceNamespace::SourceAll(inner.to_string())
        } else if s.starts_with("xref/") {
            let inner = s.trim_start_matches("xref/");
            Self::XRef(XRef::from_str(inner)?)
        } else {
            match s {
                "sid" => Self::Sid(),
                "name" => Self::Name(),
                "listkey" => Self::ListKey(),
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // SubstanceNamespace tests
    #[test]
    fn test_substance_namespace_parse() {
        assert_eq!(
            SubstanceNamespace::from_str("sid").unwrap(),
            SubstanceNamespace::Sid()
        );
        assert_eq!(
            SubstanceNamespace::from_str("name").unwrap(),
            SubstanceNamespace::Name()
        );
        assert_eq!(
            SubstanceNamespace::from_str("listkey").unwrap(),
            SubstanceNamespace::ListKey()
        );
    }
}
