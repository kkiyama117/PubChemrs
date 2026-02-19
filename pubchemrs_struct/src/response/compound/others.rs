/// A single property entry from a compound record, consisting of a URN key and a value.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct CompoundProps {
    /// Uniform Resource Name identifying the property.
    pub urn: PropsUrn,
    /// The property value.
    pub value: PropsValue,
}

/// Uniform Resource Name for a compound property, identifying its source and type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct PropsUrn {
    /// Numeric data type identifier.
    datatype: u32,
    /// Implementation identifier.
    implementation: Option<String>,
    /// Property label (e.g. `"Molecular Formula"`, `"SMILES"`).
    pub label: String,
    /// Property sub-name (e.g. `"Canonical"`, `"Isomeric"`).
    pub name: Option<String>,
    /// Parameter string, if any.
    parameters: Option<String>,
    /// Release identifier.
    release: Option<String>,
    /// Software that produced the value.
    software: Option<String>,
    /// Data source name.
    source: Option<String>,
    /// Version of the producing software/algorithm.
    version: Option<String>,
}

/// A property value from a compound record.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
#[serde(rename_all = "lowercase")]
pub enum PropsValue {
    /// Integer value.
    Ival(u32),
    /// Floating-point value.
    Fval(f64),
    /// Integer vector (e.g. used in 3D records).
    Ivec(Vec<i32>),
    /// Floating-point vector (e.g. multipoles in 3D records).
    Fvec(Vec<f64>),
    /// String value.
    Sval(String),
    /// String list value (e.g. pharmacophore features, shape fingerprints in 3D records).
    Slist(Vec<String>),
    /// Binary data encoded as a string.
    Binary(String),
}

impl PropsValue {
    /// Extract a String value from Sval variant.
    pub fn as_string(&self) -> Option<String> {
        match self {
            PropsValue::Sval(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Extract an f64 value. Fval returns directly, Ival is losslessly converted, Sval attempts to parse.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PropsValue::Fval(f) => Some(*f),
            PropsValue::Ival(i) => Some(f64::from(*i)),
            PropsValue::Sval(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Extract a u32 value. Ival returns directly, Sval attempts to parse.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            PropsValue::Ival(i) => Some(*i),
            PropsValue::Sval(s) => s.parse().ok(),
            _ => None,
        }
    }
}

/// Structural feature counts for a compound.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct CompoundTCount {
    /// Number of chiral atom centers.
    atom_chiral: u32,
    /// Number of defined chiral atom centers.
    atom_chiral_def: u32,
    /// Number of undefined chiral atom centers.
    atom_chiral_undef: u32,
    /// Number of chiral bonds.
    bond_chiral: u32,
    /// Number of defined chiral bonds.
    bond_chiral_def: u32,
    /// Number of undefined chiral bonds.
    bond_chiral_undef: u32,
    /// Number of covalent units (connected components).
    covalent_unit: u32,
    /// Number of heavy (non-hydrogen) atoms.
    heavy_atom: u32,
    /// Number of isotope-labeled atoms.
    isotope_atom: u32,
    /// Number of tautomers (-1 if unknown).
    tautomers: i32,
}

/// Stereochemistry annotation for a compound.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum Stereo {
    /// Tetrahedral stereocenter definition.
    #[serde(rename = "tetrahedral")]
    Tetrahedral {
        /// Atom ID of the ligand above the plane.
        above: u32,
        /// Atom ID of the ligand below the plane.
        below: u32,
        /// Atom ID of the bottom ligand.
        bottom: u32,
        /// Atom ID of the chiral center.
        center: u32,
        /// Parity value (1 = clockwise, 2 = counter-clockwise).
        parity: u32,
        /// Atom ID of the top ligand.
        top: u32,
        /// Stereo type identifier.
        #[serde(rename = "type")]
        _type: u32,
    },
}
