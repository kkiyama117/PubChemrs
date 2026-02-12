use pubchemrs_tokio::{CompoundQuery, OtherInputsQuery, PubChemClient};

#[tokio::test]
#[ignore] // Requires network
async fn compound_query_molecular_formula_by_name() {
    let formula = CompoundQuery::with_name("aspirin")
        .molecular_formula()
        .await
        .unwrap();
    assert_eq!(formula, Some("C9H8O4".to_string()));
}

#[tokio::test]
#[ignore]
async fn compound_query_molecular_weight_by_cid() {
    let weight = CompoundQuery::with_cid(2244)
        .molecular_weight()
        .await
        .unwrap();
    assert!(weight.is_some());
    assert!((weight.unwrap() - 180.16).abs() < 0.1);
}

#[tokio::test]
#[ignore]
async fn compound_query_synonyms() {
    let synonyms = CompoundQuery::with_name("aspirin")
        .synonyms()
        .await
        .unwrap();
    assert!(!synonyms.is_empty());
    assert!(
        synonyms
            .iter()
            .any(|s| s.to_lowercase().contains("aspirin"))
    );
}

#[tokio::test]
#[ignore]
async fn compound_query_batch_properties() {
    let props = CompoundQuery::with_cids(&[2244, 962])
        .properties(&["MolecularFormula"])
        .await
        .unwrap();
    assert_eq!(props.len(), 2);
}

#[tokio::test]
#[ignore]
async fn compound_query_record() {
    let record = CompoundQuery::with_cid(2244).record().await.unwrap();
    assert!(record.cid.is_some());
}

#[tokio::test]
#[ignore]
async fn compound_query_smiles_lookup() {
    let formula = CompoundQuery::with_smiles("CC(=O)OC1=CC=CC=C1C(=O)O")
        .molecular_formula()
        .await
        .unwrap();
    assert_eq!(formula, Some("C9H8O4".to_string()));
}

#[tokio::test]
#[ignore]
async fn compound_query_cid_by_name() {
    let cid = CompoundQuery::with_name("aspirin").cid().await.unwrap();
    assert_eq!(cid, Some(2244));
}

#[tokio::test]
#[ignore]
async fn other_inputs_substance_sources() {
    let sources = OtherInputsQuery::substance_sources().fetch().await.unwrap();
    assert!(!sources.is_empty());
}

#[tokio::test]
#[ignore]
async fn compound_query_with_custom_client() {
    let client = PubChemClient::default();
    let formula = CompoundQuery::with_name("aspirin")
        .using_client(&client)
        .molecular_formula()
        .await
        .unwrap();
    assert_eq!(formula, Some("C9H8O4".to_string()));
}

#[tokio::test]
#[ignore]
async fn other_inputs_periodic_table_json() {
    let json = OtherInputsQuery::periodic_table()
        .fetch_json()
        .await
        .unwrap();
    assert!(json.is_object());
}
