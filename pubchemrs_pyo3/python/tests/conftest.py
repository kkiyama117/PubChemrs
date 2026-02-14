"""Shared fixtures for pubchemrs tests."""

import pytest

import pubchemrs


@pytest.fixture(scope="session")
def client():
    """Shared PubChemClient instance for all tests."""
    return pubchemrs.PubChemClient()
