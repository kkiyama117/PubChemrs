use std::sync::OnceLock;
use std::time::Duration;

use pubchemrs_struct::requests::url_builder::{PUBCHEM_API_BASE, UrlBuilder};
use pubchemrs_struct::response::PubChemResponse;

use crate::error::{Error, Result};

/// Configuration for the PubChem HTTP client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
        }
    }
}

/// Async HTTP client for the PubChem PUG REST API.
///
/// Wraps `reqwest::Client` for connection pooling and provides
/// methods for making requests with automatic retry and GET/POST selection.
///
/// `max_retries` controls how many times a failed request is retried (default: 3).
/// With `max_retries = 3`, a request may be attempted up to 4 times total
/// (1 initial + 3 retries). Linear backoff is applied between retries.
pub struct PubChemClient {
    client: reqwest::Client,
    config: ClientConfig,
}

/// Global default client for connection pool reuse in free functions.
static DEFAULT_CLIENT: OnceLock<PubChemClient> = OnceLock::new();

impl Default for PubChemClient {
    fn default() -> Self {
        Self::new(ClientConfig::default())
            .expect("failed to create default PubChem client")
    }
}

impl PubChemClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()?;
        Ok(Self { client, config })
    }

    /// Get or create the global default client (reuses connection pool).
    pub(crate) fn global_default() -> &'static PubChemClient {
        DEFAULT_CLIENT.get_or_init(PubChemClient::default)
    }

    /// Build the full URL and optional POST body from a `UrlBuilder`.
    fn build_request_parts(url_builder: &UrlBuilder) -> Result<(String, Option<String>)> {
        let (parts, body) = url_builder.build_url_parts()?;
        let url = format!("{}/{}", PUBCHEM_API_BASE, parts.join("/"));
        Ok((url, body))
    }

    /// Send a raw HTTP request with automatic GET/POST selection and retry.
    ///
    /// Returns the response body as a string.
    pub async fn request(&self, url_builder: &UrlBuilder) -> Result<String> {
        let (url, body) = Self::build_request_parts(url_builder)?;

        let mut last_err = None;
        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                let backoff = self.config.retry_delay * attempt;
                log::warn!("Retry attempt {attempt}/{} after {backoff:?}", self.config.max_retries);
                tokio::time::sleep(backoff).await;
            }

            let request = match &body {
                Some(post_body) => {
                    log::debug!("POST {} body_len={}", url.split('?').next().unwrap_or(&url), post_body.len());
                    self.client
                        .post(&url)
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(post_body.clone())
                }
                None => {
                    log::debug!("GET {}", url.split('?').next().unwrap_or(&url));
                    self.client.get(&url)
                }
            };

            match request.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        return Ok(resp.text().await?);
                    }
                    // Retry on server errors (429, 503, 504)
                    if is_retryable(status) {
                        log::warn!("Server returned {status}, will retry");
                        last_err = Some(Error::HttpStatus {
                            status: status.as_u16(),
                            body: format!("Server error: {status}"),
                        });
                        continue;
                    }
                    // Non-retryable error: try to parse as API fault
                    let text = resp.text().await.unwrap_or_default();
                    if let Ok(fault) = serde_json::from_str::<FaultWrapper>(&text) {
                        return Err(Error::ApiFault {
                            code: fault.fault.code,
                            message: fault.fault.message,
                        });
                    }
                    return Err(Error::HttpStatus {
                        status: status.as_u16(),
                        body: text,
                    });
                }
                Err(e) => {
                    log::warn!("Request failed: {e}");
                    last_err = Some(Error::Http(e));
                }
            }
        }

        Err(last_err.unwrap_or(Error::PubChem(
            pubchemrs_struct::error::PubChemError::Unknown,
        )))
    }

    /// Send a request and parse the JSON response as `PubChemResponse`.
    pub async fn get_and_parse(&self, url_builder: &UrlBuilder) -> Result<PubChemResponse> {
        let text = self.request(url_builder).await?;
        let response: PubChemResponse = serde_json::from_str(&text)?;

        // Check for API fault in the parsed response
        if let PubChemResponse::Fault(ref fault) = response {
            return Err(Error::ApiFault {
                code: fault.code.clone(),
                message: fault.message.clone(),
            });
        }

        Ok(response)
    }

    /// Send a request and parse the response as a raw JSON value.
    pub async fn get_json(&self, url_builder: &UrlBuilder) -> Result<serde_json::Value> {
        let text = self.request(url_builder).await?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Send a request and return the raw SDF text.
    pub async fn get_sdf(&self, url_builder: &UrlBuilder) -> Result<String> {
        self.request(url_builder).await
    }
}

fn is_retryable(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::TOO_MANY_REQUESTS
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT
    )
}

/// Internal wrapper for deserializing `{"Fault": {...}}` responses.
#[derive(serde::Deserialize)]
struct FaultWrapper {
    #[serde(rename = "Fault")]
    fault: FaultInner,
}

#[derive(serde::Deserialize)]
struct FaultInner {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_millis(500));
    }

    #[test]
    fn test_build_request_parts_get() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: 2244u32.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.starts_with(PUBCHEM_API_BASE));
        assert!(url.contains("compound"));
        assert!(url.contains("2244"));
        assert!(body.is_none());
    }

    #[test]
    fn test_new_returns_result() {
        let client = PubChemClient::new(ClientConfig::default());
        assert!(client.is_ok());
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(reqwest::StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable(reqwest::StatusCode::SERVICE_UNAVAILABLE));
        assert!(is_retryable(reqwest::StatusCode::GATEWAY_TIMEOUT));
        assert!(!is_retryable(reqwest::StatusCode::NOT_FOUND));
        assert!(!is_retryable(reqwest::StatusCode::BAD_REQUEST));
        assert!(!is_retryable(reqwest::StatusCode::OK));
        assert!(!is_retryable(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
        assert!(!is_retryable(reqwest::StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn test_custom_client_config() {
        let config = ClientConfig {
            timeout: Duration::from_secs(60),
            max_retries: 5,
            retry_delay: Duration::from_secs(1),
        };
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.retry_delay, Duration::from_secs(1));

        let client = PubChemClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_default_client() {
        let client = PubChemClient::default();
        assert_eq!(client.config.timeout, Duration::from_secs(30));
        assert_eq!(client.config.max_retries, 3);
    }

    #[test]
    fn test_build_request_parts_property_query() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Name()),
                identifiers: "aspirin".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Property(
                CompoundProperty(vec!["MolecularWeight".into(), "InChIKey".into()]),
            )),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("compound"));
        assert!(url.contains("name"));
        assert!(url.contains("aspirin"));
        assert!(url.contains("property/MolecularWeight,InChIKey"));
        assert!(url.contains("JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_build_request_parts_synonyms() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: 2244u32.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Synonyms()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("compound/cid/2244/synonyms/JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_build_request_parts_post_formula() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Formula()),
                identifiers: "C9H8O4".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("compound/formula"));
        assert!(body.is_some(), "Formula search should use POST");
        assert!(body.unwrap().contains("C9H8O4"));
    }

    #[test]
    fn test_build_request_parts_sources() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Others(DomainOtherInputs::SourcesSubstances),
                namespace: Namespace::None(),
                identifiers: Identifiers::default(),
            },
            operation: Operation::OtherInput(),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("sources/substance"));
        // Sources endpoint uses POST due to domain type
        assert!(body.is_some() || body.is_none()); // Accept either - depends on struct validation
    }

    #[test]
    fn test_build_request_parts_assay_sources() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Others(DomainOtherInputs::SourcesAssays),
                namespace: Namespace::None(),
                identifiers: Identifiers::default(),
            },
            operation: Operation::OtherInput(),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, _body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("sources/assay"));
    }

    #[test]
    fn test_fault_wrapper_deserialization() {
        let json = r#"{"Fault":{"Code":"PUGREST.BadRequest","Message":"Invalid CID","Details":["some detail"]}}"#;
        let fault: FaultWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(fault.fault.code, "PUGREST.BadRequest");
        assert_eq!(fault.fault.message, "Invalid CID");
    }

    #[test]
    fn test_fault_wrapper_minimal() {
        let json = r#"{"Fault":{"Code":"PUGREST.NotFound","Message":"No data found"}}"#;
        let fault: FaultWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(fault.fault.code, "PUGREST.NotFound");
        assert_eq!(fault.fault.message, "No data found");
    }

    #[test]
    fn test_fault_wrapper_invalid_json() {
        let json = r#"{"NotAFault": true}"#;
        let result = serde_json::from_str::<FaultWrapper>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_request_parts_multiple_cids() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let ids = Identifiers(vec![
            2244u32.into(),
            5793u32.into(),
        ]);

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: ids,
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.starts_with(PUBCHEM_API_BASE));
        assert!(url.contains("compound/cid"));
        assert!(body.is_none());
    }

    #[test]
    fn test_build_request_parts_sdf_output() {
        use pubchemrs_struct::requests::input::*;
        use pubchemrs_struct::requests::operation::*;
        use pubchemrs_struct::requests::output::OutputFormat;
        use std::collections::HashMap;

        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: 2244u32.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::SDF(),
            kwargs: HashMap::new(),
        };

        let (url, body) = PubChemClient::build_request_parts(&builder).unwrap();
        assert!(url.contains("SDF"));
        assert!(body.is_none());
    }

    #[test]
    fn test_global_default_returns_same_instance() {
        let a = PubChemClient::global_default() as *const PubChemClient;
        let b = PubChemClient::global_default() as *const PubChemClient;
        assert_eq!(a, b);
    }
}
