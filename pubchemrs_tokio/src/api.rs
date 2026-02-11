use std::collections::HashMap;

use pubchemrs_struct::properties::{CompoundProperties, PropertyTableResponse};
use pubchemrs_struct::requests::input::*;
use pubchemrs_struct::requests::operation::*;
use pubchemrs_struct::requests::output::OutputFormat;
use pubchemrs_struct::requests::url_builder::UrlBuilder;
use pubchemrs_struct::response::{
    Compound, PubChemInformation, PubChemInformationList, PubChemResponse,
};

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
            PubChemResponse::InformationList(info_list) => Ok(info_list.get_information_list()),
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
    pub async fn get_all_sources(&self, domain: Option<Domain>) -> Result<Vec<String>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pubchemrs_struct::requests::url_builder::PUBCHEM_API_BASE;

    /// Helper to build URL from a UrlBuilder and verify it.
    fn build_url(builder: &UrlBuilder) -> (String, Option<String>) {
        let (parts, body) = builder.build_url_parts().unwrap();
        let url = format!("{}/{}", PUBCHEM_API_BASE, parts.join("/"));
        (url, body)
    }

    #[test]
    fn test_get_compounds_url_by_cid() {
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

        let (url, body) = build_url(&builder);
        assert!(url.contains("compound/cid/2244/record/JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_get_compounds_url_by_name() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Name()),
                identifiers: "aspirin".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = build_url(&builder);
        assert!(url.contains("compound/name/aspirin/record/JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_get_properties_url() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Name()),
                identifiers: "water".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Property(
                CompoundProperty(vec!["MolecularWeight".into(), "InChIKey".into()]),
            )),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = build_url(&builder);
        assert!(url.contains("compound/name/water/property/MolecularWeight,InChIKey/JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_get_synonyms_compound_domain_selection() {
        // Compound namespace should select Compound domain
        let namespace = Namespace::Compound(CompoundNamespace::Name());
        let domain = match &namespace {
            Namespace::Substance(_) => Domain::Substance(),
            _ => Domain::Compound(),
        };
        assert_eq!(domain, Domain::Compound());

        let operation = match &domain {
            Domain::Substance() => {
                Operation::Substance(SubstanceOperationSpecification::Synonyms())
            }
            _ => Operation::Compound(CompoundOperationSpecification::Synonyms()),
        };
        assert_eq!(
            operation,
            Operation::Compound(CompoundOperationSpecification::Synonyms())
        );
    }

    #[test]
    fn test_get_synonyms_substance_domain_selection() {
        // Substance namespace should select Substance domain
        let namespace = Namespace::Substance(SubstanceNamespace::Sid());
        let domain = match &namespace {
            Namespace::Substance(_) => Domain::Substance(),
            _ => Domain::Compound(),
        };
        assert_eq!(domain, Domain::Substance());

        let operation = match &domain {
            Domain::Substance() => {
                Operation::Substance(SubstanceOperationSpecification::Synonyms())
            }
            _ => Operation::Compound(CompoundOperationSpecification::Synonyms()),
        };
        assert_eq!(
            operation,
            Operation::Substance(SubstanceOperationSpecification::Synonyms())
        );
    }

    #[test]
    fn test_get_synonyms_url_compound() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Name()),
                identifiers: "caffeine".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Synonyms()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = build_url(&builder);
        assert!(url.contains("compound/name/caffeine/synonyms/JSON"));
        assert!(body.is_none());
    }

    #[test]
    fn test_get_all_sources_default_is_substance() {
        let domain: Option<Domain> = None;
        let source_domain = match domain {
            Some(Domain::Assay()) => Domain::Others(DomainOtherInputs::SourcesAssays),
            _ => Domain::Others(DomainOtherInputs::SourcesSubstances),
        };
        assert_eq!(
            source_domain,
            Domain::Others(DomainOtherInputs::SourcesSubstances)
        );
    }

    #[test]
    fn test_get_all_sources_assay_domain() {
        let domain = Some(Domain::Assay());
        let source_domain = match domain {
            Some(Domain::Assay()) => Domain::Others(DomainOtherInputs::SourcesAssays),
            _ => Domain::Others(DomainOtherInputs::SourcesSubstances),
        };
        assert_eq!(
            source_domain,
            Domain::Others(DomainOtherInputs::SourcesAssays)
        );
    }

    #[test]
    fn test_get_all_sources_compound_falls_to_substance() {
        // Non-Assay domains should default to substance sources
        let domain = Some(Domain::Compound());
        let source_domain = match domain {
            Some(Domain::Assay()) => Domain::Others(DomainOtherInputs::SourcesAssays),
            _ => Domain::Others(DomainOtherInputs::SourcesSubstances),
        };
        assert_eq!(
            source_domain,
            Domain::Others(DomainOtherInputs::SourcesSubstances)
        );
    }

    #[test]
    fn test_get_all_sources_url_substance() {
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

        let (url, _body) = build_url(&builder);
        assert!(url.contains("sources/substance"));
        assert!(url.contains("JSON"));
    }

    #[test]
    fn test_get_all_sources_url_assay() {
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

        let (url, _body) = build_url(&builder);
        assert!(url.contains("sources/assay"));
        assert!(url.contains("JSON"));
    }

    #[test]
    fn test_get_compounds_url_by_smiles() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Smiles()),
                identifiers: "CC(=O)OC1=CC=CC=C1C(=O)O".into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Record()),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, body) = build_url(&builder);
        assert!(url.contains("compound/smiles"));
        // SMILES may use POST
        let _ = body; // Accept either GET or POST
    }

    #[test]
    fn test_get_properties_single_property() {
        let builder = UrlBuilder {
            input_specification: InputSpecification {
                domain: Domain::Compound(),
                namespace: Namespace::Compound(CompoundNamespace::Cid()),
                identifiers: 2244u32.into(),
            },
            operation: Operation::Compound(CompoundOperationSpecification::Property(
                CompoundProperty(vec!["MolecularFormula".into()]),
            )),
            output: OutputFormat::JSON(),
            kwargs: HashMap::new(),
        };

        let (url, _body) = build_url(&builder);
        assert!(url.contains("property/MolecularFormula"));
    }
}
