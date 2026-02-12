//! Raw compound record types deserialized directly from the PubChem API.
//!
//! These mirror the JSON structure of PubChem `PC_Compounds` responses.
//! Use [`Compound::setup_atoms`] and [`Compound::setup_bonds`] to convert
//! the raw arrays into higher-level [`crate::structs`] types.

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

use std::collections::HashMap;

use itertools::{Itertools, izip};

use self::atom::AtomInner;
use self::bond::BondInner;
use self::coordinate::CoordsInner;
use self::others::*;
use crate::error::*;
use crate::structs::Element;

/// A collection of compound records.
pub type Compounds = Vec<Compound>;

/// Represents a chemical compound with its properties.
/// This is a pure Rust struct that mirrors PubChemPy's Compound class.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct Compound {
    /// Structural atoms.
    /// The response of atoms is not useful by used as it is, so we need to convert them when using.
    pub atoms: AtomInner,
    /// Bonds between atoms.
    /// The response of bonds is not useful by used as it is, so we need to convert them when using.
    pub bonds: Option<BondInner>,
    /// The total (or net) charge of a molecule.
    pub charge: i32,
    /// Coordinate sets for atom positions.
    pub coords: Vec<CoordsInner>,
    /// Counts of various structural features (chiral atoms, heavy atoms, etc.).
    pub count: CompoundTCount,
    /// Compound identifier (CID).
    #[serde(rename = "id")]
    pub cid: Option<CompoundID>,
    /// Compound property key-value pairs.
    pub props: Vec<CompoundProps>,
    /// Stereochemistry annotations, if present.
    pub stereo: Option<Vec<Stereo>>,
}

impl Compound {
    /// TODO: implement this.
    pub fn as_dataframe() {
        todo!()
    }

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

    /// If no coordinates in record, Return Ok(None).
    /// If there are data of coordinates but length of them are not the same as other ones, return Error.
    pub fn parse_coords(&self) -> PubChemResult<Option<HashMap<u32, crate::structs::Coordinate>>> {
        // get first vec in records.
        let first_one = self.coords.first();
        if first_one.is_none() {
            return Ok(None);
        }
        let first_one = self.coords.first().unwrap();
        // get the atom ids of each coordinates.
        let coord_ids = &first_one.aid;
        // get first coordinate vector in the first vec of records.
        let first_coord = &first_one.conformers.first().ok_or(PubChemError::Unknown)?;
        // create the pair of (x,y,z)
        let xs = &first_coord.x;
        let ys = &first_coord.y;
        let zs = &first_coord.z;
        // Generate iterable of (x,y) and after that, merge z
        let coordinates: Vec<crate::structs::Coordinate> = xs
            .iter()
            .zip_longest(ys.iter())
            .map(|case| match case {
                itertools::EitherOrBoth::Both(x, y) => Ok((*x, *y)),
                // Not the same length
                _ => Err(PubChemError::ParseResponseError(
                    "Error parsing atom coordinates".into(),
                )),
            })
            .process_results(|x_ys| {
                // create coordinates
                match zs {
                    Some(zs) => x_ys
                        .zip_longest(zs.iter())
                        .map(|inner| match inner {
                            itertools::EitherOrBoth::Both((x, y), z) => {
                                Ok(crate::structs::Coordinate::new(x, y, Some(*z)))
                            }
                            // Not the same length
                            _ => Err(PubChemError::ParseResponseError(
                                "Error parsing atom coordinates".into(),
                            )),
                        })
                        .process_results(|iter| iter.collect()),
                    None => Ok(x_ys
                        .map(|(x, y)| crate::structs::Coordinate::new(x, y, None))
                        .collect()),
                }
            })??;
        let result = coord_ids
            .iter()
            .zip_longest(coordinates.into_iter())
            .map(|inner| match inner {
                itertools::EitherOrBoth::Both(aid, coord) => Ok((*aid, coord)),
                _ => Err(PubChemError::ParseResponseError(
                    "Error parsing atom coordinates".into(),
                )),
            })
            .process_results(|result| result.collect())?;
        Ok(Some(result))
    }

    /// Derive Atom objects from the record.
    /// Creates atoms from atom IDs, elements, coordinates, and charges.
    /// TODO: Implement the faster way than current one.
    pub fn setup_atoms(&self) -> PubChemResult<Vec<crate::structs::Atom>> {
        let aids = &self.atoms.aid;
        let element_ids = &self.atoms.element;
        let coordinates = self.parse_coords()?;
        // Build charges lookup
        let charges: HashMap<u32, i32> = self
            .atoms
            .charge
            .as_ref()
            .map(|charge_inner| {
                charge_inner
                    .iter()
                    .map(|inner| (inner.aid, inner.value))
                    .collect()
            })
            .unwrap_or_default();
        // At first, zip all things
        let a: HashMap<u32, (u32, Option<crate::structs::Coordinate>)> = match coordinates {
            Some(coordinate_data) => {
                // With coordinates: merge aid, element_id, and coordinates
                aids.iter()
                    .zip_longest(element_ids.iter())
                    .map(|pair| match pair {
                        itertools::EitherOrBoth::Both(aid, element_id) => {
                            Ok((*aid, (*element_id, coordinate_data.get(aid).copied())))
                        }
                        _ => Err(PubChemError::ParseResponseError(
                            "Atom aids and elements length mismatch".into(),
                        )),
                    })
                    .process_results(|pair_iter| pair_iter.collect())?
            }
            // Without coordinate, only use atom id and element
            None => aids
                .iter()
                .zip_longest(element_ids.iter())
                .map(|pair| match pair {
                    // Check the length
                    itertools::EitherOrBoth::Both(aid, element_id) => {
                        Ok((*aid, (*element_id, None)))
                    }
                    _ => Err(PubChemError::ParseResponseError("".into())),
                })
                .process_results(|pair_iter| pair_iter.collect())?,
        };
        // construct Atom
        let atoms: HashMap<u32, _> = a
            .iter()
            // element_id to element
            .map(|(key, (element_id, coordinate_op))| {
                Element::try_from(*element_id as u8).map(|element| (*key, (element, coordinate_op)))
            })
            .process_results(|inner_iter| inner_iter.collect())?;
        let atoms = atoms
            .into_iter()
            .map(|(aid, (element, coord))| {
                let charge = charges.get(&aid).copied().unwrap_or(0);
                crate::structs::Atom::_from_record_data(aid, element, *coord, charge)
            })
            .sorted_by(|a, b| a.aid.cmp(&b.aid))
            .collect();

        Ok(atoms)
    }

    /// Derive Bond objects from the record.
    pub fn setup_bonds(&self) -> PubChemResult<Option<Vec<crate::structs::Bond>>> {
        match self.bonds.as_ref() {
            Some(bonds) => {
                // Create bonds
                let aid1s = &bonds.aid1;
                let aid2s = &bonds.aid2;
                let orders = &bonds.order;
                let styles = &self.coords.first().and_then(|inner| {
                    inner
                        .conformers
                        .first()
                        .and_then(|c_inner| c_inner.style.as_ref())
                });

                if aid1s.len() == aid2s.len() && aid2s.len() == orders.len() {
                    let bonds: Result<Vec<crate::structs::Bond>, PubChemError> =
                        izip!(aid1s.iter(), aid2s.iter(), orders.iter())
                            .map(|(aid1, aid2, order)| {
                                let order = crate::structs::BondType::try_from(*order as u8)
                                    .map_err(|_| {
                                        PubChemError::ParseResponseError(
                                            format!("Invalid bond order: {}", order).into(),
                                        )
                                    })?;
                                Ok(crate::structs::Bond::new(*aid1, *aid2, Some(order), None))
                            })
                            .collect();
                    let mut bonds = bonds?;
                    // Add styles if coords exist and styles in coords.
                    if !self.coords.is_empty()
                        && let Some(inner_style) = styles
                    {
                        let style_aid1s = &inner_style.aid1;
                        let style_aid2s = &inner_style.aid2;
                        let style_vals = &inner_style.annotation;
                        for bond in &mut bonds {
                            for (aid1, aid2, style) in izip!(style_aid1s, style_aid2s, style_vals) {
                                if bond.is_same_bond_with_aid(*aid1, *aid2) {
                                    bond.set_style(Some(*style));
                                }
                            }
                        }
                    }
                    bonds.sort_by(|a, b| (a.aid1, a.aid2).cmp(&(b.aid1, b.aid2)));
                    Ok(Some(bonds))
                } else {
                    Err(PubChemError::Unknown)
                }
            }
            None => Ok(None),
        }
    }
}

/// Compound identifier wrapper as returned in the raw API response.
#[derive(Debug, Clone, PartialEq, Eq, Copy, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
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
