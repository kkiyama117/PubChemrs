use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt::{self, Display};
use std::str::FromStr;

/// A strongly-typed compound property tag for PubChem API queries.
///
/// Each variant corresponds to a known PubChem compound property.
/// [`Other`](CompoundPropertyTag::Other) provides forward compatibility
/// for new/unknown properties.
///
/// # Conversions
///
/// - [`Display`] outputs the API name (e.g. `"MolecularWeight"`, `"SMILES"`, `"InChI"`)
/// - [`FromStr`] accepts API names, snake_case, and known aliases; unknown
///   strings become [`Other`](CompoundPropertyTag::Other) (never fails)
/// - [`From<&str>`] and [`From<String>`] delegate to [`FromStr`]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompoundPropertyTag {
    /// Molecular formula (e.g. `"C9H8O4"`). API name: `MolecularFormula`.
    MolecularFormula,
    /// Molecular weight. API name: `MolecularWeight`.
    MolecularWeight,
    /// Full SMILES (with stereochemistry). API name: `SMILES`.
    Smiles,
    /// Connectivity-only SMILES. API name: `ConnectivitySMILES`.
    ConnectivitySmiles,
    /// Legacy canonical SMILES. API name: `CanonicalSMILES`.
    CanonicalSmiles,
    /// Legacy isomeric SMILES. API name: `IsomericSMILES`.
    IsomericSmiles,
    /// InChI identifier. API name: `InChI`.
    InChI,
    /// InChI hash key. API name: `InChIKey`.
    InChIKey,
    /// IUPAC systematic name. API name: `IUPACName`.
    IupacName,
    /// Octanol-water partition coefficient. API name: `XLogP`.
    XLogP,
    /// Exact mass. API name: `ExactMass`.
    ExactMass,
    /// Monoisotopic mass. API name: `MonoisotopicMass`.
    MonoisotopicMass,
    /// Topological polar surface area. API name: `TPSA`.
    Tpsa,
    /// Molecular complexity. API name: `Complexity`.
    Complexity,
    /// Formal charge. API name: `Charge`.
    Charge,
    /// Hydrogen bond donor count. API name: `HBondDonorCount`.
    HBondDonorCount,
    /// Hydrogen bond acceptor count. API name: `HBondAcceptorCount`.
    HBondAcceptorCount,
    /// Rotatable bond count. API name: `RotatableBondCount`.
    RotatableBondCount,
    /// Heavy (non-hydrogen) atom count. API name: `HeavyAtomCount`.
    HeavyAtomCount,
    /// Isotope atom count. API name: `IsotopeAtomCount`.
    IsotopeAtomCount,
    /// Atom stereo center count. API name: `AtomStereoCount`.
    AtomStereoCount,
    /// Defined atom stereo center count. API name: `DefinedAtomStereoCount`.
    DefinedAtomStereoCount,
    /// Undefined atom stereo center count. API name: `UndefinedAtomStereoCount`.
    UndefinedAtomStereoCount,
    /// Bond stereo center count. API name: `BondStereoCount`.
    BondStereoCount,
    /// Defined bond stereo center count. API name: `DefinedBondStereoCount`.
    DefinedBondStereoCount,
    /// Undefined bond stereo center count. API name: `UndefinedBondStereoCount`.
    UndefinedBondStereoCount,
    /// Covalent unit count. API name: `CovalentUnitCount`.
    CovalentUnitCount,
    /// 3D volume. API name: `Volume3D`.
    Volume3D,
    /// 3D conformer model RMSD. API name: `ConformerModelRMSD3D`.
    ConformerModelRmsd3D,
    /// 3D X steric quadrupole. API name: `XStericQuadrupole3D`.
    XStericQuadrupole3D,
    /// 3D Y steric quadrupole. API name: `YStericQuadrupole3D`.
    YStericQuadrupole3D,
    /// 3D Z steric quadrupole. API name: `ZStericQuadrupole3D`.
    ZStericQuadrupole3D,
    /// 3D pharmacophore feature count. API name: `FeatureCount3D`.
    FeatureCount3D,
    /// 3D acceptor feature count. API name: `FeatureAcceptorCount3D`.
    FeatureAcceptorCount3D,
    /// 3D donor feature count. API name: `FeatureDonorCount3D`.
    FeatureDonorCount3D,
    /// 3D anion feature count. API name: `FeatureAnionCount3D`.
    FeatureAnionCount3D,
    /// 3D cation feature count. API name: `FeatureCationCount3D`.
    FeatureCationCount3D,
    /// 3D ring feature count. API name: `FeatureRingCount3D`.
    FeatureRingCount3D,
    /// 3D hydrophobe feature count. API name: `FeatureHydrophobeCount3D`.
    FeatureHydrophobeCount3D,
    /// 3D effective rotor count. API name: `EffectiveRotorCount3D`.
    EffectiveRotorCount3D,
    /// 3D conformer count. API name: `ConformerCount3D`.
    ConformerCount3D,
    /// 2D fingerprint. API name: `Fingerprint2D`.
    Fingerprint2D,
    /// Unknown or future property (forward compatibility).
    Other(String),
}

impl CompoundPropertyTag {
    /// Returns the canonical snake_case name for this property tag.
    ///
    /// For known variants this returns a borrowed `&'static str` in a [`Cow`];
    /// for [`Other`](Self::Other) it returns the stored string as-is.
    pub fn snake_case_name(&self) -> Cow<'_, str> {
        match self {
            Self::MolecularFormula => Cow::Borrowed("molecular_formula"),
            Self::MolecularWeight => Cow::Borrowed("molecular_weight"),
            Self::Smiles => Cow::Borrowed("smiles"),
            Self::ConnectivitySmiles => Cow::Borrowed("connectivity_smiles"),
            Self::CanonicalSmiles => Cow::Borrowed("canonical_smiles"),
            Self::IsomericSmiles => Cow::Borrowed("isomeric_smiles"),
            Self::InChI => Cow::Borrowed("inchi"),
            Self::InChIKey => Cow::Borrowed("inchikey"),
            Self::IupacName => Cow::Borrowed("iupac_name"),
            Self::XLogP => Cow::Borrowed("xlogp"),
            Self::ExactMass => Cow::Borrowed("exact_mass"),
            Self::MonoisotopicMass => Cow::Borrowed("monoisotopic_mass"),
            Self::Tpsa => Cow::Borrowed("tpsa"),
            Self::Complexity => Cow::Borrowed("complexity"),
            Self::Charge => Cow::Borrowed("charge"),
            Self::HBondDonorCount => Cow::Borrowed("h_bond_donor_count"),
            Self::HBondAcceptorCount => Cow::Borrowed("h_bond_acceptor_count"),
            Self::RotatableBondCount => Cow::Borrowed("rotatable_bond_count"),
            Self::HeavyAtomCount => Cow::Borrowed("heavy_atom_count"),
            Self::IsotopeAtomCount => Cow::Borrowed("isotope_atom_count"),
            Self::AtomStereoCount => Cow::Borrowed("atom_stereo_count"),
            Self::DefinedAtomStereoCount => Cow::Borrowed("defined_atom_stereo_count"),
            Self::UndefinedAtomStereoCount => Cow::Borrowed("undefined_atom_stereo_count"),
            Self::BondStereoCount => Cow::Borrowed("bond_stereo_count"),
            Self::DefinedBondStereoCount => Cow::Borrowed("defined_bond_stereo_count"),
            Self::UndefinedBondStereoCount => Cow::Borrowed("undefined_bond_stereo_count"),
            Self::CovalentUnitCount => Cow::Borrowed("covalent_unit_count"),
            Self::Volume3D => Cow::Borrowed("volume_3d"),
            Self::ConformerModelRmsd3D => Cow::Borrowed("conformer_model_rmsd_3d"),
            Self::XStericQuadrupole3D => Cow::Borrowed("x_steric_quadrupole_3d"),
            Self::YStericQuadrupole3D => Cow::Borrowed("y_steric_quadrupole_3d"),
            Self::ZStericQuadrupole3D => Cow::Borrowed("z_steric_quadrupole_3d"),
            Self::FeatureCount3D => Cow::Borrowed("feature_count_3d"),
            Self::FeatureAcceptorCount3D => Cow::Borrowed("feature_acceptor_count_3d"),
            Self::FeatureDonorCount3D => Cow::Borrowed("feature_donor_count_3d"),
            Self::FeatureAnionCount3D => Cow::Borrowed("feature_anion_count_3d"),
            Self::FeatureCationCount3D => Cow::Borrowed("feature_cation_count_3d"),
            Self::FeatureRingCount3D => Cow::Borrowed("feature_ring_count_3d"),
            Self::FeatureHydrophobeCount3D => Cow::Borrowed("feature_hydrophobe_count_3d"),
            Self::EffectiveRotorCount3D => Cow::Borrowed("effective_rotor_count_3d"),
            Self::ConformerCount3D => Cow::Borrowed("conformer_count_3d"),
            Self::Fingerprint2D => Cow::Borrowed("fingerprint_2d"),
            Self::Other(s) => Cow::Borrowed(s.as_str()),
        }
    }

    /// Returns `true` only for `Other("")`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Other(s) if s.is_empty())
    }

    /// Returns an iterator over all known (non-[`Other`](Self::Other)) variants.
    pub fn variants() -> impl Iterator<Item = CompoundPropertyTag> {
        [
            Self::MolecularFormula,
            Self::MolecularWeight,
            Self::Smiles,
            Self::ConnectivitySmiles,
            Self::CanonicalSmiles,
            Self::IsomericSmiles,
            Self::InChI,
            Self::InChIKey,
            Self::IupacName,
            Self::XLogP,
            Self::ExactMass,
            Self::MonoisotopicMass,
            Self::Tpsa,
            Self::Complexity,
            Self::Charge,
            Self::HBondDonorCount,
            Self::HBondAcceptorCount,
            Self::RotatableBondCount,
            Self::HeavyAtomCount,
            Self::IsotopeAtomCount,
            Self::AtomStereoCount,
            Self::DefinedAtomStereoCount,
            Self::UndefinedAtomStereoCount,
            Self::BondStereoCount,
            Self::DefinedBondStereoCount,
            Self::UndefinedBondStereoCount,
            Self::CovalentUnitCount,
            Self::Volume3D,
            Self::ConformerModelRmsd3D,
            Self::XStericQuadrupole3D,
            Self::YStericQuadrupole3D,
            Self::ZStericQuadrupole3D,
            Self::FeatureCount3D,
            Self::FeatureAcceptorCount3D,
            Self::FeatureDonorCount3D,
            Self::FeatureAnionCount3D,
            Self::FeatureCationCount3D,
            Self::FeatureRingCount3D,
            Self::FeatureHydrophobeCount3D,
            Self::EffectiveRotorCount3D,
            Self::ConformerCount3D,
            Self::Fingerprint2D,
        ]
        .into_iter()
    }
}

/// Try to parse a string as a known [`CompoundPropertyTag`] variant.
///
/// Returns `None` for unrecognised strings (which become [`Other`](CompoundPropertyTag::Other)
/// in [`FromStr`] / [`From`] conversions).
fn parse_known(s: &str) -> Option<CompoundPropertyTag> {
    use CompoundPropertyTag::*;
    Some(match s {
        // API names
        "MolecularFormula" | "molecular_formula" => MolecularFormula,
        "MolecularWeight" | "molecular_weight" => MolecularWeight,
        "SMILES" | "smiles" => Smiles,
        "ConnectivitySMILES" | "connectivity_smiles" => ConnectivitySmiles,
        "CanonicalSMILES" | "canonical_smiles" => CanonicalSmiles,
        "IsomericSMILES" | "isomeric_smiles" => IsomericSmiles,
        "InChI" | "inchi" => InChI,
        "InChIKey" | "inchikey" => InChIKey,
        "IUPACName" | "iupac_name" => IupacName,
        "XLogP" | "xlogp" => XLogP,
        "ExactMass" | "exact_mass" => ExactMass,
        "MonoisotopicMass" | "monoisotopic_mass" => MonoisotopicMass,
        "TPSA" | "tpsa" => Tpsa,
        "Complexity" | "complexity" => Complexity,
        "Charge" | "charge" => Charge,
        "HBondDonorCount" | "h_bond_donor_count" => HBondDonorCount,
        "HBondAcceptorCount" | "h_bond_acceptor_count" => HBondAcceptorCount,
        "RotatableBondCount" | "rotatable_bond_count" => RotatableBondCount,
        "HeavyAtomCount" | "heavy_atom_count" => HeavyAtomCount,
        "IsotopeAtomCount" | "isotope_atom_count" => IsotopeAtomCount,
        "AtomStereoCount" | "atom_stereo_count" => AtomStereoCount,
        "DefinedAtomStereoCount" | "defined_atom_stereo_count" => DefinedAtomStereoCount,
        "UndefinedAtomStereoCount" | "undefined_atom_stereo_count" => UndefinedAtomStereoCount,
        "BondStereoCount" | "bond_stereo_count" => BondStereoCount,
        "DefinedBondStereoCount" | "defined_bond_stereo_count" => DefinedBondStereoCount,
        "UndefinedBondStereoCount" | "undefined_bond_stereo_count" => UndefinedBondStereoCount,
        "CovalentUnitCount" | "covalent_unit_count" => CovalentUnitCount,
        "Volume3D" | "volume_3d" => Volume3D,
        "ConformerModelRMSD3D" | "conformer_model_rmsd_3d" => ConformerModelRmsd3D,
        "XStericQuadrupole3D" | "x_steric_quadrupole_3d" => XStericQuadrupole3D,
        "YStericQuadrupole3D" | "y_steric_quadrupole_3d" => YStericQuadrupole3D,
        "ZStericQuadrupole3D" | "z_steric_quadrupole_3d" => ZStericQuadrupole3D,
        "FeatureCount3D" | "feature_count_3d" => FeatureCount3D,
        "FeatureAcceptorCount3D" | "feature_acceptor_count_3d" => FeatureAcceptorCount3D,
        "FeatureDonorCount3D" | "feature_donor_count_3d" => FeatureDonorCount3D,
        "FeatureAnionCount3D" | "feature_anion_count_3d" => FeatureAnionCount3D,
        "FeatureCationCount3D" | "feature_cation_count_3d" => FeatureCationCount3D,
        "FeatureRingCount3D" | "feature_ring_count_3d" => FeatureRingCount3D,
        "FeatureHydrophobeCount3D" | "feature_hydrophobe_count_3d" => FeatureHydrophobeCount3D,
        "EffectiveRotorCount3D" | "effective_rotor_count_3d" => EffectiveRotorCount3D,
        "ConformerCount3D" | "conformer_count_3d" => ConformerCount3D,
        "Fingerprint2D" | "fingerprint_2d" => Fingerprint2D,
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Trait implementations
// ---------------------------------------------------------------------------

/// Outputs the PubChem API name for this property tag.
///
/// The format varies by property (e.g. `"MolecularWeight"`, `"SMILES"`,
/// `"InChI"`, `"TPSA"`). For [`Other`](CompoundPropertyTag::Other) it
/// outputs the stored string as-is.
impl Display for CompoundPropertyTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MolecularFormula => f.write_str("MolecularFormula"),
            Self::MolecularWeight => f.write_str("MolecularWeight"),
            Self::Smiles => f.write_str("SMILES"),
            Self::ConnectivitySmiles => f.write_str("ConnectivitySMILES"),
            Self::CanonicalSmiles => f.write_str("CanonicalSMILES"),
            Self::IsomericSmiles => f.write_str("IsomericSMILES"),
            Self::InChI => f.write_str("InChI"),
            Self::InChIKey => f.write_str("InChIKey"),
            Self::IupacName => f.write_str("IUPACName"),
            Self::XLogP => f.write_str("XLogP"),
            Self::ExactMass => f.write_str("ExactMass"),
            Self::MonoisotopicMass => f.write_str("MonoisotopicMass"),
            Self::Tpsa => f.write_str("TPSA"),
            Self::Complexity => f.write_str("Complexity"),
            Self::Charge => f.write_str("Charge"),
            Self::HBondDonorCount => f.write_str("HBondDonorCount"),
            Self::HBondAcceptorCount => f.write_str("HBondAcceptorCount"),
            Self::RotatableBondCount => f.write_str("RotatableBondCount"),
            Self::HeavyAtomCount => f.write_str("HeavyAtomCount"),
            Self::IsotopeAtomCount => f.write_str("IsotopeAtomCount"),
            Self::AtomStereoCount => f.write_str("AtomStereoCount"),
            Self::DefinedAtomStereoCount => f.write_str("DefinedAtomStereoCount"),
            Self::UndefinedAtomStereoCount => f.write_str("UndefinedAtomStereoCount"),
            Self::BondStereoCount => f.write_str("BondStereoCount"),
            Self::DefinedBondStereoCount => f.write_str("DefinedBondStereoCount"),
            Self::UndefinedBondStereoCount => f.write_str("UndefinedBondStereoCount"),
            Self::CovalentUnitCount => f.write_str("CovalentUnitCount"),
            Self::Volume3D => f.write_str("Volume3D"),
            Self::ConformerModelRmsd3D => f.write_str("ConformerModelRMSD3D"),
            Self::XStericQuadrupole3D => f.write_str("XStericQuadrupole3D"),
            Self::YStericQuadrupole3D => f.write_str("YStericQuadrupole3D"),
            Self::ZStericQuadrupole3D => f.write_str("ZStericQuadrupole3D"),
            Self::FeatureCount3D => f.write_str("FeatureCount3D"),
            Self::FeatureAcceptorCount3D => f.write_str("FeatureAcceptorCount3D"),
            Self::FeatureDonorCount3D => f.write_str("FeatureDonorCount3D"),
            Self::FeatureAnionCount3D => f.write_str("FeatureAnionCount3D"),
            Self::FeatureCationCount3D => f.write_str("FeatureCationCount3D"),
            Self::FeatureRingCount3D => f.write_str("FeatureRingCount3D"),
            Self::FeatureHydrophobeCount3D => f.write_str("FeatureHydrophobeCount3D"),
            Self::EffectiveRotorCount3D => f.write_str("EffectiveRotorCount3D"),
            Self::ConformerCount3D => f.write_str("ConformerCount3D"),
            Self::Fingerprint2D => f.write_str("Fingerprint2D"),
            Self::Other(s) => f.write_str(s),
        }
    }
}

impl FromStr for CompoundPropertyTag {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_known(s).unwrap_or_else(|| Self::Other(s.to_string())))
    }
}

impl From<&str> for CompoundPropertyTag {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl From<String> for CompoundPropertyTag {
    fn from(s: String) -> Self {
        parse_known(&s).unwrap_or(Self::Other(s))
    }
}

impl serde::Serialize for CompoundPropertyTag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for CompoundPropertyTag {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

// ---------------------------------------------------------------------------
// CompoundProperty (list of tags)
// ---------------------------------------------------------------------------

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_url_string().fmt(f)
    }
}

impl FromStr for CompoundProperty {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tags: Vec<CompoundPropertyTag> = s.split(',').map(CompoundPropertyTag::from).collect();
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

impl From<CompoundPropertyTag> for CompoundProperty {
    fn from(value: CompoundPropertyTag) -> Self {
        Self(vec![value])
    }
}

impl From<&str> for CompoundProperty {
    fn from(value: &str) -> Self {
        Self(vec![CompoundPropertyTag::from(value)])
    }
}

impl From<String> for CompoundProperty {
    fn from(value: String) -> Self {
        Self(vec![CompoundPropertyTag::from(value)])
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_outputs_api_name() {
        assert_eq!(
            CompoundPropertyTag::MolecularFormula.to_string(),
            "MolecularFormula"
        );
        assert_eq!(
            CompoundPropertyTag::MolecularWeight.to_string(),
            "MolecularWeight"
        );
        assert_eq!(CompoundPropertyTag::Smiles.to_string(), "SMILES");
        assert_eq!(
            CompoundPropertyTag::ConnectivitySmiles.to_string(),
            "ConnectivitySMILES"
        );
        assert_eq!(CompoundPropertyTag::InChI.to_string(), "InChI");
        assert_eq!(CompoundPropertyTag::InChIKey.to_string(), "InChIKey");
        assert_eq!(CompoundPropertyTag::IupacName.to_string(), "IUPACName");
        assert_eq!(CompoundPropertyTag::XLogP.to_string(), "XLogP");
        assert_eq!(CompoundPropertyTag::Tpsa.to_string(), "TPSA");
        assert_eq!(
            CompoundPropertyTag::HBondDonorCount.to_string(),
            "HBondDonorCount"
        );
        assert_eq!(CompoundPropertyTag::Volume3D.to_string(), "Volume3D");
        assert_eq!(
            CompoundPropertyTag::ConformerModelRmsd3D.to_string(),
            "ConformerModelRMSD3D"
        );
        assert_eq!(
            CompoundPropertyTag::Fingerprint2D.to_string(),
            "Fingerprint2D"
        );
        assert_eq!(
            CompoundPropertyTag::Other("FutureProp".into()).to_string(),
            "FutureProp"
        );
    }

    #[test]
    fn test_from_str_snake_case() {
        assert_eq!(
            CompoundPropertyTag::from("molecular_weight"),
            CompoundPropertyTag::MolecularWeight,
        );
        assert_eq!(
            CompoundPropertyTag::from("smiles"),
            CompoundPropertyTag::Smiles
        );
        assert_eq!(
            CompoundPropertyTag::from("inchi"),
            CompoundPropertyTag::InChI
        );
        assert_eq!(
            CompoundPropertyTag::from("inchikey"),
            CompoundPropertyTag::InChIKey
        );
        assert_eq!(
            CompoundPropertyTag::from("iupac_name"),
            CompoundPropertyTag::IupacName
        );
        assert_eq!(
            CompoundPropertyTag::from("xlogp"),
            CompoundPropertyTag::XLogP
        );
        assert_eq!(CompoundPropertyTag::from("tpsa"), CompoundPropertyTag::Tpsa);
        assert_eq!(
            CompoundPropertyTag::from("h_bond_donor_count"),
            CompoundPropertyTag::HBondDonorCount,
        );
        assert_eq!(
            CompoundPropertyTag::from("volume_3d"),
            CompoundPropertyTag::Volume3D
        );
        assert_eq!(
            CompoundPropertyTag::from("fingerprint_2d"),
            CompoundPropertyTag::Fingerprint2D,
        );
        // "fingerprint" is not an official API name, becomes Other
        assert_eq!(
            CompoundPropertyTag::from("fingerprint"),
            CompoundPropertyTag::Other("fingerprint".into()),
        );
    }

    #[test]
    fn test_from_str_api_name() {
        assert_eq!(
            CompoundPropertyTag::from("MolecularWeight"),
            CompoundPropertyTag::MolecularWeight,
        );
        assert_eq!(
            CompoundPropertyTag::from("SMILES"),
            CompoundPropertyTag::Smiles
        );
        assert_eq!(
            CompoundPropertyTag::from("InChIKey"),
            CompoundPropertyTag::InChIKey
        );
        assert_eq!(
            CompoundPropertyTag::from("Fingerprint2D"),
            CompoundPropertyTag::Fingerprint2D,
        );
    }

    #[test]
    fn test_from_str_conformer_model_rmsd() {
        assert_eq!(
            CompoundPropertyTag::from("conformer_model_rmsd_3d"),
            CompoundPropertyTag::ConformerModelRmsd3D,
        );
        // Non-official alias becomes Other
        assert_eq!(
            CompoundPropertyTag::from("conformer_rmsd_3d"),
            CompoundPropertyTag::Other("conformer_rmsd_3d".into()),
        );
    }

    #[test]
    fn test_from_str_unknown_becomes_other() {
        assert_eq!(
            CompoundPropertyTag::from("FutureProperty"),
            CompoundPropertyTag::Other("FutureProperty".into()),
        );
        assert_eq!(
            CompoundPropertyTag::from("SomeNewField"),
            CompoundPropertyTag::Other("SomeNewField".into()),
        );
    }

    #[test]
    fn test_roundtrip_from_str_display() {
        for variant in CompoundPropertyTag::variants() {
            let api_name = variant.to_string();
            let parsed = CompoundPropertyTag::from(api_name.as_str());
            assert_eq!(parsed, variant, "roundtrip failed for {api_name}");
        }
    }

    #[test]
    fn test_is_empty() {
        assert!(CompoundPropertyTag::Other(String::new()).is_empty());
        assert!(!CompoundPropertyTag::MolecularFormula.is_empty());
        assert!(!CompoundPropertyTag::Other("x".into()).is_empty());
    }

    #[test]
    fn test_variants_count() {
        assert_eq!(CompoundPropertyTag::variants().count(), 42);
    }

    #[test]
    fn test_variants_no_duplicate_snake_case_names() {
        let mut names: Vec<_> = CompoundPropertyTag::variants()
            .map(|v| v.snake_case_name().into_owned())
            .collect();
        names.sort();
        let len_before = names.len();
        names.dedup();
        assert_eq!(len_before, names.len(), "duplicate snake_case names found");
    }

    #[test]
    fn test_compound_property_parse_preserves_tags() {
        let prop: CompoundProperty = "MolecularFormula,MolecularWeight".parse().unwrap();
        assert_eq!(prop.0.len(), 2);
        assert_eq!(prop.0[0], CompoundPropertyTag::MolecularFormula);
        assert_eq!(prop.0[1], CompoundPropertyTag::MolecularWeight);
    }

    #[test]
    fn test_serde_roundtrip() {
        let tag = CompoundPropertyTag::MolecularWeight;
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"MolecularWeight\"");
        let parsed: CompoundPropertyTag = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, tag);
    }

    #[test]
    fn test_serde_other_roundtrip() {
        let tag = CompoundPropertyTag::Other("CustomProp".into());
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"CustomProp\"");
        let parsed: CompoundPropertyTag = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, tag);
    }

    #[test]
    fn test_from_string_reuses_allocation_for_other() {
        let s = String::from("UnknownProp");
        let tag = CompoundPropertyTag::from(s);
        assert_eq!(tag, CompoundPropertyTag::Other("UnknownProp".into()));
    }

    #[test]
    fn test_from_string_known_variant() {
        let s = String::from("MolecularWeight");
        let tag = CompoundPropertyTag::from(s);
        assert_eq!(tag, CompoundPropertyTag::MolecularWeight);
    }

    #[test]
    fn test_compound_property_from_tag() {
        let prop = CompoundProperty::from(CompoundPropertyTag::MolecularWeight);
        assert_eq!(prop.0.len(), 1);
        assert_eq!(prop.0[0], CompoundPropertyTag::MolecularWeight);
    }

    #[test]
    fn test_compound_property_from_str_slice() {
        let prop = CompoundProperty::from("MolecularWeight");
        assert_eq!(prop.0.len(), 1);
        assert_eq!(prop.0[0], CompoundPropertyTag::MolecularWeight);
    }
}
