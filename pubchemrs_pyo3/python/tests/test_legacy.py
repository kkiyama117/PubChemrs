"""Test legacy pubchempy-compatible API (adapted from pubchempy tests)."""

import pytest

from pubchemrs.legacy import get_compounds, get_properties, get_synonyms, get_all_sources


@pytest.mark.network
class TestLegacyGetCompounds:
    """Test legacy get_compounds function."""

    def test_by_cid(self):
        """Test retrieving compound by CID."""
        compounds = get_compounds(2244, "cid")
        assert len(compounds) == 1
        c = compounds[0]
        assert c.cid == 2244
        assert c.molecular_formula == "C9H8O4"

    def test_by_name(self):
        """Test retrieving compound by name."""
        compounds = get_compounds("aspirin", "name")
        assert len(compounds) >= 1
        assert any(c.cid == 2244 for c in compounds)

    def test_by_smiles(self):
        """Test retrieving compound by SMILES."""
        compounds = get_compounds("CC(=O)OC1=CC=CC=C1C(=O)O", "smiles")
        assert len(compounds) >= 1


@pytest.mark.network
class TestLegacyGetProperties:
    """Test legacy get_properties function."""

    def test_properties_dict(self):
        """Test that get_properties returns dicts with expected keys."""
        results = get_properties(
            ["MolecularFormula", "MolecularWeight"],
            2244,
            "cid",
        )
        assert len(results) == 1
        d = results[0]
        assert "CID" in d
        assert d["CID"] == 2244
        assert "MolecularFormula" in d
        assert "MolecularWeight" in d


@pytest.mark.network
class TestLegacyGetSynonyms:
    """Test legacy get_synonyms function."""

    def test_synonyms_dict(self):
        """Test that get_synonyms returns dicts with CID and Synonym."""
        results = get_synonyms(2244, "cid")
        assert len(results) >= 1
        d = results[0]
        assert "CID" in d
        assert d["CID"] == 2244
        assert "Synonym" in d
        assert len(d["Synonym"]) > 5
        lower_synonyms = [s.lower() for s in d["Synonym"]]
        assert "aspirin" in lower_synonyms


@pytest.mark.network
class TestLegacyGetAllSources:
    """Test legacy get_all_sources function."""

    def test_substance_sources(self):
        """Test retrieving substance sources."""
        sources = get_all_sources("substance")
        assert len(sources) > 20

    def test_assay_sources(self):
        """Test retrieving assay sources."""
        sources = get_all_sources("assay")
        assert len(sources) > 20
