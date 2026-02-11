//! # pubchemrs_tokio
//!
//! Async HTTP client for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest).
//!
//! This crate provides [`PubChemClient`] for making requests to PubChem with automatic
//! retry, GET/POST selection, and connection pooling. It uses the type definitions from
//! [`pubchemrs_struct`].
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use pubchemrs_tokio::PubChemClient;
//! use pubchemrs_struct::requests::input::CompoundNamespace;
//! use std::collections::HashMap;
//!
//! # async fn example() -> pubchemrs_tokio::error::Result<()> {
//! let client = PubChemClient::default();
//! let props = client.get_properties(
//!     "aspirin",
//!     CompoundNamespace::Name(),
//!     &["MolecularWeight".into(), "InChIKey".into()],
//!     HashMap::new(),
//! ).await?;
//! println!("{:?}", props[0].molecular_weight);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod error;

pub use client::{ClientConfig, PubChemClient};

// Re-export key types from pubchemrs_struct for convenience
pub use pubchemrs_struct;

use std::collections::HashMap;

use pubchemrs_struct::properties::CompoundProperties;
use pubchemrs_struct::requests::input::*;
use pubchemrs_struct::requests::operation::CompoundPropertyTag;
use pubchemrs_struct::response::{Compound, PubChemInformation};

/// Fetch full compound records using a default client.
pub async fn get_compounds(
    identifiers: impl Into<Identifiers>,
    namespace: CompoundNamespace,
    kwargs: HashMap<String, String>,
) -> error::Result<Vec<Compound>> {
    PubChemClient::global_default()
        .get_compounds(identifiers, namespace, kwargs)
        .await
}

/// Fetch compound properties using a default client.
pub async fn get_properties(
    identifiers: impl Into<Identifiers>,
    namespace: CompoundNamespace,
    properties: &[CompoundPropertyTag],
    kwargs: HashMap<String, String>,
) -> error::Result<Vec<CompoundProperties>> {
    PubChemClient::global_default()
        .get_properties(identifiers, namespace, properties, kwargs)
        .await
}

/// Fetch synonyms using a default client.
pub async fn get_synonyms(
    identifiers: impl Into<Identifiers>,
    namespace: Namespace,
    kwargs: HashMap<String, String>,
) -> error::Result<Vec<PubChemInformation>> {
    PubChemClient::global_default()
        .get_synonyms(identifiers, namespace, kwargs)
        .await
}

/// Fetch all source names using a default client.
pub async fn get_all_sources(domain: Option<Domain>) -> error::Result<Vec<String>> {
    PubChemClient::global_default()
        .get_all_sources(domain)
        .await
}
