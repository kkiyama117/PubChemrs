"""Test compound property retrieval (adapted from pubchempy test_properties.py)."""

import pytest

import pubchemrs


@pytest.mark.network
class TestProperties:
    """Test fetching compound properties from PubChem API."""

    def test_properties_by_name(self, client):
        """Test retrieving properties by compound name."""
        results = client.get_properties_sync(
            "aspirin",
            ["SMILES", "InChIKey"],
            "name",
        )
        assert len(results) >= 1
        props = results[0]
        assert props.cid > 0
        assert props.smiles is not None
        assert props.inchikey is not None

    def test_properties_by_cid(self, client):
        """Test retrieving properties by CID."""
        results = client.get_properties_sync(
            2244,
            ["MolecularFormula", "MolecularWeight", "InChIKey"],
            "cid",
        )
        assert len(results) == 1
        props = results[0]
        assert props.cid == 2244
        assert props.molecular_formula == "C9H8O4"
        assert props.molecular_weight is not None
        assert abs(props.molecular_weight - 180.16) < 0.1
        assert props.inchikey is not None

    def test_multiple_properties(self, client):
        """Test retrieving many properties at once."""
        results = client.get_properties_sync(
            2244,
            [
                "MolecularFormula",
                "MolecularWeight",
                "SMILES",
                "InChI",
                "InChIKey",
                "IUPACName",
                "XLogP",
                "TPSA",
                "Complexity",
                "HBondDonorCount",
                "HBondAcceptorCount",
                "RotatableBondCount",
                "HeavyAtomCount",
                "ExactMass",
            ],
            "cid",
        )
        assert len(results) == 1
        props = results[0]
        assert props.cid == 2244
        assert props.molecular_formula == "C9H8O4"
        assert isinstance(props.molecular_weight, float)
        assert props.smiles is not None
        assert props.inchi is not None
        assert props.inchikey is not None
        assert props.iupac_name is not None
        assert isinstance(props.xlogp, float)
        assert isinstance(props.tpsa, float)
        assert isinstance(props.complexity, float)
        assert isinstance(props.h_bond_donor_count, int)
        assert isinstance(props.h_bond_acceptor_count, int)
        assert isinstance(props.rotatable_bond_count, int)
        assert isinstance(props.heavy_atom_count, int)
        assert isinstance(props.exact_mass, float)

    def test_properties_multiple_cids(self, client):
        """Test retrieving properties for multiple CIDs."""
        results = client.get_properties_sync(
            [2244, 5793],
            ["MolecularFormula"],
            "cid",
        )
        assert len(results) == 2
        cids = {p.cid for p in results}
        assert cids == {2244, 5793}

    def test_properties_by_smiles(self, client):
        """Test retrieving properties by SMILES."""
        results = client.get_properties_sync(
            "CC(=O)OC1=CC=CC=C1C(=O)O",
            ["MolecularFormula", "InChIKey"],
            "smiles",
        )
        assert len(results) >= 1
        assert results[0].molecular_formula is not None


@pytest.mark.network
class TestSynonyms:
    """Test fetching synonyms from PubChem API."""

    def test_synonyms_by_cid(self, client):
        """Test retrieving synonyms for aspirin (CID 2244)."""
        results = client.get_synonyms_sync(2244, "cid")
        assert len(results) >= 1
        info = results[0]
        assert info.cid == 2244
        assert len(info.synonym) > 5
        lower_synonyms = [s.lower() for s in info.synonym]
        assert "aspirin" in lower_synonyms

    def test_synonyms_by_name(self, client):
        """Test retrieving synonyms by name."""
        results = client.get_synonyms_sync("caffeine", "name")
        assert len(results) >= 1
        assert len(results[0].synonym) > 0
