//! Request construction types for the PubChem PUG REST API.
//!
//! This module provides the building blocks to construct API request URLs following
//! the PUG REST pattern: `/{domain}/{namespace}/{identifiers}/{operation}/{output}`.

mod common;
pub mod input;
pub mod operation;
pub mod output;
/// URL construction from request components.
pub mod url_builder;

pub use common::{UrlParts, XRef};
pub use url_builder::{PUBCHEM_API_BASE, UrlBuilder};
