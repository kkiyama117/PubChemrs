"""Test error handling (adapted from pubchempy test_errors.py)."""

import pytest

from pubchemrs import PubChemClient, PubChemNotFoundError, PubChemAPIError


@pytest.mark.network
class TestErrors:
    """Test error handling for invalid requests."""

    def test_invalid_identifier(self, client):
        """Test that an invalid identifier raises an API error."""
        with pytest.raises((PubChemAPIError, ValueError)):
            client.get_properties_sync(
                "xxx_invalid_xxx",
                ["MolecularFormula"],
                "cid",
            )

    def test_notfound_identifier(self, client):
        """Test that a nonexistent compound name raises NotFoundError."""
        with pytest.raises(PubChemNotFoundError):
            client.get_properties_sync(
                "xxxxnotarealcompoundname",
                ["MolecularFormula"],
                "name",
            )

    def test_invalid_namespace(self):
        """Test that an invalid namespace raises ValueError."""
        client = PubChemClient()
        with pytest.raises(ValueError):
            client.get_properties_sync(
                2244,
                ["MolecularFormula"],
                "invalid_namespace",
            )

    def test_invalid_identifier_type(self):
        """Test that an invalid identifier type raises TypeError."""
        client = PubChemClient()
        with pytest.raises(TypeError):
            client.get_properties_sync(
                3.14,  # float is not a valid identifier
                ["MolecularFormula"],
                "cid",
            )
