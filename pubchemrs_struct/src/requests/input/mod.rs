//! Input specification types for PubChem API requests.
//!
//! Combines domain, namespace, and identifiers to form the input portion
//! of a PUG REST URL.

mod domain;
mod identifiers;
mod namespace;

pub use domain::*;
pub use identifiers::*;
pub use namespace::*;
use std::{borrow::Cow, str::FromStr};

use crate::error::{PubChemError, PubChemResult};
use crate::requests::common::UrlParts;

/// Input specification combining domain, namespace, and identifiers for a PubChem API request.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct InputSpecification {
    /// The data domain to query (e.g., compound, substance, assay).
    pub domain: Domain,
    /// The namespace that determines how identifiers are interpreted.
    pub namespace: Namespace,
    /// The identifier values used to look up records.
    pub identifiers: Identifiers,
}

impl InputSpecification {
    /// Creates a new `InputSpecification` by parsing domain and namespace from strings.
    pub fn new<'a, D: Into<Cow<'a, str>>, N: Into<Cow<'a, str>>, I: Into<Identifiers>>(
        domain: D,
        namespace: N,
        identifiers: I,
    ) -> PubChemResult<Self> {
        Ok(Self {
            domain: Domain::from_str(&domain.into())?,
            namespace: Namespace::from_str(&namespace.into())?,
            identifiers: identifiers.into(),
        })
    }

    /// Check Input specification is good
    pub fn validate(&self) -> PubChemResult<&Self> {
        // Validate identifier is not empty
        if self.identifiers.is_empty() {
            match self.domain {
                // TODO: check each domain has identifiers or not with official document.
                Domain::Others(_) => {}
                _ => {
                    return Err(PubChemError::InvalidInput(
                        "identifier/cid cannot be None".into(),
                    ));
                }
            }
        }
        Ok(self)
    }

    /// Check if this request should use `POST` of HTTP
    /// Use POST for certain namespaces like formula searches
    pub fn use_post(&self) -> bool {
        self.namespace.is_search()
            || matches!(
                self.domain,
                Domain::Others(DomainOtherInputs::SourcesSubstances)
            )
            || matches!(
                self.domain,
                Domain::Others(DomainOtherInputs::SourcesAssays)
            )
    }

    /// Some requests use HTTP post with body
    pub fn to_url_parts_with_body(&self) -> (Vec<String>, Option<String>) {
        // Check if Request need post data or urlid to determine HTTP method and prepare POST body.
        // And if identifier is a list, join it with commas into string, but this process is already done in `to_string`.
        // And then, request body is like `cid=1,2,3,4,5`
        // See [PubChem PUG REST HP](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest#section=Request-POST-Body) also.
        if !self.use_post() {
            (
                self.domain
                    .to_url_parts()
                    .into_iter()
                    .chain(self.namespace.to_url_parts())
                    .chain(self.identifiers.to_url_parts())
                    .collect(),
                None,
            )
        } else {
            (
                self.domain
                    .to_url_parts()
                    .into_iter()
                    .chain(self.namespace.to_url_parts())
                    .collect(),
                Some(
                    [
                        self.namespace.to_string(),
                        // There is one element in [`identifilers.to_url_parts`]
                        self.identifiers
                            .to_url_parts()
                            .into_iter()
                            .collect::<Vec<_>>()[0]
                            .clone(),
                    ]
                    .join("="),
                ),
            )
        }
    }
}

impl UrlParts for InputSpecification {
    fn to_url_parts(&self) -> Vec<String> {
        self.to_url_parts_with_body().0
    }
}
