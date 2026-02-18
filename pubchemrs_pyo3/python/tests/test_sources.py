"""Test data source retrieval (adapted from pubchempy test_sources.py)."""

import pytest

import pubchemrs


@pytest.mark.network
class TestSources:
    """Test fetching source lists from PubChem API."""

    def test_substance_sources(self, client):
        """Test retrieving substance source names."""
        sources = client.get_all_sources_sync()
        assert len(sources) > 20
        lower_sources = [s.lower() for s in sources]
        # These are well-known, long-standing PubChem sources
        assert any("zinc" in s for s in lower_sources)

    def test_assay_sources(self, client):
        """Test retrieving assay source names."""
        sources = client.get_all_sources_sync("assay")
        assert len(sources) > 20
        lower_sources = [s.lower() for s in sources]
        assert any("chembl" in s for s in lower_sources)

    def test_substance_sources_default(self, client):
        """Test that default domain is substance."""
        sources_default = client.get_all_sources_sync()
        sources_explicit = client.get_all_sources_sync("substance")
        # Both should return the same list
        assert sources_default == sources_explicit
