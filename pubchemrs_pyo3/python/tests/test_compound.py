"""Test Compound class (adapted from pubchempy test_compound.py)."""

import pytest

from pubchemrs import Compound


@pytest.mark.network
class TestCompound:
    """Test the Compound convenience class."""

    @pytest.fixture(scope="class")
    def benzene(self):
        """Fetch benzene (CID 241) as a Compound."""
        return Compound.from_cid(241)

    @pytest.fixture(scope="class")
    def aspirin(self):
        """Fetch aspirin (CID 2244) as a Compound."""
        return Compound.from_cid(2244)

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
        assert aspirin.isomeric_smiles is not None

    def test_properties_types(self, aspirin):
        """Test that property types are correct."""
        assert isinstance(aspirin.molecular_weight, float)
        assert isinstance(aspirin.iupac_name, str)
        assert isinstance(aspirin.xlogp, float)
        assert isinstance(aspirin.tpsa, float)
        assert isinstance(aspirin.complexity, float)
        assert isinstance(aspirin.h_bond_donor_count, int)
        assert isinstance(aspirin.h_bond_acceptor_count, int)
        assert isinstance(aspirin.rotatable_bond_count, int)
        assert isinstance(aspirin.heavy_atom_count, int)

    def test_to_dict(self, aspirin):
        """Test to_dict() method."""
        d = aspirin.to_dict()
        assert isinstance(d, dict)
        assert d["cid"] == 2244
        assert d["molecular_formula"] == "C9H8O4"
        assert "molecular_weight" in d
        assert "inchikey" in d
        assert "isomeric_smiles" in d

    def test_repr(self, aspirin):
        """Test __repr__ output."""
        r = repr(aspirin)
        assert "2244" in r
        assert "Compound" in r

    def test_equality(self):
        """Test Compound equality based on CID."""
        c1 = Compound.from_cid(2244)
        c2 = Compound.from_cid(2244)
        assert c1 == c2

    def test_inequality(self):
        """Test Compound inequality for different CIDs."""
        c1 = Compound.from_cid(2244)
        c2 = Compound.from_cid(241)
        assert c1 != c2

    def test_from_cid_returns_correct_type(self):
        """Test that from_cid returns a Compound instance."""
        c = Compound.from_cid(2244)
        assert isinstance(c, Compound)
        assert c.cid == 2244
