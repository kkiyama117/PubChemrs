use crate::error::PubChemResult;
use crate::requests::common::UrlParts;
use crate::requests::input::*;
use crate::requests::operation::*;
use crate::requests::output::*;

use std::collections::HashMap;
use std::fmt::Debug;

/// Base URL for PubChem PUG REST API
pub const PUBCHEM_API_BASE: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug";

/// Request builder for constructing PubChem PUG REST API URLs.
///
/// Assembles input specification, operation, and output format into URL path
/// segments and an optional POST body. Use `Default::default()` for optional fields.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct UrlBuilder {
    /// The input specification (domain, namespace, and identifiers).
    pub input_specification: InputSpecification,
    /// The operation to perform on the matched records.
    pub operation: Operation,
    /// The desired output format for the response.
    pub output: OutputFormat,
    /// Additional query parameters appended to the request URL.
    // TODO: Temporary impl
    pub kwargs: HashMap<String, String>,
}

impl UrlBuilder {
    /// Creates a new `UrlBuilder` with the given components.
    pub fn new(
        input_specification: InputSpecification,
        operation: Option<Operation>,
        output: OutputFormat,
        kwargs: HashMap<String, String>,
    ) -> Self {
        Self {
            input_specification,
            operation: operation.unwrap_or_default(),
            output,
            kwargs,
        }
    }

    /// Creates a `UrlBuilder` from individual component values.
    // TODO: Think how to get inputs
    pub fn from_values(
        // TODO: Create temporary union type of `str | int | List[str|int]` or other method
        identifiers: Identifiers,
        namespace: Namespace,
        domain: Domain,
        operation: Option<Operation>,
        output: OutputFormat,
        // TODO: Temporary impl
        kwargs: HashMap<String, String>,
    ) -> PubChemResult<Self> {
        let input_specification = InputSpecification {
            domain,
            namespace,
            identifiers,
        };
        Ok(Self {
            input_specification,
            operation: operation.unwrap_or_default(),
            output,
            kwargs,
        })
    }

    /// Build the URL path parts and optional POST body.
    ///
    /// Returns a tuple of (url_parts, optional_post_body).
    /// The url_parts can be joined with "/" and appended to `PUBCHEM_API_BASE`.
    pub fn build_url_parts(&self) -> PubChemResult<(Vec<String>, Option<String>)> {
        let input_specification = self.input_specification.validate()?;
        let (url_parts, request_body) = input_specification.to_url_parts_with_body();
        let url_parts: Vec<String> = url_parts
            .into_iter()
            .chain(self.operation.to_url_parts())
            .chain(self.output.to_url_parts())
            .filter(|inner| !inner.is_empty())
            .collect();
        Ok((url_parts, request_body))
    }
}
