use std::collections::HashMap;

use pubchemrs_struct::properties::{CompoundProperties, PropertyTableResponse};
use pubchemrs_struct::requests::input::*;
use pubchemrs_struct::requests::operation::*;
use pubchemrs_struct::requests::output::OutputFormat;
use pubchemrs_struct::requests::url_builder::UrlBuilder;
use pubchemrs_struct::response::{Compound, PubChemInformation, PubChemInformationList, PubChemResponse};

use crate::client::PubChemClient;
use crate::error::Result;

impl PubChemClient {
    /// Fetch full compound records from PubChem.
    ///
    /// Returns the raw `Compound` structures from the PUG REST API.
    pub async fn get_compounds(
        &self,
        identifiers: impl Into<Identifiers>,
        namespace: CompoundNamespace,
        kwargs: HashMap<String, String>,
    ) -> Result<Vec<Compound>> {
        let url_builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(namespace),
                identifiers: identifiers.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs,
        };

        let response = self.get_and_parse(&url_builder).await?;
        match response {
            PubChemResponse::Compounds(compounds) => Ok(compounds),
            _other => Err(crate::error::Error::PubChem(
                pubchemrs_struct::error::PubChemError::ParseResponseError(
                    "Expected Compounds response, got unexpected variant".into(),
                ),
            )),
        }
    }

    /// Fetch compound properties from PubChem.
    ///
    /// Uses the PropertyTable endpoint to retrieve specific properties.
    pub async fn get_properties(
        &self,
        identifiers: impl Into<Identifiers>,
        namespace: CompoundNamespace,
        properties: &[CompoundPropertyTag],
        kwargs: HashMap<String, String>,
    ) -> Result<Vec<CompoundProperties>> {
        let compound_property = CompoundProperty(properties.to_vec());

        let url_builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(namespace),
                identifiers: identifiers.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Property(
                compound_property,
            )),
            output: OutputFormat::JSON(),
            kwargs,
        };

        let json = self.get_json(&url_builder).await?;
        let table: PropertyTableResponse = serde_json::from_value(json)?;
        Ok(table.property_table.properties)
    }

    /// Fetch synonyms for compounds or substances.
    pub async fn get_synonyms(
        &self,
        identifiers: impl Into<Identifiers>,
        namespace: Namespace,
        kwargs: HashMap<String, String>,
    ) -> Result<Vec<PubChemInformation>> {
        let domain = match &namespace {
            Namespace::Substance(_) => Domain::Substance(),
            _ => Domain::Compound(),
        };

        let operation = match &domain {
            Domain::Substance() => {
                Operation::Substance(SubstanceOperationSpecification::Synonyms())
            }
            _ => Operation::Compound(CompoundOperationSpecification::Synonyms()),
        };

        let url_builder = UrlBuilder {
            input_specification: InputSpecification {
                domain,
                namespace,
                identifiers: identifiers.into(),
            },
            operation,
            output: OutputFormat::JSON(),
            kwargs,
        };

        let response = self.get_and_parse(&url_builder).await?;
        match response {
            PubChemResponse::InformationList(info_list) => {
                Ok(info_list.get_information_list())
            }
            _other => Err(crate::error::Error::PubChem(
                pubchemrs_struct::error::PubChemError::ParseResponseError(
                    "Expected InformationList response, got unexpected variant".into(),
                ),
            )),
        }
    }

    /// Fetch all source names for a given domain.
    ///
    /// If `domain` is `None`, defaults to substance sources.
    pub async fn get_all_sources(
        &self,
        domain: Option<Domain>,
    ) -> Result<Vec<String>> {
        let source_domain = match domain {
            Some(Domain::Assay()) => Domain::Others(DomainOtherInputs::SourcesAssays),
            _ => Domain::Others(DomainOtherInputs::SourcesSubstances),
        };

        let url_builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: source_domain,
                namespace: Namespace::None(),
                identifiers: Identifiers::default(),
            },
            operation: Operation::OtherInput(),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let response = self.get_and_parse(&url_builder).await?;
        match response {
            PubChemResponse::InformationList(PubChemInformationList::SourceName(names)) => {
                Ok(names)
            }
            _other => Err(crate::error::Error::PubChem(
                pubchemrs_struct::error::PubChemError::ParseResponseError(
                    "Expected SourceName list response, got unexpected variant".into(),
                ),
            )),
        }
    }
}
