"""pubchemrs - Rust-powered PubChem API client for Python."""

from pubchemrs._pubchemrs import (
    PubChemClient,
    CompoundProperties,
    Compound,
    CompoundRecord,
)
from pubchemrs.exceptions import PubChemError, PubChemAPIError, PubChemNotFoundError
from pubchemrs._core import _get_default_client

__all__ = [
    "PubChemClient",
    "Compound",
    "CompoundRecord",
    "CompoundProperties",
    "PubChemError",
    "PubChemAPIError",
    "PubChemNotFoundError",
    "get_compounds",
    "get_compounds_async",
    "get_properties",
    "get_properties_async",
    "get_synonyms",
    "get_synonyms_async",
    "get_all_sources",
    "get_all_sources_async",
    "compound_to_series",
    "compounds_to_frame",
]

__version__ = "0.1.0"


def get_compounds(identifier, namespace="cid"):
    """Retrieve full compound records from PubChem (synchronous).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        namespace: Namespace string (default: "cid").

    Returns:
        List of raw Compound objects.
    """
    return _get_default_client().get_compounds_sync(identifier, namespace)


async def get_compounds_async(identifier, namespace="cid"):
    """Retrieve full compound records from PubChem (async).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        namespace: Namespace string (default: "cid").

    Returns:
        List of raw Compound objects.
    """
    return await _get_default_client().get_compounds(identifier, namespace)


def get_properties(identifier, properties, namespace="cid"):
    """Retrieve compound properties from PubChem (synchronous).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        properties: List of property name strings.
        namespace: Namespace string (default: "cid").

    Returns:
        List of CompoundProperties objects.
    """
    return _get_default_client().get_properties_sync(identifier, properties, namespace)


async def get_properties_async(identifier, properties, namespace="cid"):
    """Retrieve compound properties from PubChem (async).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        properties: List of property name strings.
        namespace: Namespace string (default: "cid").

    Returns:
        List of CompoundProperties objects.
    """
    return await _get_default_client().get_properties(identifier, properties, namespace)


def get_synonyms(identifier, namespace="cid"):
    """Retrieve synonyms for compounds (synchronous).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        namespace: Namespace string (default: "cid").

    Returns:
        List of PubChemInformation objects with synonym data.
    """
    return _get_default_client().get_synonyms_sync(identifier, namespace)


async def get_synonyms_async(identifier, namespace="cid"):
    """Retrieve synonyms for compounds (async).

    Args:
        identifier: CID (int), name (str), or list of CIDs.
        namespace: Namespace string (default: "cid").

    Returns:
        List of PubChemInformation objects with synonym data.
    """
    return await _get_default_client().get_synonyms(identifier, namespace)


def get_all_sources(domain=None):
    """Retrieve all source names for a domain (synchronous).

    Args:
        domain: Domain string ("substance" or "assay"). Defaults to "substance".

    Returns:
        List of source name strings.
    """
    return _get_default_client().get_all_sources_sync(domain)


async def get_all_sources_async(domain=None):
    """Retrieve all source names for a domain (async).

    Args:
        domain: Domain string ("substance" or "assay"). Defaults to "substance".

    Returns:
        List of source name strings.
    """
    return await _get_default_client().get_all_sources(domain)


def compound_to_series(compound, properties=None):
    """Convert a Compound to a pandas Series.

    Args:
        compound: A Compound object.
        properties: Optional list of property names to include.

    Returns:
        pandas Series containing compound data.
    """
    import pandas as pd

    return pd.Series(compound.to_dict(properties))


def compounds_to_frame(compounds, properties=None):
    """Convert a list of Compounds to a pandas DataFrame.

    Args:
        compounds: A Compound or list of Compound objects.
        properties: Optional list of property names to include as columns.

    Returns:
        pandas DataFrame indexed by CID.
    """
    import pandas as pd

    if not isinstance(compounds, list):
        compounds = [compounds]
    return pd.DataFrame.from_records(
        [c.to_dict(properties) for c in compounds], index="cid"
    )
