use std::{fmt::Display, str::FromStr};

/// A list of compound property tags to retrieve from the PubChem API.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct CompoundProperty(
    /// The list of property tag names (e.g., `MolecularFormula`, `MolecularWeight`).
    pub Vec<CompoundPropertyTag>,
);

impl CompoundProperty {
    /// Returns `true` if no property tags are present or all are empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|inner| inner.is_empty())
    }

    /// Formats the property tags as a comma-separated string for use in the URL path.
    pub fn to_url_string(&self) -> String {
        self.0
            .iter()
            .map(|inner| inner.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Display for CompoundProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_url_string().fmt(f)
    }
}

impl FromStr for CompoundProperty {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tags: Vec<CompoundPropertyTag> = s.split(',').map(|t| t.to_string()).collect();
        if tags.is_empty() || tags.iter().all(|t| t.is_empty()) {
            Err(crate::error::ParseEnumError::VariantNotFound)
        } else {
            Ok(Self(tags))
        }
    }
}

impl FromIterator<CompoundPropertyTag> for CompoundProperty {
    fn from_iter<T: IntoIterator<Item = CompoundPropertyTag>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<I: Into<CompoundPropertyTag>> From<I> for CompoundProperty {
    fn from(value: I) -> Self {
        Self(vec![value.into()])
    }
}

/// A compound property tag name (e.g., `MolecularFormula`, `MolecularWeight`, `IUPACName`).
pub type CompoundPropertyTag = String;

/// Mapping from snake_case Python-style property names to PubChem API PascalCase keys.
///
/// This mapping is compatible with the legacy `pubchempy` library's `PROPERTY_MAP`.
/// Unknown property names are passed through unchanged for forward compatibility.
///
/// Each entry is `(snake_case_name, PascalCaseApiKey)`.
pub static PROPERTY_MAP: &[(&str, &str)] = &[
    ("molecular_formula", "MolecularFormula"),
    ("molecular_weight", "MolecularWeight"),
    ("smiles", "SMILES"),
    ("connectivity_smiles", "ConnectivitySMILES"),
    ("canonical_smiles", "CanonicalSMILES"),
    ("isomeric_smiles", "IsomericSMILES"),
    ("inchi", "InChI"),
    ("inchikey", "InChIKey"),
    ("iupac_name", "IUPACName"),
    ("xlogp", "XLogP"),
    ("exact_mass", "ExactMass"),
    ("monoisotopic_mass", "MonoisotopicMass"),
    ("tpsa", "TPSA"),
    ("complexity", "Complexity"),
    ("charge", "Charge"),
    ("h_bond_donor_count", "HBondDonorCount"),
    ("h_bond_acceptor_count", "HBondAcceptorCount"),
    ("rotatable_bond_count", "RotatableBondCount"),
    ("heavy_atom_count", "HeavyAtomCount"),
    ("isotope_atom_count", "IsotopeAtomCount"),
    ("atom_stereo_count", "AtomStereoCount"),
    ("defined_atom_stereo_count", "DefinedAtomStereoCount"),
    ("undefined_atom_stereo_count", "UndefinedAtomStereoCount"),
    ("bond_stereo_count", "BondStereoCount"),
    ("defined_bond_stereo_count", "DefinedBondStereoCount"),
    ("undefined_bond_stereo_count", "UndefinedBondStereoCount"),
    ("covalent_unit_count", "CovalentUnitCount"),
    ("volume_3d", "Volume3D"),
    ("conformer_rmsd_3d", "ConformerModelRMSD3D"),
    ("conformer_model_rmsd_3d", "ConformerModelRMSD3D"),
    ("x_steric_quadrupole_3d", "XStericQuadrupole3D"),
    ("y_steric_quadrupole_3d", "YStericQuadrupole3D"),
    ("z_steric_quadrupole_3d", "ZStericQuadrupole3D"),
    ("feature_count_3d", "FeatureCount3D"),
    ("feature_acceptor_count_3d", "FeatureAcceptorCount3D"),
    ("feature_donor_count_3d", "FeatureDonorCount3D"),
    ("feature_anion_count_3d", "FeatureAnionCount3D"),
    ("feature_cation_count_3d", "FeatureCationCount3D"),
    ("feature_ring_count_3d", "FeatureRingCount3D"),
    ("feature_hydrophobe_count_3d", "FeatureHydrophobeCount3D"),
    ("effective_rotor_count_3d", "EffectiveRotorCount3D"),
    ("conformer_count_3d", "ConformerCount3D"),
    ("fingerprint", "Fingerprint2D"),
];

/// Normalize a single property tag from snake_case to PascalCase PubChem API key.
///
/// If the tag matches a known snake_case key in [`PROPERTY_MAP`], returns the
/// corresponding PascalCase API key. Otherwise, returns the input unchanged
/// (forward compatibility with new/unknown properties).
///
/// # Examples
///
/// ```
/// use pubchemrs_struct::requests::operation::normalize_property_tag;
///
/// assert_eq!(normalize_property_tag("molecular_weight"), "MolecularWeight");
/// assert_eq!(normalize_property_tag("MolecularWeight"), "MolecularWeight");
/// assert_eq!(normalize_property_tag("UnknownProp"), "UnknownProp");
/// ```
pub fn normalize_property_tag(tag: &str) -> String {
    PROPERTY_MAP
        .iter()
        .find(|(snake, _)| *snake == tag)
        .map_or_else(|| tag.to_string(), |(_, pascal)| (*pascal).to_string())
}

/// Normalize a slice of property tags from snake_case to PascalCase PubChem API keys.
///
/// Applies [`normalize_property_tag`] to each tag, returning a new `Vec`.
pub fn normalize_property_tags(tags: &[String]) -> Vec<String> {
    tags.iter().map(|t| normalize_property_tag(t)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_snake_case() {
        assert_eq!(
            normalize_property_tag("molecular_weight"),
            "MolecularWeight"
        );
        assert_eq!(normalize_property_tag("smiles"), "SMILES");
        assert_eq!(normalize_property_tag("inchi"), "InChI");
        assert_eq!(normalize_property_tag("inchikey"), "InChIKey");
        assert_eq!(normalize_property_tag("iupac_name"), "IUPACName");
        assert_eq!(normalize_property_tag("xlogp"), "XLogP");
        assert_eq!(normalize_property_tag("tpsa"), "TPSA");
        assert_eq!(
            normalize_property_tag("h_bond_donor_count"),
            "HBondDonorCount"
        );
        assert_eq!(normalize_property_tag("volume_3d"), "Volume3D");
        assert_eq!(normalize_property_tag("fingerprint"), "Fingerprint2D");
    }

    #[test]
    fn test_normalize_pascal_case_passthrough() {
        assert_eq!(normalize_property_tag("MolecularWeight"), "MolecularWeight");
        assert_eq!(normalize_property_tag("SMILES"), "SMILES");
        assert_eq!(normalize_property_tag("InChIKey"), "InChIKey");
        assert_eq!(normalize_property_tag("Fingerprint2D"), "Fingerprint2D");
    }

    #[test]
    fn test_normalize_unknown_passthrough() {
        assert_eq!(normalize_property_tag("FutureProperty"), "FutureProperty");
        assert_eq!(normalize_property_tag("SomeNewField"), "SomeNewField");
    }

    #[test]
    fn test_normalize_alias() {
        assert_eq!(
            normalize_property_tag("conformer_rmsd_3d"),
            "ConformerModelRMSD3D"
        );
        assert_eq!(
            normalize_property_tag("conformer_model_rmsd_3d"),
            "ConformerModelRMSD3D"
        );
    }

    #[test]
    fn test_property_map_count() {
        // 42 legacy entries + 1 new (fingerprint) = 43
        assert_eq!(PROPERTY_MAP.len(), 43);
    }

    #[test]
    fn test_normalize_batch() {
        let tags = vec![
            "molecular_formula".to_string(),
            "MolecularWeight".to_string(),
            "unknown_prop".to_string(),
        ];
        let normalized = normalize_property_tags(&tags);
        assert_eq!(
            normalized,
            vec!["MolecularFormula", "MolecularWeight", "unknown_prop"]
        );
    }

    #[test]
    fn test_property_map_no_duplicate_keys() {
        let mut keys: Vec<&str> = PROPERTY_MAP.iter().map(|(k, _)| *k).collect();
        keys.sort();
        let len_before = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), len_before, "PROPERTY_MAP has duplicate keys");
    }

    #[test]
    fn test_compound_property_parse_preserves_tags() {
        let prop: CompoundProperty = "MolecularFormula,MolecularWeight".parse().unwrap();
        assert_eq!(prop.0.len(), 2);
        assert_eq!(prop.0[0], "MolecularFormula");
        assert_eq!(prop.0[1], "MolecularWeight");
    }
}
