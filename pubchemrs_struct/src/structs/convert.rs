//! Conversions from raw PubChem API response types to higher-level structural types.
//!
//! Implements [`TryFrom`] for converting [`crate::response::Compound`] records
//! into [`Atom`] and [`Bond`] collections.

use std::collections::HashMap;

use itertools::{Itertools, izip};

use crate::error::*;
use crate::response::compound::Compound;
use crate::structs::{Atom, Bond, BondType, Coordinate, Element};

/// Parse coordinate data from a compound record into a map of atom ID to coordinate.
///
/// Returns `Ok(None)` if no coordinate data is present.
fn parse_coords(compound: &Compound) -> PubChemResult<Option<HashMap<u32, Coordinate>>> {
    let first_one = match compound.coords.first() {
        Some(c) => c,
        None => return Ok(None),
    };
    let coord_ids = &first_one.aid;
    let first_coord = first_one
        .conformers
        .first()
        .ok_or(PubChemError::ParseResponseError(
            "No conformer data found in coordinate record".into(),
        ))?;
    let xs = &first_coord.x;
    let ys = &first_coord.y;
    let zs = &first_coord.z;
    let coordinates: Vec<Coordinate> = xs
        .iter()
        .zip_longest(ys.iter())
        .map(|case| match case {
            itertools::EitherOrBoth::Both(x, y) => Ok((*x, *y)),
            _ => Err(PubChemError::ParseResponseError(
                "Error parsing atom coordinates".into(),
            )),
        })
        .process_results(|x_ys| match zs {
            Some(zs) => x_ys
                .zip_longest(zs.iter())
                .map(|inner| match inner {
                    itertools::EitherOrBoth::Both((x, y), z) => Ok(Coordinate::new(x, y, Some(*z))),
                    _ => Err(PubChemError::ParseResponseError(
                        "Error parsing atom coordinates".into(),
                    )),
                })
                .process_results(|iter| iter.collect()),
            None => Ok(x_ys.map(|(x, y)| Coordinate::new(x, y, None)).collect()),
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

impl TryFrom<&Compound> for Vec<Atom> {
    type Error = PubChemError;

    fn try_from(compound: &Compound) -> PubChemResult<Self> {
        let aids = &compound.atoms.aid;
        let element_ids = &compound.atoms.element;
        let coordinates = parse_coords(compound)?;
        // Build charges lookup
        let charges: HashMap<u32, i32> = compound
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
        // Zip atom IDs with element IDs, convert to Atom directly
        let atoms: Vec<Atom> = aids
            .iter()
            .zip(element_ids.iter())
            .map(|(aid, element_id)| {
                let element = Element::try_from(*element_id as u8)?;
                let coord = coordinates.as_ref().and_then(|c| c.get(aid).copied());
                let charge = charges.get(aid).copied().unwrap_or(0);
                Ok(Atom::from_record_data(*aid, element, coord, charge))
            })
            .collect::<PubChemResult<Vec<_>>>()?;

        Ok(atoms)
    }
}

impl TryFrom<&Compound> for Option<Vec<Bond>> {
    type Error = PubChemError;

    fn try_from(compound: &Compound) -> PubChemResult<Self> {
        let bonds_inner = match compound.bonds.as_ref() {
            Some(b) => b,
            None => return Ok(None),
        };
        let aid1s = &bonds_inner.aid1;
        let aid2s = &bonds_inner.aid2;
        let orders = &bonds_inner.order;
        let styles = &compound.coords.first().and_then(|inner| {
            inner
                .conformers
                .first()
                .and_then(|c_inner| c_inner.style.as_ref())
        });

        if aid1s.len() != aid2s.len() || aid2s.len() != orders.len() {
            return Err(PubChemError::ParseResponseError(
                format!(
                    "Bond array length mismatch: aid1={}, aid2={}, order={}",
                    aid1s.len(),
                    aid2s.len(),
                    orders.len()
                )
                .into(),
            ));
        }

        let bonds: Result<Vec<Bond>, PubChemError> =
            izip!(aid1s.iter(), aid2s.iter(), orders.iter())
                .map(|(aid1, aid2, order)| {
                    let order = BondType::try_from(*order as u8).map_err(|_| {
                        PubChemError::ParseResponseError(
                            format!("Invalid bond order: {}", order).into(),
                        )
                    })?;
                    Ok(Bond::new(*aid1, *aid2, Some(order), None))
                })
                .collect();
        let mut bonds = bonds?;
        // Apply style annotations from conformer data
        if !compound.coords.is_empty()
            && let Some(inner_style) = styles
        {
            let style_aid1s = &inner_style.aid1;
            let style_aid2s = &inner_style.aid2;
            let style_vals = &inner_style.annotation;
            bonds = bonds
                .into_iter()
                .map(|bond| {
                    for (aid1, aid2, style) in izip!(style_aid1s, style_aid2s, style_vals) {
                        if bond.is_same_bond_with_aid(*aid1, *aid2) {
                            return bond.with_style(Some(*style));
                        }
                    }
                    bond
                })
                .collect();
        }
        bonds.sort_by(|a, b| (a.aid1, a.aid2).cmp(&(b.aid1, b.aid2)));
        Ok(Some(bonds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::compound::Compound;

    fn minimal_compound_json() -> &'static str {
        r#"{
            "atoms": {"aid": [1, 2], "element": [6, 8]},
            "bonds": {"aid1": [1], "aid2": [2], "order": [2]},
            "charge": 0,
            "coords": [{"aid": [1, 2], "conformers": [{"x": [0.0, 1.0], "y": [0.0, 1.0]}], "type": []}],
            "count": {"atom_chiral": 0, "atom_chiral_def": 0, "atom_chiral_undef": 0, "bond_chiral": 0, "bond_chiral_def": 0, "bond_chiral_undef": 0, "covalent_unit": 1, "heavy_atom": 2, "isotope_atom": 0, "tautomers": -1},
            "id": {"id": {"cid": 1}},
            "props": []
        }"#
    }

    fn minimal_compound() -> Compound {
        serde_json::from_str(minimal_compound_json()).unwrap()
    }

    #[test]
    fn try_from_compound_atoms() {
        let compound = minimal_compound();
        let atoms: Vec<Atom> = Vec::<Atom>::try_from(&compound).unwrap();
        assert_eq!(atoms.len(), 2);
        assert_eq!(atoms[0].element, Element::C);
        assert_eq!(atoms[1].element, Element::O);
        assert_eq!(atoms[0].aid, 1);
        assert_eq!(atoms[1].aid, 2);
    }

    #[test]
    fn try_from_compound_bonds() {
        let compound = minimal_compound();
        let bonds: Option<Vec<Bond>> = Option::<Vec<Bond>>::try_from(&compound).unwrap();
        let bonds = bonds.unwrap();
        assert_eq!(bonds.len(), 1);
        assert_eq!(bonds[0].aid1, 1);
        assert_eq!(bonds[0].aid2, 2);
        assert_eq!(bonds[0].order, BondType::Double);
    }

    #[test]
    fn try_from_compound_no_bonds() {
        let mut compound = minimal_compound();
        compound.bonds = None;
        let bonds: Option<Vec<Bond>> = Option::<Vec<Bond>>::try_from(&compound).unwrap();
        assert!(bonds.is_none());
    }

    #[test]
    fn try_from_compound_atoms_no_coords() {
        let mut compound = minimal_compound();
        compound.coords.clear();
        let atoms: Vec<Atom> = Vec::<Atom>::try_from(&compound).unwrap();
        assert_eq!(atoms.len(), 2);
        assert!(atoms[0].coordinate.is_none());
    }

    #[test]
    fn try_from_compound_atoms_with_charges() {
        let json = r#"{
            "atoms": {"aid": [1, 2], "element": [6, 8], "charge": [{"aid": 2, "value": -1}]},
            "bonds": {"aid1": [1], "aid2": [2], "order": [2]},
            "charge": -1,
            "coords": [{"aid": [1, 2], "conformers": [{"x": [0.0, 1.0], "y": [0.0, 1.0]}], "type": []}],
            "count": {"atom_chiral": 0, "atom_chiral_def": 0, "atom_chiral_undef": 0, "bond_chiral": 0, "bond_chiral_def": 0, "bond_chiral_undef": 0, "covalent_unit": 1, "heavy_atom": 2, "isotope_atom": 0, "tautomers": -1},
            "id": {"id": {"cid": 1}},
            "props": []
        }"#;
        let compound: Compound = serde_json::from_str(json).unwrap();
        let atoms: Vec<Atom> = Vec::<Atom>::try_from(&compound).unwrap();
        assert_eq!(atoms[0].charge, 0);
        assert_eq!(atoms[1].charge, -1);
    }

    #[test]
    fn try_from_compound_bonds_with_styles() {
        let json = r#"{
            "atoms": {"aid": [1, 2], "element": [6, 8]},
            "bonds": {"aid1": [1], "aid2": [2], "order": [2]},
            "charge": 0,
            "coords": [{"aid": [1, 2], "conformers": [{"x": [0.0, 1.0], "y": [0.0, 1.0], "style": {"aid1": [1], "aid2": [2], "annotation": [5]}}], "type": []}],
            "count": {"atom_chiral": 0, "atom_chiral_def": 0, "atom_chiral_undef": 0, "bond_chiral": 0, "bond_chiral_def": 0, "bond_chiral_undef": 0, "covalent_unit": 1, "heavy_atom": 2, "isotope_atom": 0, "tautomers": -1},
            "id": {"id": {"cid": 1}},
            "props": []
        }"#;
        let compound: Compound = serde_json::from_str(json).unwrap();
        let bonds: Option<Vec<Bond>> = Option::<Vec<Bond>>::try_from(&compound).unwrap();
        let bonds = bonds.unwrap();
        assert_eq!(bonds[0].style, Some(5));
    }

    #[test]
    fn try_from_compound_atoms_3d() {
        let json = r#"{
            "atoms": {"aid": [1, 2], "element": [6, 8]},
            "bonds": {"aid1": [1], "aid2": [2], "order": [2]},
            "charge": 0,
            "coords": [{"aid": [1, 2], "conformers": [{"x": [0.0, 1.0], "y": [0.0, 1.0], "z": [0.5, 1.5]}], "type": []}],
            "count": {"atom_chiral": 0, "atom_chiral_def": 0, "atom_chiral_undef": 0, "bond_chiral": 0, "bond_chiral_def": 0, "bond_chiral_undef": 0, "covalent_unit": 1, "heavy_atom": 2, "isotope_atom": 0, "tautomers": -1},
            "id": {"id": {"cid": 1}},
            "props": []
        }"#;
        let compound: Compound = serde_json::from_str(json).unwrap();
        let atoms: Vec<Atom> = Vec::<Atom>::try_from(&compound).unwrap();
        assert!(atoms[0].coordinate.unwrap().z.is_some());
    }
}
