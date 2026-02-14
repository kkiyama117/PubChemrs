"""Test async API methods."""

import pytest

import pubchemrs


@pytest.mark.network
class TestAsync:
    """Test async variants of the API."""

    @pytest.mark.asyncio
    async def test_get_properties_async(self):
        """Test async property retrieval."""
        client = pubchemrs.PubChemClient()
        results = await client.get_properties(
            2244,
            ["MolecularFormula", "MolecularWeight"],
            "cid",
        )
        assert len(results) == 1
        assert results[0].cid == 2244
        assert results[0].molecular_formula == "C9H8O4"

    @pytest.mark.asyncio
    async def test_get_synonyms_async(self):
        """Test async synonym retrieval."""
        client = pubchemrs.PubChemClient()
        results = await client.get_synonyms(2244, "cid")
        assert len(results) >= 1
        assert results[0].cid == 2244
        assert len(results[0].synonym) > 0

    @pytest.mark.asyncio
    async def test_get_all_sources_async(self):
        """Test async source retrieval."""
        client = pubchemrs.PubChemClient()
        sources = await client.get_all_sources()
        assert len(sources) > 20

    @pytest.mark.asyncio
    async def test_module_level_async(self):
        """Test module-level async convenience functions."""
        results = await pubchemrs.get_properties_async(
            2244,
            ["MolecularFormula"],
            "cid",
        )
        assert len(results) == 1
        assert results[0].cid == 2244
