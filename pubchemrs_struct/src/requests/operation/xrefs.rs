use std::{fmt::Display, str::FromStr};

use crate::requests::common::XRef;

/// API operation (what to do with the data)
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct XRefs(pub Vec<XRef>);

impl XRefs {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

impl Display for XRefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_string().fmt(f)
    }
}

impl FromIterator<XRef> for XRefs {
    fn from_iter<T: IntoIterator<Item = XRef>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<I: Into<XRef>> From<I> for XRefs {
    fn from(value: I) -> Self {
        Self(vec![value.into()])
    }
}

impl FromStr for XRefs {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let results: Result<Vec<XRef>, _> = s.split(',').map(XRef::from_str).collect();
        results.map(Self)
    }
}
