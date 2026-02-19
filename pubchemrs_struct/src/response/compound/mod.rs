//! Raw compound record types deserialized directly from the PubChem API.
//!
//! These mirror the JSON structure of PubChem `PC_Compounds` responses.
//! Use `TryFrom<&Compound>` for [`Vec<crate::structs::Atom>`] and
//! [`Option<Vec<crate::structs::Bond>>`] to convert raw arrays into
//! higher-level [`crate::structs`] types.

/// Raw atom data arrays.
pub mod atom;
/// Raw bond data arrays.
pub mod bond;
/// Conformer coordinate data.
pub mod conformer;
/// Coordinate set wrapper.
pub mod coordinate;
/// Properties, counts, and stereochemistry.
pub mod others;

#[cfg(feature = "pyo3")]
mod py_methods;

use self::atom::AtomInner;
use self::bond::BondInner;
use self::coordinate::CoordsInner;
use self::others::*;

/// A collection of compound records.
pub type Compounds = Vec<Compound>;

/// Represents a chemical compound with its properties.
/// This is a pure Rust struct that mirrors PubChemPy's Compound class.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "pyo3",
    pyo3::pyclass(name = "CompoundRecord", from_py_object)
)]
pub struct Compound {
    /// Structural atoms.
    /// The response of atoms is not useful by used as it is, so we need to convert them when using.
    pub atoms: AtomInner,
    /// Bonds between atoms.
    /// The response of bonds is not useful by used as it is, so we need to convert them when using.
    pub bonds: Option<BondInner>,
    /// The total (or net) charge of a molecule (absent in some 3D records).
    #[serde(default)]
    pub charge: Option<i32>,
    /// Coordinate sets for atom positions.
    pub coords: Vec<CoordsInner>,
    /// Counts of various structural features (chiral atoms, heavy atoms, etc.).
    /// Absent in some 3D records.
    #[serde(default)]
    pub count: Option<CompoundTCount>,
    /// Compound identifier (CID).
    #[serde(rename = "id")]
    pub cid: Option<CompoundID>,
    /// Compound property key-value pairs.
    pub props: Vec<CompoundProps>,
    /// Stereochemistry annotations, if present.
    pub stereo: Option<Vec<Stereo>>,
}

impl Compound {
    /// Search props array by label and return the first matching value.
    pub fn parse_prop_by_label(&self, label: &str) -> Option<&PropsValue> {
        self.props
            .iter()
            .find(|p| p.urn.label == label)
            .map(|p| &p.value)
    }

    /// Search props array by label and name, return the first matching value.
    pub fn parse_prop_by_label_and_name(&self, label: &str, name: &str) -> Option<&PropsValue> {
        self.props
            .iter()
            .find(|p| p.urn.label == label && p.urn.name.as_deref() == Some(name))
            .map(|p| &p.value)
    }

    /// Search props array by implementation identifier, return the first matching value.
    pub fn parse_prop_by_implementation(&self, implementation: &str) -> Option<&PropsValue> {
        self.props
            .iter()
            .find(|p| p.urn.implementation.as_deref() == Some(implementation))
            .map(|p| &p.value)
    }
}

/// Compound identifier wrapper as returned in the raw API response.
#[derive(Debug, Clone, PartialEq, Eq, Copy, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
#[repr(u32)]
pub enum CompoundID {
    /// PubChem Compound ID (CID).
    #[serde(rename = "id")]
    Cid {
        /// The numeric CID value.
        cid: u32,
    },
}

impl PartialEq<crate::structs::CompoundID> for CompoundID {
    fn eq(&self, other: &crate::structs::CompoundID) -> bool {
        match self {
            CompoundID::Cid { cid } => *cid == *other,
        }
    }
}

impl From<crate::structs::CompoundID> for CompoundID {
    fn from(value: crate::structs::CompoundID) -> Self {
        Self::Cid { cid: value }
    }
}

impl From<CompoundID> for crate::structs::CompoundID {
    fn from(value: CompoundID) -> Self {
        match value {
            CompoundID::Cid { cid } => cid,
        }
    }
}

impl std::hash::Hash for CompoundID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            CompoundID::Cid { cid } => cid.hash(state),
        }
    }
}

impl std::fmt::Display for CompoundID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundID::Cid { cid } => cid.fmt(f),
        }
    }
}
