"""pubchemrs.legacy - pubchempy-compatible API.

Provides drop-in replacement functions for pubchempy.
Phase 1 covers: get_compounds, get_properties, get_synonyms, get_all_sources.
"""

from __future__ import annotations

from pubchemrs._core import _get_default_client
from pubchemrs.compound import Compound


def get_compounds(
    identifier,
    namespace="cid",
    searchtype=None,
    **kwargs,
) -> list[Compound]:
    """Retrieve compounds from PubChem (pubchempy-compatible).

    Args:
        identifier: CID (int), name (str), or list of identifiers.
        namespace: Namespace string (default: "cid").
            Common values: "cid", "name", "smiles", "inchi", "inchikey", "formula".
        searchtype: Ignored in Phase 1 (for future substructure/superstructure search).
        **kwargs: Additional keyword arguments (reserved for future use).

    Returns:
        List of Compound objects with pubchempy-compatible property access.
    """
    # Map pubchempy-style namespaces
    ns = _map_namespace(namespace)

    client = _get_default_client()
    props_list = client.get_properties_sync(
        identifier,
        [
            "MolecularFormula",
            "MolecularWeight",
            "InChIKey",
            "InChI",
            "SMILES",
            "ConnectivitySMILES",
            "IUPACName",
            "ExactMass",
            "MonoisotopicMass",
            "TPSA",
            "XLogP",
            "Complexity",
            "HBondDonorCount",
            "HBondAcceptorCount",
            "RotatableBondCount",
            "HeavyAtomCount",
            "Charge",
        ],
        ns,
    )
    return [Compound(p) for p in props_list]


def get_properties(
    properties: list[str],
    identifier,
    namespace: str = "cid",
    **kwargs,
) -> list[dict]:
    """Retrieve specific properties from PubChem (pubchempy-compatible).

    Args:
        properties: List of property name strings.
        identifier: CID (int), name (str), or list of identifiers.
        namespace: Namespace string (default: "cid").
        **kwargs: Additional keyword arguments (reserved for future use).

    Returns:
        List of dictionaries with requested properties.
    """
    ns = _map_namespace(namespace)
    client = _get_default_client()
    props_list = client.get_properties_sync(identifier, properties, ns)
    results = []
    for p in props_list:
        d = {"CID": p.cid}
        for prop_name in properties:
            attr = _property_to_attr(prop_name)
            if hasattr(p, attr):
                d[prop_name] = getattr(p, attr)
        results.append(d)
    return results


def get_synonyms(
    identifier,
    namespace: str = "cid",
    **kwargs,
) -> list[dict]:
    """Retrieve synonyms from PubChem (pubchempy-compatible).

    Args:
        identifier: CID (int), name (str), or list of identifiers.
        namespace: Namespace string (default: "cid").
        **kwargs: Additional keyword arguments (reserved for future use).

    Returns:
        List of dictionaries with CID and Synonym keys.
    """
    ns = _map_namespace(namespace)
    client = _get_default_client()
    info_list = client.get_synonyms_sync(identifier, ns)
    return [{"CID": info.cid, "Synonym": info.synonym} for info in info_list]


def get_all_sources(domain: str = "substance") -> list[str]:
    """Retrieve all source names (pubchempy-compatible).

    Args:
        domain: "substance" or "assay" (default: "substance").

    Returns:
        List of source name strings.
    """
    client = _get_default_client()
    return client.get_all_sources_sync(domain if domain != "substance" else None)


def _map_namespace(namespace: str) -> str:
    """Map pubchempy-style namespace names to pubchemrs namespace strings."""
    mapping = {
        "cid": "cid",
        "name": "name",
        "smiles": "smiles",
        "sdf": "sdf",
        "inchi": "inchi",
        "inchikey": "inchikey",
        "formula": "formula",
        "sid": "sid",
    }
    return mapping.get(namespace.lower(), namespace)


def _property_to_attr(prop_name: str) -> str:
    """Convert PubChem PascalCase property names to snake_case attribute names."""
    import re

    # Handle known acronyms
    s = prop_name.replace("IUPAC", "Iupac").replace("SMILES", "Smiles")
    s = s.replace("TPSA", "Tpsa").replace("XLogP", "Xlogp")
    s = s.replace("InChI", "Inchi").replace("InChIKey", "Inchikey")
    # CamelCase to snake_case
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", s)
    s = re.sub(r"(?<=[A-Z])([A-Z][a-z])", r"_\1", s)
    return s.lower()
