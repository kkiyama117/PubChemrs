#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct CompoundProps {
    pub urn: PropsUrn,
    pub value: PropsValue,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct PropsUrn {
    datatype: u32,
    implementation: Option<String>,
    pub label: String,
    pub name: Option<String>,
    parameters: Option<String>,
    release: Option<String>,
    software: Option<String>,
    source: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
#[serde(rename_all = "lowercase")]
pub enum PropsValue {
    Ival(u32),
    Fval(f32),
    Sval(String),
    // TODO: Use `binary`
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

    /// Extract an f32 value. Fval returns directly, Sval attempts to parse.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            PropsValue::Fval(f) => Some(*f),
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct CompoundTCount {
    atom_chiral: u32,
    atom_chiral_def: u32,
    atom_chiral_undef: u32,
    bond_chiral: u32,
    bond_chiral_def: u32,
    bond_chiral_undef: u32,
    covalent_unit: u32,
    heavy_atom: u32,
    isotope_atom: u32,
    tautomers: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Stereo {
    #[serde(rename = "tetrahedral")]
    Tetrahedral {
        above: u32,
        below: u32,
        bottom: u32,
        center: u32,
        parity: u32,
        top: u32,
        #[serde(rename = "type")]
        _type: u32,
    },
}
