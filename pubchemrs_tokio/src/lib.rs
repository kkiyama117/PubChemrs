//! # pubchemrs_tokio
//!
//! Async HTTP client for the [PubChem PUG REST API](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest).
//!
//! This crate provides [`PubChemClient`] for making requests to PubChem with automatic
//! retry, GET/POST selection, and connection pooling. It uses the type definitions from
//! [`pubchemrs_struct`].
//!
//! ## Quick Start — Convenience API
//!
//! For common queries, use [`CompoundQuery`] or [`OtherInputsQuery`]:
//!
//! ```rust,no_run
//! use pubchemrs_tokio::{CompoundQuery, OtherInputsQuery};
//!
//! # async fn example() -> pubchemrs_tokio::error::Result<()> {
//! // Single property
//! let formula = CompoundQuery::with_name("aspirin")
//!     .molecular_formula()
//!     .await?;
//!
//! // Multiple properties in one request
//! let props = CompoundQuery::with_cid(2244)
//!     .properties(&["MolecularFormula", "MolecularWeight", "InChIKey"])
//!     .await?;
//!
//! // Batch query
//! let batch = CompoundQuery::with_cids(&[2244, 5793])
//!     .properties(&["MolecularFormula"])
//!     .await?;
//!
//! // Synonyms
//! let synonyms = CompoundQuery::with_name("caffeine")
//!     .synonyms()
//!     .await?;
//!
//! // List all substance sources
//! let sources = OtherInputsQuery::substance_sources().fetch().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Low-Level Client
//!
//! For full control, use [`PubChemClient`] directly:
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
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod convenience;
pub mod error;

pub use client::{ClientConfig, PubChemClient};
pub use convenience::{CompoundQuery, OtherInputsQuery};

// Re-export key types from pubchemrs_struct for convenience
pub use pubchemrs_struct;
