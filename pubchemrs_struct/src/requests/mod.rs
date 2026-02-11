// Each parts of url requests
mod common;
pub mod input;
pub mod operation;
pub mod output;
pub mod url_builder;

pub use common::{UrlParts, XRef};
pub use url_builder::{PUBCHEM_API_BASE, UrlBuilder};
