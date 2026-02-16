use std::{fmt::Display, str::FromStr};

/// A list of compound property tags to retrieve from the PubChem API.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct CompoundProperty(
    /// The list of property tag names (e.g., `MolecularFormula`, `MolecularWeight`).
    pub Vec<CompoundPropertyTag>,
);

impl CompoundProperty {
    /// Returns `true` if no property tags are present or all are empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|inner| inner.is_empty())
    }

    /// Formats the property tags as a comma-separated string for use in the URL path.
    pub fn to_url_string(&self) -> String {
        self.0
            .iter()
            .map(|inner| inner.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Display for CompoundProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_string().fmt(f)
    }
}

impl FromStr for CompoundProperty {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tags: Vec<CompoundPropertyTag> = s.split(',').map(|t| t.to_string()).collect();
        if tags.is_empty() || tags.iter().all(|t| t.is_empty()) {
            Err(crate::error::ParseEnumError::VariantNotFound)
        } else {
            Ok(Self(tags))
        }
    }
}

impl FromIterator<CompoundPropertyTag> for CompoundProperty {
    fn from_iter<T: IntoIterator<Item = CompoundPropertyTag>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<I: Into<CompoundPropertyTag>> From<I> for CompoundProperty {
    fn from(value: I) -> Self {
        Self(vec![value.into()])
    }
}

/// A compound property tag name (e.g., `MolecularFormula`, `MolecularWeight`, `IUPACName`).
pub type CompoundPropertyTag = String;
