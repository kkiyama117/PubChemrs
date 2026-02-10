use std::{fmt::Display, str::FromStr};

/// API operation (what to do with the data)
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct CompoundProperty(pub Vec<CompoundPropertyTag>);

impl CompoundProperty {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|inner| inner.is_empty())
    }

    /// In Url, This is joined with ","
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

/// Property of compound, or list of comma separated values
/// TODO: Check All properties
pub type CompoundPropertyTag = String;
