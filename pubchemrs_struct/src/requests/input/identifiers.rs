use std::{borrow::Cow, fmt::Display};

use crate::requests::common::UrlParts;

/// The identifier to use as a search query.
///
/// This is [Vec] of [`IdentifierValue`], and IdentifierValue is [u32] or [str].
/// If you only use one identifier, for example, when you search compound by `cid`, do like
/// ```no_run
/// use pubchemrs_struct::requests::input::Identifiers;
/// let identifiers: Identifiers = 32.into();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct Identifiers(pub Vec<IdentifierValue>);

impl Identifiers {
    /// Returns `true` if no identifiers are present or all are empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|inner| inner.is_empty())
    }
}

impl UrlParts for Identifiers {
    fn to_url_parts(&self) -> Vec<String> {
        vec![
            self.0
                .iter()
                .map(|inner| inner.to_url_string())
                .collect::<Vec<String>>()
                .join(","),
        ]
    }
}

impl FromIterator<IdentifierValue> for Identifiers {
    fn from_iter<T: IntoIterator<Item = IdentifierValue>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<I: Into<IdentifierValue>> From<I> for Identifiers {
    fn from(value: I) -> Self {
        Self(vec![value.into()])
    }
}

/// A single identifier value, either numeric or string.
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum IdentifierValue {
    /// Numeric identifier (e.g., CID, SID, AID).
    Int(u32),
    /// String identifier (e.g., chemical name, InChI, SMILES).
    String(String),
}

impl IdentifierValue {
    fn to_url_string(&self) -> String {
        urlencoding::encode(self.to_string().as_str()).into()
    }
}

impl Display for IdentifierValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierValue::Int(i) => i.fmt(f),
            IdentifierValue::String(s) => s.fmt(f),
        }
    }
}

impl IdentifierValue {
    /// Returns `true` if this identifier is empty (zero for `Int`, empty string for `String`).
    pub fn is_empty(&self) -> bool {
        match self {
            IdentifierValue::Int(i) => *i == 0,
            IdentifierValue::String(s) => s.is_empty(),
        }
    }
}

impl From<String> for IdentifierValue {
    fn from(value: String) -> Self {
        IdentifierValue::String(value)
    }
}

impl From<&str> for IdentifierValue {
    fn from(value: &str) -> Self {
        IdentifierValue::String(value.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for IdentifierValue {
    fn from(value: Cow<'a, str>) -> Self {
        IdentifierValue::String(value.into_owned())
    }
}

impl From<u32> for IdentifierValue {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}
