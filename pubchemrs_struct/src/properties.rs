use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize a value that may be either a number or a string containing a number.
/// PubChem API returns some numeric fields (MolecularWeight, ExactMass, MonoisotopicMass)
/// as strings rather than numbers.
fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    struct StringOrNumber;

    impl<'de> de::Visitor<'de> for StringOrNumber {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or a string containing a number")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Some(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Some(v as f64))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Some(v as f64))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<f64>().map(Some).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(StringOrNumber)
}

/// Strongly-typed compound properties from the PubChem PropertyTable API.
///
/// This struct maps the JSON response from the PubChem
/// [`/compound/{namespace}/{id}/property/{props}/JSON`](https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest#section=Compound-Property-Tables)
/// endpoint into Rust types.
///
/// # Field Availability
///
/// All fields except [`cid`](CompoundProperties::cid) are `Option<T>` because the API only
/// returns the properties you request. Unrequested fields deserialize as `None`.
///
/// # Type Coercion
///
/// The PubChem API returns some numeric fields as JSON strings (notably `MolecularWeight`,
/// `ExactMass`, and `MonoisotopicMass`). These are automatically parsed into `f64` during
/// deserialization using a custom deserializer.
///
/// # Property Categories
///
/// | Category | Fields |
/// |----------|--------|
/// | **Identifiers** | `cid`, `inchi`, `inchikey`, `iupac_name` |
/// | **SMILES** | `smiles` (current), `connectivity_smiles` (current), `canonical_smiles` (legacy), `isomeric_smiles` (legacy) |
/// | **Physical** | `molecular_formula`, `molecular_weight`, `exact_mass`, `monoisotopic_mass`, `charge` |
/// | **Descriptors** | `xlogp`, `tpsa`, `complexity` |
/// | **Counts** | `h_bond_donor_count`, `h_bond_acceptor_count`, `rotatable_bond_count`, `heavy_atom_count`, `isotope_atom_count`, `covalent_unit_count` |
/// | **Stereochemistry** | `atom_stereo_count`, `defined_atom_stereo_count`, `undefined_atom_stereo_count`, `bond_stereo_count`, `defined_bond_stereo_count`, `undefined_bond_stereo_count` |
/// | **Fingerprint** | `fingerprint` (hex-encoded 881-bit PubChem fingerprint) |
/// | **3D** | `volume_3d`, `conformer_rmsd_3d`, `effective_rotor_count_3d`, `conformer_count_3d`, steric quadrupoles, pharmacophore feature counts |
///
/// # SMILES Variants
///
/// PubChem has [two current SMILES variants](https://pubchem.ncbi.nlm.nih.gov/docs/glossary#section=SMILES):
///
/// | Current Name | JSON Key | Rust field | Content |
/// |-------------|----------|------------|---------|
/// | **SMILES** | `SMILES` | `smiles` | Complete SMILES with stereochemistry and isotopes |
/// | **Connectivity SMILES** | `ConnectivitySMILES` | `connectivity_smiles` | Connectivity only, no stereochemistry or isotopes |
///
/// The former "Canonical SMILES" has been renamed to "Connectivity SMILES", and the former
/// "Isomeric SMILES" is now simply called "SMILES". When requesting the legacy property tags
/// (`CanonicalSMILES`, `IsomericSMILES`), the API may return the values under the new JSON
/// keys (`ConnectivitySMILES`, `SMILES`) instead. All four fields are provided for
/// compatibility with both old and new API responses.
///
/// # Example
///
/// ```rust
/// use pubchemrs_struct::properties::{PropertyTableResponse, CompoundProperties};
///
/// let json = r#"{"PropertyTable":{"Properties":[{"CID":962,"MolecularFormula":"H2O","InChIKey":"XLYOFNOQVPJJNP-UHFFFAOYSA-N"}]}}"#;
/// let resp: PropertyTableResponse = serde_json::from_str(json).unwrap();
/// let water = &resp.property_table.properties[0];
///
/// assert_eq!(water.cid, 962);
/// assert_eq!(water.molecular_formula.as_deref(), Some("H2O"));
/// assert!(water.molecular_weight.is_none()); // not requested
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(get_all))]
pub struct CompoundProperties {
    #[serde(rename = "CID")]
    pub cid: u64,

    #[serde(rename = "MolecularFormula", default)]
    pub molecular_formula: Option<String>,

    #[serde(
        rename = "MolecularWeight",
        default,
        deserialize_with = "deserialize_string_or_number"
    )]
    pub molecular_weight: Option<f64>,

    /// Complete SMILES including stereochemistry and isotope information.
    ///
    /// This is the current PubChem SMILES property (formerly "Isomeric SMILES").
    /// JSON key: `SMILES`. Request tag: `IsomericSMILES`.
    #[serde(rename = "SMILES", default)]
    pub smiles: Option<String>,

    /// Connectivity-only SMILES without stereochemistry or isotope information.
    /// Analogous to the connectivity layer of the InChI Key.
    ///
    /// This is the current PubChem property (formerly "Canonical SMILES").
    /// JSON key: `ConnectivitySMILES`. Request tag: `CanonicalSMILES`.
    #[serde(rename = "ConnectivitySMILES", default)]
    pub connectivity_smiles: Option<String>,

    /// Legacy field: may appear in older API responses or specific endpoints.
    /// Prefer [`smiles`](Self::smiles) and [`connectivity_smiles`](Self::connectivity_smiles).
    #[serde(rename = "CanonicalSMILES", default)]
    pub canonical_smiles: Option<String>,

    /// Legacy field: may appear in older API responses or specific endpoints.
    /// Prefer [`smiles`](Self::smiles) and [`connectivity_smiles`](Self::connectivity_smiles).
    #[serde(rename = "IsomericSMILES", default)]
    pub isomeric_smiles: Option<String>,

    #[serde(rename = "InChI", default)]
    pub inchi: Option<String>,

    #[serde(rename = "InChIKey", default)]
    pub inchikey: Option<String>,

    #[serde(rename = "IUPACName", default)]
    pub iupac_name: Option<String>,

    #[serde(rename = "XLogP", default)]
    pub xlogp: Option<f64>,

    #[serde(
        rename = "ExactMass",
        default,
        deserialize_with = "deserialize_string_or_number"
    )]
    pub exact_mass: Option<f64>,

    #[serde(
        rename = "MonoisotopicMass",
        default,
        deserialize_with = "deserialize_string_or_number"
    )]
    pub monoisotopic_mass: Option<f64>,

    #[serde(rename = "TPSA", default)]
    pub tpsa: Option<f64>,

    #[serde(rename = "Complexity", default)]
    pub complexity: Option<f64>,

    #[serde(rename = "Charge", default)]
    pub charge: Option<i32>,

    #[serde(rename = "HBondDonorCount", default)]
    pub h_bond_donor_count: Option<u32>,

    #[serde(rename = "HBondAcceptorCount", default)]
    pub h_bond_acceptor_count: Option<u32>,

    #[serde(rename = "RotatableBondCount", default)]
    pub rotatable_bond_count: Option<u32>,

    #[serde(rename = "Fingerprint2D", default)]
    pub fingerprint: Option<String>,

    #[serde(rename = "HeavyAtomCount", default)]
    pub heavy_atom_count: Option<u32>,

    #[serde(rename = "IsotopeAtomCount", default)]
    pub isotope_atom_count: Option<u32>,

    #[serde(rename = "AtomStereoCount", default)]
    pub atom_stereo_count: Option<u32>,

    #[serde(rename = "DefinedAtomStereoCount", default)]
    pub defined_atom_stereo_count: Option<u32>,

    #[serde(rename = "UndefinedAtomStereoCount", default)]
    pub undefined_atom_stereo_count: Option<u32>,

    #[serde(rename = "BondStereoCount", default)]
    pub bond_stereo_count: Option<u32>,

    #[serde(rename = "DefinedBondStereoCount", default)]
    pub defined_bond_stereo_count: Option<u32>,

    #[serde(rename = "UndefinedBondStereoCount", default)]
    pub undefined_bond_stereo_count: Option<u32>,

    #[serde(rename = "CovalentUnitCount", default)]
    pub covalent_unit_count: Option<u32>,

    #[serde(rename = "Volume3D", default)]
    pub volume_3d: Option<f64>,

    #[serde(rename = "ConformerModelRMSD3D", default)]
    pub conformer_rmsd_3d: Option<f64>,

    #[serde(rename = "EffectiveRotorCount3D", default)]
    pub effective_rotor_count_3d: Option<f64>,

    #[serde(rename = "ConformerCount3D", default)]
    pub conformer_count_3d: Option<u32>,

    #[serde(rename = "XStericQuadrupole3D", default)]
    pub x_steric_quadrupole_3d: Option<f64>,

    #[serde(rename = "YStericQuadrupole3D", default)]
    pub y_steric_quadrupole_3d: Option<f64>,

    #[serde(rename = "ZStericQuadrupole3D", default)]
    pub z_steric_quadrupole_3d: Option<f64>,

    #[serde(rename = "FeatureCount3D", default)]
    pub feature_count_3d: Option<u32>,

    #[serde(rename = "FeatureAcceptorCount3D", default)]
    pub feature_acceptor_count_3d: Option<u32>,

    #[serde(rename = "FeatureDonorCount3D", default)]
    pub feature_donor_count_3d: Option<u32>,

    #[serde(rename = "FeatureAnionCount3D", default)]
    pub feature_anion_count_3d: Option<u32>,

    #[serde(rename = "FeatureCationCount3D", default)]
    pub feature_cation_count_3d: Option<u32>,

    #[serde(rename = "FeatureRingCount3D", default)]
    pub feature_ring_count_3d: Option<u32>,

    #[serde(rename = "FeatureHydrophobeCount3D", default)]
    pub feature_hydrophobe_count_3d: Option<u32>,
}

/// Wrapper for the PubChem PropertyTable API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTableResponse {
    #[serde(rename = "PropertyTable")]
    pub property_table: PropertyTable,
}

/// Container for a list of compound properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTable {
    #[serde(rename = "Properties")]
    pub properties: Vec<CompoundProperties>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASPIRIN_FIXTURE: &str = include_str!("../tests/fixtures/aspirin_properties.json");

    #[test]
    fn test_deserialize_property_table_response() {
        let response: PropertyTableResponse =
            serde_json::from_str(ASPIRIN_FIXTURE).expect("should deserialize");
        assert_eq!(response.property_table.properties.len(), 1);
    }

    #[test]
    fn test_deserialize_compound_properties_cid() {
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let props = &response.property_table.properties[0];
        assert_eq!(props.cid, 2244);
    }

    #[test]
    fn test_deserialize_compound_properties_string_fields() {
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let props = &response.property_table.properties[0];
        assert_eq!(props.molecular_formula.as_deref(), Some("C9H8O4"));
        assert_eq!(
            props.inchi.as_deref(),
            Some("InChI=1S/C9H8O4/c1-6(10)13-8-5-3-2-4-7(8)9(11)12/h2-5H,1H3,(H,11,12)")
        );
        assert_eq!(
            props.inchikey.as_deref(),
            Some("BSYNRYMUTXBXSQ-UHFFFAOYSA-N")
        );
        assert_eq!(props.iupac_name.as_deref(), Some("2-acetyloxybenzoic acid"));
    }

    #[test]
    fn test_deserialize_smiles_all_variants() {
        // Aspirin fixture contains all 4 SMILES variants
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let props = &response.property_table.properties[0];

        // Current fields
        assert_eq!(props.smiles.as_deref(), Some("CC(=O)OC1=CC=CC=C1C(=O)O"));
        assert_eq!(
            props.connectivity_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
        // Legacy fields
        assert_eq!(
            props.canonical_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
        assert_eq!(
            props.isomeric_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
    }

    #[test]
    fn test_deserialize_smiles_current_fields_only() {
        // Modern API response with only current SMILES field names
        let json = r#"{
            "PropertyTable": {
                "Properties": [{
                    "CID": 2244,
                    "ConnectivitySMILES": "CC(=O)OC1=CC=CC=C1C(=O)O",
                    "SMILES": "CC(=O)OC1=CC=CC=C1C(=O)O"
                }]
            }
        }"#;
        let response: PropertyTableResponse =
            serde_json::from_str(json).expect("should handle current SMILES fields");
        let props = &response.property_table.properties[0];

        assert_eq!(
            props.connectivity_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
        assert_eq!(props.smiles.as_deref(), Some("CC(=O)OC1=CC=CC=C1C(=O)O"));
        // Legacy fields should be None
        assert!(props.canonical_smiles.is_none());
        assert!(props.isomeric_smiles.is_none());
    }

    #[test]
    fn test_deserialize_smiles_legacy_fields_only() {
        // Older API response with only legacy SMILES field names
        let json = r#"{
            "PropertyTable": {
                "Properties": [{
                    "CID": 2244,
                    "CanonicalSMILES": "CC(=O)OC1=CC=CC=C1C(=O)O",
                    "IsomericSMILES": "CC(=O)OC1=CC=CC=C1C(=O)O"
                }]
            }
        }"#;
        let response: PropertyTableResponse =
            serde_json::from_str(json).expect("should handle legacy SMILES fields");
        let props = &response.property_table.properties[0];

        assert_eq!(
            props.canonical_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
        assert_eq!(
            props.isomeric_smiles.as_deref(),
            Some("CC(=O)OC1=CC=CC=C1C(=O)O")
        );
        // Current fields should be None
        assert!(props.smiles.is_none());
        assert!(props.connectivity_smiles.is_none());
    }

    #[test]
    fn test_deserialize_compound_properties_numeric_fields() {
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let props = &response.property_table.properties[0];

        // f64 fields
        assert_eq!(props.xlogp, Some(1.2));
        assert!((props.tpsa.unwrap() - 63.6).abs() < 0.01);
        assert!((props.complexity.unwrap() - 212.0).abs() < 0.01);

        // MolecularWeight comes as string from API - should parse to f64
        assert!((props.molecular_weight.unwrap() - 180.16).abs() < 0.01);
        assert!((props.exact_mass.unwrap() - 180.04225873).abs() < 1e-6);
        assert!((props.monoisotopic_mass.unwrap() - 180.04225873).abs() < 1e-6);
    }

    #[test]
    fn test_deserialize_compound_properties_integer_fields() {
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let props = &response.property_table.properties[0];

        assert_eq!(props.charge, Some(0));
        assert_eq!(props.h_bond_donor_count, Some(1));
        assert_eq!(props.h_bond_acceptor_count, Some(4));
        assert_eq!(props.rotatable_bond_count, Some(3));
        assert_eq!(props.heavy_atom_count, Some(13));
        assert_eq!(props.isotope_atom_count, Some(0));
        assert_eq!(props.atom_stereo_count, Some(0));
        assert_eq!(props.defined_atom_stereo_count, Some(0));
        assert_eq!(props.undefined_atom_stereo_count, Some(0));
        assert_eq!(props.bond_stereo_count, Some(0));
        assert_eq!(props.defined_bond_stereo_count, Some(0));
        assert_eq!(props.undefined_bond_stereo_count, Some(0));
        assert_eq!(props.covalent_unit_count, Some(1));
    }

    #[test]
    fn test_deserialize_partial_properties() {
        // API may return only requested fields; others should be None
        let json = r#"{
            "PropertyTable": {
                "Properties": [{
                    "CID": 962,
                    "MolecularFormula": "H2O",
                    "InChIKey": "XLYOFNOQVPJJNP-UHFFFAOYSA-N"
                }]
            }
        }"#;
        let response: PropertyTableResponse =
            serde_json::from_str(json).expect("should handle partial properties");
        let props = &response.property_table.properties[0];
        assert_eq!(props.cid, 962);
        assert_eq!(props.molecular_formula.as_deref(), Some("H2O"));
        assert_eq!(
            props.inchikey.as_deref(),
            Some("XLYOFNOQVPJJNP-UHFFFAOYSA-N")
        );
        // All unreturned fields should be None
        assert!(props.molecular_weight.is_none());
        assert!(props.canonical_smiles.is_none());
        assert!(props.xlogp.is_none());
        assert!(props.charge.is_none());
    }

    #[test]
    fn test_serialize_roundtrip() {
        let response: PropertyTableResponse = serde_json::from_str(ASPIRIN_FIXTURE).unwrap();
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: PropertyTableResponse = serde_json::from_str(&serialized).unwrap();
        let props = &deserialized.property_table.properties[0];
        assert_eq!(props.cid, 2244);
        assert_eq!(
            props.inchikey.as_deref(),
            Some("BSYNRYMUTXBXSQ-UHFFFAOYSA-N")
        );
    }
}
