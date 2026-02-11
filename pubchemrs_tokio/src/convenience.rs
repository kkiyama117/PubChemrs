//! Ergonomic convenience API for common PubChem queries.
//!
//! This module provides [`CompoundQuery`] and [`OtherInputsQuery`] as high-level
//! entry points that build on the lower-level [`PubChemClient`](crate::PubChemClient) methods.
//!
//! # Examples
//!
//! ```rust,no_run
//! use pubchemrs_tokio::{CompoundQuery, OtherInputsQuery};
//!
//! # async fn example() -> pubchemrs_tokio::error::Result<()> {
//! // Single compound property
//! let formula = CompoundQuery::with_name("aspirin")
//!     .molecular_formula()
//!     .await?;
//!
//! // Batch properties
//! let props = CompoundQuery::with_cids(&[2244, 962])
//!     .properties(&["MolecularFormula", "MolecularWeight"])
//!     .await?;
//!
//! // List all substance sources
//! let sources = OtherInputsQuery::substance_sources().fetch().await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use pubchemrs_struct::properties::CompoundProperties;
use pubchemrs_struct::requests::input::*;
use pubchemrs_struct::requests::operation::*;
use pubchemrs_struct::requests::output::OutputFormat;
use pubchemrs_struct::requests::url_builder::UrlBuilder;
use pubchemrs_struct::response::{Compound, PubChemInformationList, PubChemResponse};

use crate::client::PubChemClient;
use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// CompoundQuery
// ---------------------------------------------------------------------------

/// Lazy query builder for PubChem compound lookups.
///
/// Construct via `with_name`, `with_cid`, `with_smiles`, etc., then call a
/// terminal method (e.g. [`molecular_formula`](Self::molecular_formula),
/// [`properties`](Self::properties)) which performs the actual HTTP request.
///
/// # Examples
///
/// ```rust,no_run
/// use pubchemrs_tokio::CompoundQuery;
///
/// # async fn example() -> pubchemrs_tokio::error::Result<()> {
/// let formula = CompoundQuery::with_name("aspirin")
///     .molecular_formula()
///     .await?;
/// assert_eq!(formula, Some("C9H8O4".to_string()));
/// # Ok(())
/// # }
/// ```
pub struct CompoundQuery<'a> {
    namespace: CompoundNamespace,
    identifiers: Identifiers,
    client: Option<&'a PubChemClient>,
}

// -- Constructors -----------------------------------------------------------

impl<'a> CompoundQuery<'a> {
    /// Search by compound name (e.g. "aspirin", "caffeine").
    pub fn with_name(name: &str) -> Self {
        Self {
            namespace: CompoundNamespace::Name(),
            identifiers: name.into(),
            client: None,
        }
    }

    /// Search by PubChem Compound ID.
    pub fn with_cid(cid: u32) -> Self {
        Self {
            namespace: CompoundNamespace::Cid(),
            identifiers: cid.into(),
            client: None,
        }
    }

    /// Batch search by multiple CIDs.
    pub fn with_cids(cids: &[u32]) -> Self {
        let identifiers: Identifiers = cids.iter().map(|&c| IdentifierValue::Int(c)).collect();
        Self {
            namespace: CompoundNamespace::Cid(),
            identifiers,
            client: None,
        }
    }

    /// Search by SMILES string.
    pub fn with_smiles(smiles: &str) -> Self {
        Self {
            namespace: CompoundNamespace::Smiles(),
            identifiers: smiles.into(),
            client: None,
        }
    }

    /// Search by InChIKey.
    pub fn with_inchikey(inchikey: &str) -> Self {
        Self {
            namespace: CompoundNamespace::InchiKey(),
            identifiers: inchikey.into(),
            client: None,
        }
    }

    /// Search by molecular formula.
    pub fn with_formula(formula: &str) -> Self {
        Self {
            namespace: CompoundNamespace::Formula(),
            identifiers: formula.into(),
            client: None,
        }
    }

    /// Use a custom [`PubChemClient`] instead of the global default.
    pub fn using_client(mut self, client: &'a PubChemClient) -> Self {
        self.client = Some(client);
        self
    }

    fn resolve_client(&self) -> &PubChemClient {
        self.client.unwrap_or(PubChemClient::global_default())
    }
}

// -- Terminal methods: properties -------------------------------------------

impl CompoundQuery<'_> {
    /// Fetch specific properties by tag names.
    ///
    /// Returns one [`CompoundProperties`] per matched compound.
    /// For single-compound queries the vec typically has one element.
    ///
    /// When you need multiple properties, prefer this over individual accessor
    /// methods to avoid redundant HTTP requests.
    pub async fn properties(&self, tags: &[&str]) -> Result<Vec<CompoundProperties>> {
        let property_tags: Vec<CompoundPropertyTag> = tags.iter().map(|t| (*t).into()).collect();
        self.resolve_client()
            .get_properties(
                self.identifiers.clone(),
                self.namespace.clone(),
                &property_tags,
                HashMap::new(),
            )
            .await
    }

    /// Fetch a single [`CompoundProperties`] for single-compound queries.
    async fn first_properties(&self, tags: &[&str]) -> Result<Option<CompoundProperties>> {
        let results = self.properties(tags).await?;
        Ok(results.into_iter().next())
    }

    /// Fetch the molecular formula (e.g. "C9H8O4").
    pub async fn molecular_formula(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["MolecularFormula"])
            .await?
            .and_then(|p| p.molecular_formula))
    }

    /// Fetch the molecular weight as f64.
    pub async fn molecular_weight(&self) -> Result<Option<f64>> {
        Ok(self
            .first_properties(&["MolecularWeight"])
            .await?
            .and_then(|p| p.molecular_weight))
    }

    /// Fetch the IUPAC name.
    pub async fn iupac_name(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["IUPACName"])
            .await?
            .and_then(|p| p.iupac_name))
    }

    /// Fetch the InChI string.
    pub async fn inchi(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["InChI"])
            .await?
            .and_then(|p| p.inchi))
    }

    /// Fetch the InChIKey.
    pub async fn inchikey(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["InChIKey"])
            .await?
            .and_then(|p| p.inchikey))
    }

    /// Fetch the SMILES string (full, with stereochemistry).
    ///
    /// Falls back to the legacy `IsomericSMILES` field if the current `SMILES`
    /// field is absent.
    pub async fn smiles(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["IsomericSMILES"])
            .await?
            .and_then(|p| p.smiles.or(p.isomeric_smiles)))
    }

    /// Fetch the canonical (connectivity-only) SMILES.
    ///
    /// Falls back to the legacy `CanonicalSMILES` field if the current
    /// `ConnectivitySMILES` field is absent.
    pub async fn canonical_smiles(&self) -> Result<Option<String>> {
        Ok(self
            .first_properties(&["CanonicalSMILES"])
            .await?
            .and_then(|p| p.connectivity_smiles.or(p.canonical_smiles)))
    }

    /// Fetch the XLogP value.
    pub async fn xlogp(&self) -> Result<Option<f64>> {
        Ok(self
            .first_properties(&["XLogP"])
            .await?
            .and_then(|p| p.xlogp))
    }

    /// Fetch the exact mass.
    pub async fn exact_mass(&self) -> Result<Option<f64>> {
        Ok(self
            .first_properties(&["ExactMass"])
            .await?
            .and_then(|p| p.exact_mass))
    }

    /// Fetch the Topological Polar Surface Area (TPSA).
    pub async fn tpsa(&self) -> Result<Option<f64>> {
        Ok(self.first_properties(&["TPSA"]).await?.and_then(|p| p.tpsa))
    }

    /// Fetch the formal charge.
    pub async fn charge(&self) -> Result<Option<i32>> {
        Ok(self
            .first_properties(&["Charge"])
            .await?
            .and_then(|p| p.charge))
    }
}

// -- Terminal methods: synonyms & records -----------------------------------

impl CompoundQuery<'_> {
    /// Fetch synonyms for this compound.
    ///
    /// Returns a flat list of all synonym strings.
    pub async fn synonyms(&self) -> Result<Vec<String>> {
        let info_list = self
            .resolve_client()
            .get_synonyms(
                self.identifiers.clone(),
                Namespace::Compound(self.namespace.clone()),
                HashMap::new(),
            )
            .await?;
        Ok(info_list
            .into_iter()
            .flat_map(|info| info.synonym)
            .collect())
    }

    /// Fetch the full compound record (single compound).
    ///
    /// Returns the first compound from the response. Use [`records`](Self::records)
    /// for batch queries.
    pub async fn record(&self) -> Result<Compound> {
        let compounds = self.records().await?;
        compounds.into_iter().next().ok_or_else(|| {
            Error::PubChem(pubchemrs_struct::error::PubChemError::ParseResponseError(
                "No compound found".into(),
            ))
        })
    }

    /// Fetch all compound records (useful for batch queries).
    pub async fn records(&self) -> Result<Vec<Compound>> {
        self.resolve_client()
            .get_compounds(
                self.identifiers.clone(),
                self.namespace.clone(),
                HashMap::new(),
            )
            .await
    }

    /// Fetch the CID (PubChem Compound Identifier).
    ///
    /// Useful when searching by name, SMILES, or InChIKey to discover the CID.
    pub async fn cid(&self) -> Result<Option<u64>> {
        // CID is always included in any property response.
        // We request the lightest property to minimise payload.
        Ok(self
            .first_properties(&["MolecularFormula"])
            .await?
            .map(|p| p.cid))
    }
}

// ---------------------------------------------------------------------------
// OtherInputsQuery
// ---------------------------------------------------------------------------

/// Query builder for PubChem "Other Inputs" endpoints.
///
/// These are special input domains that do not deal with lists of PubChem
/// record identifiers. Currently supports:
///
/// - **Sources** — list of all depositors of substances or assays
/// - **Periodic Table** — summary data for PubChem's periodic table
///
/// # Examples
///
/// ```rust,no_run
/// use pubchemrs_tokio::OtherInputsQuery;
///
/// # async fn example() -> pubchemrs_tokio::error::Result<()> {
/// let sources = OtherInputsQuery::substance_sources().fetch().await?;
/// let table = OtherInputsQuery::periodic_table().fetch_json().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Future extensions
///
/// The following endpoints are not yet supported but may be added:
/// - **SourceTable** — detailed source information with record counts
/// - **Classification** — retrieve identifier lists from classification nodes
/// - **Standardize** — return standardized form of SMILES/InChI/SDF input
pub struct OtherInputsQuery<'a> {
    domain: DomainOtherInputs,
    client: Option<&'a PubChemClient>,
}

// -- Constructors -----------------------------------------------------------

impl<'a> OtherInputsQuery<'a> {
    /// List all current substance depositors (sources).
    pub fn substance_sources() -> Self {
        Self {
            domain: DomainOtherInputs::SourcesSubstances,
            client: None,
        }
    }

    /// List all current assay depositors (sources).
    pub fn assay_sources() -> Self {
        Self {
            domain: DomainOtherInputs::SourcesAssays,
            client: None,
        }
    }

    /// Retrieve the periodic table summary data.
    pub fn periodic_table() -> Self {
        Self {
            domain: DomainOtherInputs::Periodictable,
            client: None,
        }
    }

    /// Use a custom [`PubChemClient`] instead of the global default.
    pub fn using_client(mut self, client: &'a PubChemClient) -> Self {
        self.client = Some(client);
        self
    }

    fn resolve_client(&self) -> &PubChemClient {
        self.client.unwrap_or(PubChemClient::global_default())
    }

    fn build_url_builder(&self) -> UrlBuilder {
        UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Others(self.domain.clone()),
                namespace: Namespace::None(),
                identifiers: Identifiers::default(),
            },
            operation: Operation::OtherInput(),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        }
    }
}

// -- Terminal methods -------------------------------------------------------

impl OtherInputsQuery<'_> {
    /// Fetch the list of source names.
    ///
    /// Only valid for [`substance_sources`](Self::substance_sources) and
    /// [`assay_sources`](Self::assay_sources). Returns an error if called on
    /// other domain types (e.g. periodic table).
    pub async fn fetch(&self) -> Result<Vec<String>> {
        match &self.domain {
            DomainOtherInputs::SourcesSubstances | DomainOtherInputs::SourcesAssays => {}
            other => {
                return Err(Error::PubChem(
                    pubchemrs_struct::error::PubChemError::InvalidInput(
                        format!("fetch() is only valid for source queries, not {other}").into(),
                    ),
                ));
            }
        }
        let response = self
            .resolve_client()
            .get_and_parse(&self.build_url_builder())
            .await?;
        match response {
            PubChemResponse::InformationList(PubChemInformationList::SourceName(names)) => {
                Ok(names)
            }
            _ => Err(Error::PubChem(
                pubchemrs_struct::error::PubChemError::ParseResponseError(
                    "Expected SourceName list response".into(),
                ),
            )),
        }
    }

    /// Fetch the response as raw JSON.
    ///
    /// Use this for endpoints whose response type is not yet modeled
    /// (e.g. periodic table, source table).
    pub async fn fetch_json(&self) -> Result<serde_json::Value> {
        self.resolve_client()
            .get_json(&self.build_url_builder())
            .await
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- CompoundQuery constructors -----------------------------------------

    #[test]
    fn with_name_sets_namespace_and_identifiers() {
        let q = CompoundQuery::with_name("aspirin");
        assert_eq!(q.namespace, CompoundNamespace::Name());
        assert_eq!(q.identifiers, Identifiers::from("aspirin"));
        assert!(q.client.is_none());
    }

    #[test]
    fn with_cid_sets_namespace_and_identifiers() {
        let q = CompoundQuery::with_cid(2244);
        assert_eq!(q.namespace, CompoundNamespace::Cid());
        assert_eq!(q.identifiers, Identifiers::from(2244u32));
    }

    #[test]
    fn with_cids_sets_batch_identifiers() {
        let q = CompoundQuery::with_cids(&[2244, 5793]);
        assert_eq!(q.namespace, CompoundNamespace::Cid());
        let expected: Identifiers = vec![IdentifierValue::Int(2244), IdentifierValue::Int(5793)]
            .into_iter()
            .collect();
        assert_eq!(q.identifiers, expected);
    }

    #[test]
    fn with_smiles_sets_namespace() {
        let q = CompoundQuery::with_smiles("CC(=O)O");
        assert_eq!(q.namespace, CompoundNamespace::Smiles());
    }

    #[test]
    fn with_inchikey_sets_namespace() {
        let q = CompoundQuery::with_inchikey("BSYNRYMUTXBXSQ-UHFFFAOYSA-N");
        assert_eq!(q.namespace, CompoundNamespace::InchiKey());
    }

    #[test]
    fn with_formula_sets_namespace() {
        let q = CompoundQuery::with_formula("C9H8O4");
        assert_eq!(q.namespace, CompoundNamespace::Formula());
    }

    #[test]
    fn using_client_sets_custom_client() {
        let client = PubChemClient::default();
        let q = CompoundQuery::with_name("aspirin").using_client(&client);
        assert!(q.client.is_some());
    }

    #[test]
    fn resolve_client_returns_global_default_when_none() {
        let q = CompoundQuery::with_name("aspirin");
        let resolved = q.resolve_client() as *const PubChemClient;
        let global = PubChemClient::global_default() as *const PubChemClient;
        assert_eq!(resolved, global);
    }

    #[test]
    fn resolve_client_returns_custom_when_set() {
        let client = PubChemClient::default();
        let q = CompoundQuery::with_name("aspirin").using_client(&client);
        let resolved = q.resolve_client() as *const PubChemClient;
        let custom = &client as *const PubChemClient;
        assert_eq!(resolved, custom);
    }

    // -- OtherInputsQuery constructors --------------------------------------

    #[test]
    fn substance_sources_sets_domain() {
        let q = OtherInputsQuery::substance_sources();
        assert_eq!(q.domain, DomainOtherInputs::SourcesSubstances);
        assert!(q.client.is_none());
    }

    #[test]
    fn assay_sources_sets_domain() {
        let q = OtherInputsQuery::assay_sources();
        assert_eq!(q.domain, DomainOtherInputs::SourcesAssays);
    }

    #[test]
    fn periodic_table_sets_domain() {
        let q = OtherInputsQuery::periodic_table();
        assert_eq!(q.domain, DomainOtherInputs::Periodictable);
    }

    #[test]
    fn other_inputs_using_client() {
        let client = PubChemClient::default();
        let q = OtherInputsQuery::substance_sources().using_client(&client);
        assert!(q.client.is_some());
    }

    #[test]
    fn other_inputs_resolve_client_default() {
        let q = OtherInputsQuery::substance_sources();
        let resolved = q.resolve_client() as *const PubChemClient;
        let global = PubChemClient::global_default() as *const PubChemClient;
        assert_eq!(resolved, global);
    }

    #[test]
    fn other_inputs_build_url_builder_sources() {
        let q = OtherInputsQuery::substance_sources();
        let builder = q.build_url_builder();
        assert_eq!(
            builder.input_specification.domain,
            Domain::Others(DomainOtherInputs::SourcesSubstances)
        );
        assert_eq!(builder.input_specification.namespace, Namespace::None());
        assert!(builder.input_specification.identifiers.is_empty());
    }

    #[tokio::test]
    async fn fetch_rejects_non_source_domain() {
        let result = OtherInputsQuery::periodic_table().fetch().await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("fetch() is only valid for source queries"));
    }

    #[test]
    fn other_inputs_build_url_builder_periodic_table() {
        let q = OtherInputsQuery::periodic_table();
        let builder = q.build_url_builder();
        assert_eq!(
            builder.input_specification.domain,
            Domain::Others(DomainOtherInputs::Periodictable)
        );
    }
}
