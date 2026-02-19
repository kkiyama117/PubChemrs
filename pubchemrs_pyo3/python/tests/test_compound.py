"""Test Compound class (adapted from pubchempy test_compound.py)."""

import warnings

import pytest

from pubchemrs import Compound, get_compounds


@pytest.mark.network
class TestCompound:
    """Test the Compound convenience class."""

    @pytest.fixture(scope="class")
    def benzene(self):
        """Fetch benzene (CID 241) as a Compound."""
        compounds = get_compounds(241)
        return compounds[0]

    @pytest.fixture(scope="class")
    def aspirin(self):
        """Fetch aspirin (CID 2244) as a Compound."""
        compounds = get_compounds(2244)
        return compounds[0]

    def test_cid(self, benzene):
        """Test that CID is correct."""
        assert benzene.cid == 241

    def test_molecular_formula(self, aspirin):
        """Test molecular formula for aspirin."""
        assert aspirin.molecular_formula == "C9H8O4"

    def test_molecular_weight(self, aspirin):
        """Test molecular weight for aspirin."""
        assert aspirin.molecular_weight is not None
        assert abs(aspirin.molecular_weight - 180.16) < 0.1

    def test_identifiers(self, aspirin):
        """Test chemical identifiers for aspirin."""
        assert aspirin.inchikey is not None
        assert aspirin.inchikey.startswith("BSYNRYMUTXBXSQ")
        assert aspirin.inchi is not None
        assert aspirin.inchi.startswith("InChI=")
        assert aspirin.smiles is not None

    def test_deprecated_smiles(self, aspirin):
        """Test that deprecated SMILES properties emit warnings."""
        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            val = aspirin.isomeric_smiles
            assert val is not None
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecationWarning)

        with warnings.catch_warnings(record=True) as w:
            warnings.simplefilter("always")
            val = aspirin.canonical_smiles
            assert val is not None
            assert len(w) == 1
            assert issubclass(w[0].category, DeprecationWarning)

    def test_properties_types(self, aspirin):
        """Test that property types are correct."""
        assert isinstance(aspirin.molecular_weight, float)
        assert isinstance(aspirin.iupac_name, str)
        assert isinstance(aspirin.xlogp, (int, float))
        assert isinstance(aspirin.tpsa, (int, float))
        assert isinstance(aspirin.complexity, (int, float))
        assert isinstance(aspirin.h_bond_donor_count, int)
        assert isinstance(aspirin.h_bond_acceptor_count, int)
        assert isinstance(aspirin.rotatable_bond_count, int)
        assert isinstance(aspirin.heavy_atom_count, int)

    def test_atoms_and_bonds(self, aspirin):
        """Test atoms and bonds are populated."""
        assert len(aspirin.atoms) > 0
        assert len(aspirin.bonds) > 0
        assert len(aspirin.elements) == len(aspirin.atoms)

    def test_count_properties(self, aspirin):
        """Test count-based properties."""
        assert aspirin.heavy_atom_count is not None
        assert aspirin.atom_stereo_count is not None
        assert aspirin.bond_stereo_count is not None
        assert aspirin.covalent_unit_count is not None

    def test_coordinate_type(self, aspirin):
        """Test coordinate_type returns 2d or 3d."""
        ct = aspirin.coordinate_type
        assert ct in ("2d", "3d")

    def test_to_dict(self, aspirin):
        """Test to_dict() method."""
        d = aspirin.to_dict()
        assert isinstance(d, dict)
        assert d["cid"] == 2244
        assert d["molecular_formula"] == "C9H8O4"
        assert "molecular_weight" in d
        assert "inchikey" in d
        assert "smiles" in d

    def test_to_dict_with_properties(self, aspirin):
        """Test to_dict() with specific properties."""
        d = aspirin.to_dict(["cid", "molecular_formula"])
        assert set(d.keys()) == {"cid", "molecular_formula"}

    def test_repr(self, aspirin):
        """Test __repr__ output."""
        r = repr(aspirin)
        assert "2244" in r
        assert "Compound" in r

    def test_equality(self):
        """Test Compound equality based on record."""
        compounds1 = get_compounds(2244)
        compounds2 = get_compounds(2244)
        assert compounds1[0] == compounds2[0]

    def test_inequality(self):
        """Test Compound inequality for different CIDs."""
        compounds1 = get_compounds(2244)
        compounds2 = get_compounds(241)
        assert compounds1[0] != compounds2[0]

    def test_returns_correct_type(self):
        """Test that get_compounds returns Compound instances."""
        compounds = get_compounds(2244)
        assert len(compounds) > 0
        assert isinstance(compounds[0], Compound)
        assert compounds[0].cid == 2244

    def test_record_accessor(self, aspirin):
        """Test that record accessor returns CompoundRecord."""
        rec = aspirin.record
        d = rec.to_dict()
        assert isinstance(d, dict)
        assert "atoms" in d
