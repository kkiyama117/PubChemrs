use std::collections::HashMap;

use pyo3::prelude::*;
use pubchemrs_struct::properties::CompoundProperties;
use pubchemrs_struct::requests::input::CompoundNamespace;
use pubchemrs_struct::requests::operation::CompoundPropertyTag;
use pubchemrs_struct::response::Compound;
use pubchemrs_tokio::client::{ClientConfig, PubChemClient};

use crate::error::to_pyerr;

/// Python-facing PubChem API client.
///
/// Wraps the Rust `PubChemClient` and exposes both async (Python awaitable)
/// and synchronous methods.
#[pyclass(name = "PubChemClient")]
pub struct PyPubChemClient {
    inner: PubChemClient,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyPubChemClient {
    /// Create a new PubChemClient.
    ///
    /// Args:
    ///     timeout_secs: HTTP request timeout in seconds (default: 30).
    ///     max_retries: Maximum retry attempts on transient errors (default: 3).
    #[new]
    #[pyo3(signature = (timeout_secs=None, max_retries=None))]
    fn new(timeout_secs: Option<u64>, max_retries: Option<u32>) -> PyResult<Self> {
        let mut config = ClientConfig::default();
        if let Some(t) = timeout_secs {
            config.timeout = std::time::Duration::from_secs(t);
        }
        if let Some(r) = max_retries {
            config.max_retries = r;
        }
        let inner = PubChemClient::new(config).map_err(to_pyerr)?;
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner, runtime })
    }

    /// Fetch full compound records (async, returns Python awaitable).
    ///
    /// Args:
    ///     identifier: CID (int), name (str), or list of CIDs.
    ///     namespace: Namespace string (default: "cid").
    #[pyo3(signature = (identifier, namespace="cid"))]
    fn get_compounds<'py>(
        &self,
        py: Python<'py>,
        identifier: &Bound<'py, PyAny>,
        namespace: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ns = parse_compound_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = client
                .get_compounds(ids, ns, HashMap::new())
                .await
                .map_err(to_pyerr)?;
            Ok(result)
        })
    }

    /// Fetch full compound records (synchronous/blocking).
    #[pyo3(signature = (identifier, namespace="cid"))]
    fn get_compounds_sync(
        &self,
        py: Python<'_>,
        identifier: &Bound<'_, PyAny>,
        namespace: &str,
    ) -> PyResult<Vec<Compound>> {
        let ns = parse_compound_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let client = self.inner.clone();
        py.detach(|| {
            self.runtime
                .block_on(client.get_compounds(ids, ns, HashMap::new()))
                .map_err(to_pyerr)
        })
    }

    /// Fetch compound properties (async, returns Python awaitable).
    ///
    /// Args:
    ///     identifier: CID (int), name (str), or list of CIDs.
    ///     properties: List of property name strings.
    ///     namespace: Namespace string (default: "cid").
    #[pyo3(signature = (identifier, properties, namespace="cid"))]
    fn get_properties<'py>(
        &self,
        py: Python<'py>,
        identifier: &Bound<'py, PyAny>,
        properties: Vec<String>,
        namespace: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ns = parse_compound_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let props: Vec<CompoundPropertyTag> = properties;
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = client
                .get_properties(ids, ns, &props, HashMap::new())
                .await
                .map_err(to_pyerr)?;
            Ok(result)
        })
    }

    /// Fetch compound properties (synchronous/blocking).
    #[pyo3(signature = (identifier, properties, namespace="cid"))]
    fn get_properties_sync(
        &self,
        py: Python<'_>,
        identifier: &Bound<'_, PyAny>,
        properties: Vec<String>,
        namespace: &str,
    ) -> PyResult<Vec<CompoundProperties>> {
        let ns = parse_compound_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let props: Vec<CompoundPropertyTag> = properties;
        let client = self.inner.clone();
        py.detach(|| {
            self.runtime
                .block_on(client.get_properties(ids, ns, &props, HashMap::new()))
                .map_err(to_pyerr)
        })
    }

    /// Fetch synonyms for compounds (async, returns Python awaitable).
    #[pyo3(signature = (identifier, namespace="cid"))]
    fn get_synonyms<'py>(
        &self,
        py: Python<'py>,
        identifier: &Bound<'py, PyAny>,
        namespace: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let ns = parse_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = client
                .get_synonyms(ids, ns, HashMap::new())
                .await
                .map_err(to_pyerr)?;
            Ok(result)
        })
    }

    /// Fetch synonyms for compounds (synchronous/blocking).
    #[pyo3(signature = (identifier, namespace="cid"))]
    fn get_synonyms_sync(
        &self,
        py: Python<'_>,
        identifier: &Bound<'_, PyAny>,
        namespace: &str,
    ) -> PyResult<Vec<pubchemrs_struct::response::PubChemInformation>> {
        let ns = parse_namespace(namespace)?;
        let ids = extract_identifiers(identifier)?;
        let client = self.inner.clone();
        py.detach(|| {
            self.runtime
                .block_on(client.get_synonyms(ids, ns, HashMap::new()))
                .map_err(to_pyerr)
        })
    }

    /// Fetch all source names for a domain (async, returns Python awaitable).
    #[pyo3(signature = (domain=None))]
    fn get_all_sources<'py>(
        &self,
        py: Python<'py>,
        domain: Option<&str>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let d = parse_source_domain(domain);
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let result = client.get_all_sources(d).await.map_err(to_pyerr)?;
            Ok(result)
        })
    }

    /// Fetch all source names for a domain (synchronous/blocking).
    #[pyo3(signature = (domain=None))]
    fn get_all_sources_sync(&self, py: Python<'_>, domain: Option<&str>) -> PyResult<Vec<String>> {
        let d = parse_source_domain(domain);
        let client = self.inner.clone();
        py.detach(|| {
            self.runtime
                .block_on(client.get_all_sources(d))
                .map_err(to_pyerr)
        })
    }
}

fn parse_compound_namespace(ns: &str) -> PyResult<CompoundNamespace> {
    use std::str::FromStr;
    CompoundNamespace::from_str(ns).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid namespace '{ns}': {e}"))
    })
}

fn parse_namespace(ns: &str) -> PyResult<pubchemrs_struct::requests::input::Namespace> {
    use std::str::FromStr;
    pubchemrs_struct::requests::input::Namespace::from_str(ns).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid namespace '{ns}': {e}"))
    })
}

fn parse_source_domain(domain: Option<&str>) -> Option<pubchemrs_struct::requests::input::Domain> {
    match domain {
        Some("assay") => Some(pubchemrs_struct::requests::input::Domain::Assay()),
        Some("substance") | None => None,
        Some(other) => {
            use std::str::FromStr;
            pubchemrs_struct::requests::input::Domain::from_str(other).ok()
        }
    }
}

fn extract_identifiers(obj: &Bound<'_, PyAny>) -> PyResult<pubchemrs_struct::requests::input::Identifiers> {
    use pubchemrs_struct::requests::input::Identifiers;

    // Try integer (single CID)
    if let Ok(cid) = obj.extract::<u32>() {
        return Ok(cid.into());
    }

    // Try string (name, SMILES, etc.)
    if let Ok(s) = obj.extract::<String>() {
        return Ok(Identifiers::from(
            pubchemrs_struct::requests::input::IdentifierValue::from(s),
        ));
    }

    // Try list of integers
    if let Ok(cids) = obj.extract::<Vec<u32>>() {
        return Ok(Identifiers(
            cids.into_iter()
                .map(pubchemrs_struct::requests::input::IdentifierValue::from)
                .collect(),
        ));
    }

    // Try list of strings
    if let Ok(names) = obj.extract::<Vec<String>>() {
        return Ok(Identifiers(
            names
                .into_iter()
                .map(pubchemrs_struct::requests::input::IdentifierValue::from)
                .collect(),
        ));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "identifier must be int, str, list[int], or list[str]",
    ))
}
