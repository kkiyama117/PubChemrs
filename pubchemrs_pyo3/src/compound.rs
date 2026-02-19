use std::sync::OnceLock;

use num_bigint::BigUint;

use pubchemrs_struct::response::Compound;
use pubchemrs_struct::response::compound::others::PropsValue;
use pubchemrs_struct::structs::{Atom, Bond};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyDictMethods, PyList, PyString};

/// Python-facing Compound wrapper around a raw `Compound` record.
///
/// Provides convenient property getters mirroring the legacy Python Compound class,
/// with lazy caching for expensive conversions (atoms, bonds).
#[pyclass(name = "Compound")]
pub struct PyCompound {
    record: Compound,
    atoms_cache: OnceLock<Vec<Atom>>,
    bonds_cache: OnceLock<Vec<Bond>>,
}

impl PyCompound {
    pub fn from_record(record: Compound) -> Self {
        Self {
            record,
            atoms_cache: OnceLock::new(),
            bonds_cache: OnceLock::new(),
        }
    }

    fn cached_atoms(&self) -> &[Atom] {
        self.atoms_cache
            .get_or_init(|| Vec::<Atom>::try_from(&self.record).unwrap_or_default())
    }

    fn cached_bonds(&self) -> &[Bond] {
        self.bonds_cache.get_or_init(|| {
            Option::<Vec<Bond>>::try_from(&self.record)
                .ok()
                .flatten()
                .unwrap_or_default()
        })
    }
}

// ---------------------------------------------------------------------------
// Helper: convert PropsValue to a Python object
// ---------------------------------------------------------------------------

fn props_value_to_py<'py>(py: Python<'py>, value: &PropsValue) -> PyResult<Bound<'py, PyAny>> {
    match value {
        PropsValue::Ival(v) => Ok(v.into_pyobject(py)?.into_any()),
        PropsValue::Fval(v) => Ok(v.into_pyobject(py)?.into_any()),
        PropsValue::Sval(v) => Ok(PyString::new(py, v).into_any()),
        PropsValue::Binary(v) => Ok(PyString::new(py, v).into_any()),
        PropsValue::Ivec(v) => {
            let list = PyList::new(py, v)?;
            Ok(list.into_any())
        }
        PropsValue::Fvec(v) => {
            let list = PyList::new(py, v)?;
            Ok(list.into_any())
        }
        PropsValue::Slist(v) => {
            let items: Vec<Bound<'py, PyAny>> = v
                .iter()
                .map(|s| Ok(PyString::new(py, s).into_any()))
                .collect::<PyResult<_>>()?;
            let list = PyList::new(py, items)?;
            Ok(list.into_any())
        }
    }
}

/// Search conformer.data for a property by label and name.
fn parse_conformer_prop<'a>(
    record: &'a Compound,
    label: &str,
    name: &str,
) -> Option<&'a PropsValue> {
    record
        .coords
        .first()?
        .conformers
        .first()?
        .data
        .as_ref()?
        .iter()
        .find(|p| p.urn.label == label && p.urn.name.as_deref() == Some(name))
        .map(|p| &p.value)
}

/// Search coords.data for a property by label and name.
fn parse_coords_data_prop<'a>(
    record: &'a Compound,
    label: &str,
    name: &str,
) -> Option<&'a PropsValue> {
    record
        .coords
        .first()?
        .data
        .as_ref()?
        .iter()
        .find(|p| p.urn.label == label && p.urn.name.as_deref() == Some(name))
        .map(|p| &p.value)
}

// ---------------------------------------------------------------------------
// PyMethods
// ---------------------------------------------------------------------------

#[pymethods]
impl PyCompound {
    #[new]
    fn new(record: Compound) -> Self {
        Self::from_record(record)
    }

    #[getter]
    fn record(&self) -> Compound {
        self.record.clone()
    }

    // -- Direct fields ------------------------------------------------------

    #[getter]
    fn cid(&self) -> Option<u32> {
        self.record.cid.map(|id| match id {
            pubchemrs_struct::response::compound::CompoundID::Cid { cid } => cid,
        })
    }

    #[getter]
    fn charge(&self) -> i32 {
        self.record.charge.unwrap_or(0)
    }

    // -- Structural data ----------------------------------------------------

    #[getter]
    fn atoms(&self) -> Vec<Atom> {
        self.cached_atoms().to_vec()
    }

    #[getter]
    fn bonds(&self) -> Vec<Bond> {
        self.cached_bonds().to_vec()
    }

    #[getter]
    fn elements(&self) -> Vec<String> {
        self.cached_atoms()
            .iter()
            .map(|a| a.element.to_string())
            .collect()
    }

    #[getter]
    fn coordinate_type(&self) -> Option<String> {
        let types = &self.record.coords.first()?.coord_type;
        if types.contains(&1) {
            Some("2d".to_string())
        } else if types.contains(&2) {
            Some("3d".to_string())
        } else {
            None
        }
    }

    // -- Props by label -----------------------------------------------------

    #[getter]
    fn molecular_formula(&self) -> Option<String> {
        self.record
            .parse_prop_by_label("Molecular Formula")?
            .as_string()
    }

    #[getter]
    fn molecular_weight(&self) -> Option<f64> {
        self.record
            .parse_prop_by_label("Molecular Weight")?
            .as_f64()
    }

    #[getter]
    fn xlogp<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_label("Log P") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    // -- Props by label + name ----------------------------------------------

    #[getter]
    fn smiles(&self) -> Option<String> {
        self.record
            .parse_prop_by_label_and_name("SMILES", "Absolute")?
            .as_string()
    }

    #[getter]
    fn connectivity_smiles(&self) -> Option<String> {
        self.record
            .parse_prop_by_label_and_name("SMILES", "Connectivity")?
            .as_string()
    }

    #[getter]
    fn inchi(&self) -> Option<String> {
        self.record
            .parse_prop_by_label_and_name("InChI", "Standard")?
            .as_string()
    }

    #[getter]
    fn inchikey(&self) -> Option<String> {
        self.record
            .parse_prop_by_label_and_name("InChIKey", "Standard")?
            .as_string()
    }

    #[getter]
    fn iupac_name(&self) -> Option<String> {
        self.record
            .parse_prop_by_label_and_name("IUPAC Name", "Preferred")?
            .as_string()
    }

    #[getter]
    fn exact_mass(&self) -> Option<f64> {
        self.record
            .parse_prop_by_label_and_name("Mass", "Exact")?
            .as_f64()
    }

    #[getter]
    fn monoisotopic_mass(&self) -> Option<f64> {
        self.record
            .parse_prop_by_label_and_name("Weight", "MonoIsotopic")?
            .as_f64()
    }

    // -- Props by implementation --------------------------------------------

    #[getter]
    fn tpsa<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_TPSA") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn complexity<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_COMPLEXITY") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn h_bond_donor_count<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_NHDONORS") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn h_bond_acceptor_count<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_NHACCEPTORS") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn rotatable_bond_count<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_NROTBONDS") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn fingerprint<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self.record.parse_prop_by_implementation("E_SCREEN") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    // -- Count fields -------------------------------------------------------

    #[getter]
    fn heavy_atom_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.heavy_atom)
    }

    #[getter]
    fn isotope_atom_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.isotope_atom)
    }

    #[getter]
    fn atom_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.atom_chiral)
    }

    #[getter]
    fn defined_atom_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.atom_chiral_def)
    }

    #[getter]
    fn undefined_atom_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.atom_chiral_undef)
    }

    #[getter]
    fn bond_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.bond_chiral)
    }

    #[getter]
    fn defined_bond_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.bond_chiral_def)
    }

    #[getter]
    fn undefined_bond_stereo_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.bond_chiral_undef)
    }

    #[getter]
    fn covalent_unit_count(&self) -> Option<u32> {
        self.record.count.as_ref().map(|c| c.covalent_unit)
    }

    // -- 3D properties (conformer.data) -------------------------------------

    #[getter]
    fn volume_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Shape", "Volume") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn multipoles_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Shape", "Multipoles") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn mmff94_energy_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Energy", "MMFF94 NoEstat") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn conformer_id_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Conformer", "ID") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn shape_selfoverlap_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Shape", "Self Overlap") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn feature_selfoverlap_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Feature", "Self Overlap") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn shape_fingerprint_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_conformer_prop(&self.record, "Fingerprint", "Shape") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    // -- 3D properties (coords.data) ----------------------------------------

    #[getter]
    fn conformer_rmsd_3d<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        match parse_coords_data_prop(&self.record, "Conformer", "RMSD") {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    // -- 3D properties (props) ----------------------------------------------

    #[getter]
    fn effective_rotor_count_3d<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self
            .record
            .parse_prop_by_label_and_name("Count", "Effective Rotor")
        {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn pharmacophore_features_3d<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self
            .record
            .parse_prop_by_label_and_name("Features", "Pharmacophore")
        {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    #[getter]
    fn mmff94_partial_charges_3d<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        match self
            .record
            .parse_prop_by_label_and_name("Charge", "MMFF94 Partial")
        {
            Some(v) => Ok(Some(props_value_to_py(py, v)?)),
            None => Ok(None),
        }
    }

    // -- Derived properties -------------------------------------------------

    #[getter]
    fn cactvs_fingerprint(&self) -> Option<String> {
        let value = self.record.parse_prop_by_implementation("E_SCREEN")?;
        let fp = match value {
            PropsValue::Sval(s) | PropsValue::Binary(s) => s.clone(),
            _ => return None,
        };
        if fp.len() < 9 {
            return None;
        }
        // Skip first 4 bytes (8 hex chars = fingerprint length prefix)
        let hex_part = &fp[8..];
        let val = BigUint::parse_bytes(hex_part.as_bytes(), 16)?;
        let binary = format!("{val:b}");
        // Pad to full hex width, remove last 7 padding bits, then zero-fill to 881 bits
        let full_width = hex_part.len() * 4;
        let padded = format!("{:0>width$}", binary, width = full_width);
        if padded.len() >= 7 {
            let trimmed = &padded[..padded.len() - 7];
            Some(format!("{:0>881}", trimmed))
        } else {
            None
        }
    }

    // -- Deprecated properties ----------------------------------------------

    #[getter]
    fn canonical_smiles(&self, py: Python<'_>) -> PyResult<Option<String>> {
        let warnings = py.import("warnings")?;
        warnings.call_method1(
            "warn",
            (
                "canonical_smiles is deprecated, use connectivity_smiles instead",
                py.import("builtins")?.getattr("DeprecationWarning")?,
            ),
        )?;
        Ok(self.connectivity_smiles())
    }

    #[getter]
    fn isomeric_smiles(&self, py: Python<'_>) -> PyResult<Option<String>> {
        let warnings = py.import("warnings")?;
        warnings.call_method1(
            "warn",
            (
                "isomeric_smiles is deprecated, use smiles instead",
                py.import("builtins")?.getattr("DeprecationWarning")?,
            ),
        )?;
        Ok(self.smiles())
    }

    // -- Methods ------------------------------------------------------------

    #[pyo3(signature = (properties=None))]
    fn to_dict<'py>(
        &self,
        py: Python<'py>,
        properties: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        let default_props: Vec<&str> = vec![
            "cid",
            "atoms",
            "bonds",
            "elements",
            "charge",
            "coordinate_type",
            "molecular_formula",
            "molecular_weight",
            "smiles",
            "connectivity_smiles",
            "inchi",
            "inchikey",
            "iupac_name",
            "exact_mass",
            "monoisotopic_mass",
            "xlogp",
            "tpsa",
            "complexity",
            "h_bond_donor_count",
            "h_bond_acceptor_count",
            "rotatable_bond_count",
            "heavy_atom_count",
            "isotope_atom_count",
            "atom_stereo_count",
            "defined_atom_stereo_count",
            "undefined_atom_stereo_count",
            "bond_stereo_count",
            "defined_bond_stereo_count",
            "undefined_bond_stereo_count",
            "covalent_unit_count",
            "fingerprint",
            "cactvs_fingerprint",
        ];

        let prop_names: Vec<&str> = match &properties {
            Some(p) => p.iter().map(String::as_str).collect(),
            None => default_props,
        };

        for prop in &prop_names {
            self.set_prop_in_dict(py, &dict, prop)?;
        }

        Ok(dict)
    }

    fn set_prop_in_dict<'py>(
        &self,
        py: Python<'py>,
        dict: &Bound<'py, PyDict>,
        prop: &str,
    ) -> PyResult<()> {
        match prop {
            "cid" => dict.set_item(prop, self.cid())?,
            "charge" => dict.set_item(prop, self.charge())?,
            "atoms" => {
                let atoms: Vec<_> = self
                    .cached_atoms()
                    .iter()
                    .map(|a| {
                        let obj = a.clone().into_pyobject(py)?;
                        obj.call_method0("to_dict")
                    })
                    .collect::<PyResult<_>>()?;
                dict.set_item(prop, PyList::new(py, atoms)?)?;
            }
            "bonds" => {
                let bonds: Vec<_> = self
                    .cached_bonds()
                    .iter()
                    .map(|b| {
                        let obj = b.clone().into_pyobject(py)?;
                        obj.call_method0("to_dict")
                    })
                    .collect::<PyResult<_>>()?;
                dict.set_item(prop, PyList::new(py, bonds)?)?;
            }
            "elements" => dict.set_item(prop, self.elements())?,
            "coordinate_type" => dict.set_item(prop, self.coordinate_type())?,
            "molecular_formula" => dict.set_item(prop, self.molecular_formula())?,
            "molecular_weight" => dict.set_item(prop, self.molecular_weight())?,
            "smiles" => dict.set_item(prop, self.smiles())?,
            "connectivity_smiles" => dict.set_item(prop, self.connectivity_smiles())?,
            "inchi" => dict.set_item(prop, self.inchi())?,
            "inchikey" => dict.set_item(prop, self.inchikey())?,
            "iupac_name" => dict.set_item(prop, self.iupac_name())?,
            "exact_mass" => dict.set_item(prop, self.exact_mass())?,
            "monoisotopic_mass" => dict.set_item(prop, self.monoisotopic_mass())?,
            "xlogp" => dict.set_item(prop, self.xlogp(py)?)?,
            "tpsa" => dict.set_item(prop, self.tpsa(py)?)?,
            "complexity" => dict.set_item(prop, self.complexity(py)?)?,
            "h_bond_donor_count" => dict.set_item(prop, self.h_bond_donor_count(py)?)?,
            "h_bond_acceptor_count" => dict.set_item(prop, self.h_bond_acceptor_count(py)?)?,
            "rotatable_bond_count" => dict.set_item(prop, self.rotatable_bond_count(py)?)?,
            "heavy_atom_count" => dict.set_item(prop, self.heavy_atom_count())?,
            "isotope_atom_count" => dict.set_item(prop, self.isotope_atom_count())?,
            "atom_stereo_count" => dict.set_item(prop, self.atom_stereo_count())?,
            "defined_atom_stereo_count" => {
                dict.set_item(prop, self.defined_atom_stereo_count())?;
            }
            "undefined_atom_stereo_count" => {
                dict.set_item(prop, self.undefined_atom_stereo_count())?;
            }
            "bond_stereo_count" => dict.set_item(prop, self.bond_stereo_count())?,
            "defined_bond_stereo_count" => {
                dict.set_item(prop, self.defined_bond_stereo_count())?;
            }
            "undefined_bond_stereo_count" => {
                dict.set_item(prop, self.undefined_bond_stereo_count())?;
            }
            "covalent_unit_count" => dict.set_item(prop, self.covalent_unit_count())?,
            "fingerprint" => dict.set_item(prop, self.fingerprint(py)?)?,
            "cactvs_fingerprint" => dict.set_item(prop, self.cactvs_fingerprint())?,
            _ => {} // Unknown properties are silently ignored
        }
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("Compound({})", self.cid().unwrap_or(0))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.record == other.record
    }
}
