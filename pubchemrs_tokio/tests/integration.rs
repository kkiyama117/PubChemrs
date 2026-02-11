use std::collections::HashMap;

use pubchemrs_struct::requests::input::*;
use pubchemrs_tokio::PubChemClient;

#[tokio::test]
#[ignore = "requires network access to PubChem API"]
async fn test_get_properties_water() {
    let client = PubChemClient::default();
    let props = client
        .get_properties(
            "water",
            CompoundNamespace::Name(),
            &["MolecularWeight".into(), "InChIKey".into()],
            HashMap::new(),
        )
        .await
        .unwrap();

    assert!(!props.is_empty());
    let water = &props[0];
    assert!(water.molecular_weight.is_some());
    assert!(water.inchikey.is_some());
    assert_eq!(
        water.inchikey.as_deref(),
        Some("XLYOFNOQVPJJNP-UHFFFAOYSA-N")
    );
}

#[tokio::test]
#[ignore = "requires network access to PubChem API"]
async fn test_get_compounds_aspirin() {
    let client = PubChemClient::default();
    let compounds = client
        .get_compounds(2244u32, CompoundNamespace::Cid(), HashMap::new())
        .await
        .unwrap();

    assert_eq!(compounds.len(), 1);
    let aspirin = &compounds[0];
    // CID 2244 is aspirin
    assert!(aspirin.cid.is_some());
}

#[tokio::test]
#[ignore = "requires network access to PubChem API"]
async fn test_get_synonyms_caffeine() {
    let client = PubChemClient::default();
    let synonyms = client
        .get_synonyms(
            "caffeine",
            Namespace::Compound(CompoundNamespace::Name()),
            HashMap::new(),
        )
        .await
        .unwrap();

    assert!(!synonyms.is_empty());
    let info = &synonyms[0];
    assert!(info.cid.is_some());
    assert!(!info.synonym.is_empty());
    // "caffeine" should appear in synonyms (case-insensitive)
    assert!(
        info.synonym
            .iter()
            .any(|s| s.to_lowercase().contains("caffeine"))
    );
}

#[tokio::test]
#[ignore = "requires network access to PubChem API"]
async fn test_get_all_sources_substance() {
    let client = PubChemClient::default();
    let sources = client.get_all_sources(None).await.unwrap();

    assert!(!sources.is_empty());
}
