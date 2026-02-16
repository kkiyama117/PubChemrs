use crate::error::PubChemError;

/// A chemical bond between two atoms.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub struct Bond {
    /// Atom ID of the first bonded atom.
    pub aid1: u32,
    /// Atom ID of the second bonded atom.
    pub aid2: u32,
    /// Bond type (single, double, triple, etc.).
    pub order: BondType,
    /// Optional display style annotation (e.g. wedge/dash for stereo).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style: Option<u32>,
}

impl Bond {
    /// Creates a new bond between two atoms with the given order and optional style.
    pub fn new(aid1: u32, aid2: u32, order: Option<BondType>, style: Option<u32>) -> Self {
        Self {
            aid1,
            aid2,
            order: order.unwrap_or_default(),
            style,
        }
    }

    /// Returns a new bond with the given display style annotation.
    #[must_use]
    pub fn with_style(self, style: Option<u32>) -> Self {
        Self { style, ..self }
    }

    /// Returns `true` if this bond connects the same pair of atoms as `other`,
    /// regardless of atom ID ordering (bonds are undirected).
    pub fn is_same_bond(&self, other: &Self) -> bool {
        (self.aid1 == other.aid1 && self.aid2 == other.aid2)
            || (self.aid1 == other.aid2 && self.aid2 == other.aid1)
    }

    /// Returns `true` if this bond connects the given atom ID pair,
    /// regardless of atom ID ordering (bonds are undirected).
    pub fn is_same_bond_with_aid(&self, aid1: u32, aid2: u32) -> bool {
        (self.aid1 == aid1 && self.aid2 == aid2) || (self.aid1 == aid2 && self.aid2 == aid1)
    }
}

impl std::fmt::Display for Bond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bond({}, {}, {})", self.aid1, self.aid2, self.order)
    }
}

/// Chemical bond type / order.
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(eq, eq_int, from_py_object))]
#[repr(u8)]
pub enum BondType {
    /// Single bond.
    #[default]
    Single = 1,
    /// Double bond.
    Double = 2,
    /// Triple bond.
    Triple = 3,
    /// Quadruple bond.
    Quadruple = 4,
    /// Dative (coordinate) bond.
    Dative = 5,
    /// Complex bond.
    Complex = 6,
    /// Ionic bond.
    Ionic = 7,
    /// Unknown bond type.
    Unknown = 255,
}

impl_enum_str!(BondType {
    Single => "SINGLE",
    Double => "DOUBLE",
    Triple => "TRIPLE",
    Quadruple => "QUADRUPLE",
    Dative => "DATIVE",
    Complex => "COMPLEX",
    Ionic => "IONIC",
    Unknown => "UNKNOWN",
});

impl_from_repr!(BondType: u8 {
    Single = 1,
    Double = 2,
    Triple = 3,
    Quadruple = 4,
    Dative = 5,
    Complex = 6,
    Ionic = 7,
    Unknown = 255
});

impl TryFrom<u8> for BondType {
    type Error = PubChemError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(PubChemError::ParseEnum(
            crate::error::ParseEnumError::VariantNotFound,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_serialize() {
        let bond = Bond::new(3, 4, Some(BondType::Single), None);
        let bond2 = Bond::new(3, 4, None, None);
        let ser = serde_json::to_string(&bond2).unwrap();
        let de: Bond = serde_json::from_str(&ser).unwrap();
        assert_eq!(bond, de);
    }

    #[test]
    fn test_bond_type_display() {
        assert_eq!(BondType::Single.to_string(), "SINGLE");
        assert_eq!(BondType::Double.to_string(), "DOUBLE");
    }

    #[test]
    fn test_bond_type_from_str() {
        use std::str::FromStr;
        assert_eq!(BondType::from_str("SINGLE").unwrap(), BondType::Single);
        assert!(BondType::from_str("invalid").is_err());
    }

    #[test]
    fn test_bond_type_from_repr() {
        assert_eq!(BondType::from_repr(1), Some(BondType::Single));
        assert_eq!(BondType::from_repr(255), Some(BondType::Unknown));
        assert_eq!(BondType::from_repr(0), None);
    }
}
