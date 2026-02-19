use crate::error::PubChemResult;
use crate::requests::common::{DomainCompatible, UrlParts};
use crate::requests::input::*;
use crate::requests::operation::*;
use crate::requests::output::*;

use std::collections::HashMap;
use std::fmt::Debug;

/// Base URL for PubChem PUG REST API
pub const PUBCHEM_API_BASE: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug";

/// Result of building a URL from a `UrlBuilder`.
///
/// Contains the path segments, optional POST body, and optional query string
/// derived from kwargs.
#[derive(Clone, Debug, Default)]
pub struct BuiltUrl {
    /// URL path segments to join with "/".
    pub path_segments: Vec<String>,
    /// Optional POST body (used for formula, InChI, SMILES, SDF searches).
    pub post_body: Option<String>,
    /// Optional query string (e.g. `record_type=3d`), without leading `?`.
    pub query_string: Option<String>,
}

impl BuiltUrl {
    /// Assemble the full URL string from path segments and optional query string.
    ///
    /// Joins `path_segments` with `/`, prepends `PUBCHEM_API_BASE`, and appends
    /// the query string (if any) after a `?`.
    pub fn to_full_url(&self) -> String {
        let mut url = format!("{}/{}", PUBCHEM_API_BASE, self.path_segments.join("/"));
        if let Some(ref qs) = self.query_string {
            url.push('?');
            url.push_str(qs);
        }
        url
    }
}

/// Request builder for constructing PubChem PUG REST API URLs.
///
/// Assembles input specification, operation, and output format into URL path
/// segments and an optional POST body. Use `Default::default()` for optional fields.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
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

    /// Build the URL path parts, optional POST body, and optional query string.
    ///
    /// Returns a `BuiltUrl` containing path segments (to join with "/" and append
    /// to `PUBCHEM_API_BASE`), an optional POST body, and an optional query string
    /// derived from `kwargs`.
    pub fn build_url_parts(&self) -> PubChemResult<BuiltUrl> {
        let input_specification = self.input_specification.validate()?;
        self.operation
            .validate_with_domain(&self.input_specification.domain)?;
        self.input_specification
            .namespace
            .validate_with_domain(&self.input_specification.domain)?;
        let (url_parts, post_body) = input_specification.to_url_parts_with_body();
        let path_segments: Vec<String> = url_parts
            .into_iter()
            .chain(self.operation.to_url_parts())
            .chain(self.output.to_url_parts())
            .filter(|inner| !inner.is_empty())
            .collect();

        let query_string = if self.kwargs.is_empty() {
            None
        } else {
            // Sort keys for deterministic output in tests
            let mut pairs: Vec<_> = self.kwargs.iter().collect();
            pairs.sort_by_key(|(k, _)| k.as_str());
            let qs = pairs
                .into_iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            Some(qs)
        };

        Ok(BuiltUrl {
            path_segments,
            post_body,
            query_string,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_parts_rejects_mismatched_operation() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Substance(),
                namespace: Namespace::Substance(SubstanceNamespace::Sid()),
                identifiers: Identifiers::from(1234u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs: HashMap::new(),
        };
        let err = builder.build_url_parts().unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not compatible with domain"), "got: {msg}");
    }

    #[test]
    fn test_build_url_parts_rejects_mismatched_namespace() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Substance(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(1234u32),
            },
            operation: Operation::Substance(SubstanceOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs: HashMap::new(),
        };
        let err = builder.build_url_parts().unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not compatible with domain"), "got: {msg}");
    }

    #[test]
    fn test_build_url_parts_accepts_matching_domain() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs: HashMap::new(),
        };
        assert!(builder.build_url_parts().is_ok());
    }

    #[test]
    fn test_build_url_parts_empty_kwargs_no_query_string() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs: HashMap::new(),
        };
        let built = builder.build_url_parts().unwrap();
        assert!(built.query_string.is_none());
    }

    #[test]
    fn test_build_url_parts_with_kwargs_produces_query_string() {
        let mut kwargs = HashMap::new();
        kwargs.insert("record_type".to_string(), "3d".to_string());
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs,
        };
        let built = builder.build_url_parts().unwrap();
        assert_eq!(built.query_string.as_deref(), Some("record_type=3d"));
    }

    #[test]
    fn test_build_url_parts_multiple_kwargs_sorted() {
        let mut kwargs = HashMap::new();
        kwargs.insert("record_type".to_string(), "3d".to_string());
        kwargs.insert("abc".to_string(), "123".to_string());
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs,
        };
        let built = builder.build_url_parts().unwrap();
        assert_eq!(
            built.query_string.as_deref(),
            Some("abc=123&record_type=3d")
        );
    }

    #[test]
    fn test_build_url_parts_kwargs_percent_encodes_special_chars() {
        let mut kwargs = HashMap::new();
        kwargs.insert(
            "name".to_string(),
            "a value with spaces&special=chars".to_string(),
        );
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs,
        };
        let built = builder.build_url_parts().unwrap();
        assert_eq!(
            built.query_string.as_deref(),
            Some("name=a%20value%20with%20spaces%26special%3Dchars")
        );
    }

    #[test]
    fn test_built_url_to_full_url() {
        let mut kwargs = HashMap::new();
        kwargs.insert("record_type".to_string(), "3d".to_string());
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs,
        };
        let built = builder.build_url_parts().unwrap();
        let url = built.to_full_url();
        assert!(url.starts_with(PUBCHEM_API_BASE));
        assert!(url.ends_with("?record_type=3d"));
    }

    #[test]
    fn test_built_url_to_full_url_no_query() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: Identifiers::from(2244u32),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::default(),
            kwargs: HashMap::new(),
        };
        let built = builder.build_url_parts().unwrap();
        let url = built.to_full_url();
        assert!(url.starts_with(PUBCHEM_API_BASE));
        assert!(!url.contains('?'));
    }
}
