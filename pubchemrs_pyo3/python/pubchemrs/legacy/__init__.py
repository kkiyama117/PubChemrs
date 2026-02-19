from __future__ import annotations

import enum
import json
import logging
import os
import ssl
import time
import typing as t
from http.client import HTTPResponse
from urllib.error import HTTPError
from urllib.parse import quote, urlencode
from urllib.request import urlopen

from .compound import (
    ELEMENTS,
    Atom,
    Bond,
    BondType,
    Compound,
    CompoundIdType,
    CoordinateType,
    compounds_to_frame,
    deprecated,
    get_compounds,
    memoized_property,
)
from .errors import (
    BadRequestError,
    MethodNotAllowedError,
    NotFoundError,
    PubChemHTTPError,
    PubChemPyDeprecationWarning,
    PubChemPyError,
    ResponseParseError,
    ServerBusyError,
    ServerError,
    TimeoutError,
    UnimplementedError,
    create_http_error,
)

if t.TYPE_CHECKING:
    import pandas as pd

# Get SSL certs from env var or certifi package if available.
_CA_FILE = os.getenv("PUBCHEMPY_CA_BUNDLE") or os.getenv("REQUESTS_CA_BUNDLE")
if not _CA_FILE:
    try:
        import certifi

        _CA_FILE = certifi.where()
    except ImportError:
        _CA_FILE = None


__author__ = "kkiyama117"
__email__ = "k.kiyama117@gmail.com"
__version__ = "1.0.5"
__license__ = "MIT"

__all__ = [
    # Main API functions
    "get_compounds",
    "get_substances",
    "get_assays",
    "get_properties",
    "get_synonyms",
    "get_cids",
    "get_sids",
    "get_aids",
    "get_all_sources",
    "download",
    "request",
    "get",
    "get_json",
    "get_sdf",
    # Core classes
    "Compound",
    "Substance",
    "Assay",
    "Atom",
    "Bond",
    # Enum/constant classes
    "CompoundIdType",
    "BondType",
    "CoordinateType",
    "ProjectCategory",
    # Data conversion functions
    "compounds_to_frame",
    "substances_to_frame",
    # Constants
    "API_BASE",
    "ELEMENTS",
    "PROPERTY_MAP",
    # Exceptions
    "PubChemPyError",
    "ResponseParseError",
    "PubChemHTTPError",
    "BadRequestError",
    "NotFoundError",
    "MethodNotAllowedError",
    "ServerError",
    "UnimplementedError",
    "ServerBusyError",
    "TimeoutError",
    "PubChemPyDeprecationWarning",
]

#: Base URL for the PubChem PUG REST API.
API_BASE = "https://pubchem.ncbi.nlm.nih.gov/rest/pug"

log = logging.getLogger("pubchempy")
log.addHandler(logging.NullHandler())

#: Type alias for URL query parameters.
QueryParam = str | int | float | bool | list[str] | None


class ProjectCategory(enum.IntEnum):
    """To distinguish projects funded through MLSCN, MLPCN or other."""

    #: Assay depositions from MLSCN screen center
    MLSCN = 1
    #: Assay depositions from MLPCN screen center
    MLPCN = 2
    #: Assay depositions from MLSCN assay provider
    MLSCN_AP = 3
    #: Assay depositions from MLPCN assay provider
    MLPCN_AP = 4
    #: To be deprecated and replaced by options 7, 8 & 9
    JOURNAL_ARTICLE = 5
    #: Assay depositions from assay vendors
    ASSAY_VENDOR = 6
    #: Data from literature, extracted by curators
    LITERATURE_EXTRACTED = 7
    #: Data from literature, submitted by author of articles
    LITERATURE_AUTHOR = 8
    #: Data from literature, submitted by journals/publishers
    LITERATURE_PUBLISHER = 9
    #: RNAi screenings from RNAi Global Initiative
    RNAIGI = 10
    #: Other project category
    OTHER = 255


def request(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    operation: str | None = None,
    output: str = "JSON",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> HTTPResponse:
    """Construct API request from parameters and return the response.

    Full specification at https://pubchem.ncbi.nlm.nih.gov/docs/pug-rest
    """
    if not identifier:
        raise ValueError("identifier/cid cannot be None")
    # If identifier is a list, join with commas into string
    if isinstance(identifier, int):
        identifier = str(identifier)
    if not isinstance(identifier, str):
        identifier = ",".join(str(x) for x in identifier)
    # Filter None values from kwargs
    kwargs = {k: v for k, v in kwargs.items() if v is not None}
    # Build API URL
    urlid, postdata = None, None
    if namespace == "sourceid":
        identifier = identifier.replace("/", ".")
    if (
        namespace in ["listkey", "formula", "sourceid"]
        or searchtype == "xref"
        or (searchtype and namespace == "cid")
        or domain == "sources"
    ):
        urlid = quote(identifier.encode("utf8"))
    else:
        postdata = urlencode([(namespace, identifier)]).encode("utf8")
    comps = filter(None, [API_BASE, domain, searchtype, namespace, urlid, operation, output])
    apiurl = "/".join(comps)
    if kwargs:
        apiurl += f"?{urlencode(kwargs)}"
    # Make request
    try:
        log.debug(f"Request URL: {apiurl}")
        log.debug(f"Request data: {postdata}")
        context = ssl.create_default_context(cafile=_CA_FILE)
        response = urlopen(apiurl, postdata, context=context)
        return response
    except HTTPError as e:
        raise create_http_error(e) from e


def get(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    operation: str | None = None,
    output: str = "JSON",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> bytes:
    """Request wrapper that automatically handles async requests."""
    if (searchtype and searchtype != "xref") or namespace in ["formula"]:
        response = request(identifier, namespace, domain, None, "JSON", searchtype, **kwargs).read()
        status = json.loads(response.decode())
        if "Waiting" in status and "ListKey" in status["Waiting"]:
            identifier = status["Waiting"]["ListKey"]
            namespace = "listkey"
            while "Waiting" in status and "ListKey" in status["Waiting"]:
                time.sleep(2)
                response = request(identifier, namespace, domain, operation, "JSON", **kwargs).read()
                status = json.loads(response.decode())
            if not output == "JSON":
                response = request(
                    identifier,
                    namespace,
                    domain,
                    operation,
                    output,
                    searchtype,
                    **kwargs,
                ).read()
    else:
        response = request(identifier, namespace, domain, operation, output, searchtype, **kwargs).read()
    return response


def get_json(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    operation: str | None = None,
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> dict[str, t.Any] | None:
    """Request wrapper that automatically parses JSON response into a python dict.

    This function suppresses NotFoundError and returns None if no results are found.
    """
    try:
        return json.loads(get(identifier, namespace, domain, operation, "JSON", searchtype, **kwargs).decode())
    except NotFoundError as e:
        log.info(e)
        return None


def get_sdf(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    operation: str | None = None,
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> str | None:
    """Request wrapper that automatically extracts SDF from the response.

    This function suppresses NotFoundError and returns None if no results are found.
    """
    try:
        return get(identifier, namespace, domain, operation, "SDF", searchtype, **kwargs).decode()
    except NotFoundError as e:
        log.info(e)
        return None


def get_substances(
    identifier: str | int | list[str | int],
    namespace: str = "sid",
    as_dataframe: bool = False,
    **kwargs: QueryParam,
) -> list[Substance] | pd.DataFrame:
    """Retrieve the specified substance records from PubChem.

    Args:
        identifier: The substance identifier to use as a search query.
        namespace: The identifier type, one of sid, name or sourceid/<source name>.
        as_dataframe: Automatically extract the Substance properties into a pandas
            DataFrame and return that.
        **kwargs: Additional query parameters to pass to the API request.

    Returns:
        List of :class:`~pubchempy.Substance` objects, or a pandas DataFrame if
        ``as_dataframe=True``.
    """
    results = get_json(identifier, namespace, "substance", **kwargs)
    substances = [Substance(r) for r in results["PC_Substances"]] if results else []
    if as_dataframe:
        return substances_to_frame(substances)
    return substances


def get_assays(
    identifier: str | int | list[str | int],
    namespace: str = "aid",
    **kwargs: QueryParam,
) -> list[Assay]:
    """Retrieve the specified assay records from PubChem.

    Args:
        identifier: The assay identifier to use as a search query.
        namespace: The identifier type.
        **kwargs: Additional query parameters to pass to the API request.

    Returns:
        List of :class:`~pubchempy.Assay` objects.
    """
    results = get_json(identifier, namespace, "assay", "description", **kwargs)
    return [Assay(r) for r in results["PC_AssayContainer"]] if results else []


from pubchemrs._pubchemrs import PROPERTY_MAP as _RUST_PROPERTY_MAP

#: Dictionary mapping property names to their PubChem API equivalents.
#:
#: Built from Rust-generated map with legacy alias added for backward compat.
PROPERTY_MAP: dict[str, str] = {
    **_RUST_PROPERTY_MAP,
    # Legacy alias kept for backward compatibility with Compound.conformer_rmsd_3d
    "conformer_rmsd_3d": "ConformerModelRMSD3D",
}


def get_properties(
    properties: str | list[str],
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    searchtype: str | None = None,
    as_dataframe: bool = False,
    **kwargs: QueryParam,
) -> list[dict[str, t.Any]] | pd.DataFrame:
    """Retrieve the specified compound properties from PubChem.

    Args:
        properties: The properties to retrieve.
        identifier: The compound  identifier to use as a search query.
        namespace: The identifier type.
        searchtype: The advanced search type, one of substructure, superstructure
            or similarity.
        as_dataframe: Automatically extract the properties into a pandas DataFrame.
        **kwargs: Additional query parameters to pass to the API request.
    """
    if isinstance(properties, str):
        properties = properties.split(",")
    properties = ",".join([PROPERTY_MAP.get(p, p) for p in properties])
    properties = f"property/{properties}"
    results = get_json(identifier, namespace, "compound", properties, searchtype=searchtype, **kwargs)
    results = results["PropertyTable"]["Properties"] if results else []
    if as_dataframe:
        import pandas as pd

        return pd.DataFrame.from_records(results, index="CID")
    return results


def get_synonyms(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> list[dict[str, t.Any]]:
    """Retrieve synonyms (alternative names) for the specified records from PubChem.

    Synonyms include systematic names, common names, trade names, registry numbers,
    and other identifiers associated with compounds, substances, or assays.

    Args:
        identifier: The identifier to use as a search query.
        namespace: The identifier type (e.g., cid, name, smiles for compounds).
        domain: The PubChem domain to search (compound or substance).
        searchtype: The advanced search type, one of substructure, superstructure
            or similarity.
        **kwargs: Additional parameters to pass to the request.

    Returns:
        List of dictionaries containing synonym information for each matching record.
        Each dictionary contains the record identifier and a list of synonyms.
    """
    results = get_json(identifier, namespace, domain, "synonyms", searchtype=searchtype, **kwargs)
    return results["InformationList"]["Information"] if results else []


def get_cids(
    identifier: str | int | list[str | int],
    namespace: str = "name",
    domain: str = "compound",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> list[int]:
    """Retrieve Compound Identifiers (CIDs) for the specified query from PubChem.

    CIDs are unique numerical identifiers assigned to each standardized compound
    record in the PubChem Compound database. This function is useful for converting
    between different identifier types (names, SMILES, InChI, etc.) and CIDs.

    Args:
        identifier: The identifier to use as a search query.
        namespace: The identifier type (e.g. name, smiles, inchi, formula).
        domain: The PubChem domain to search (compound, substance, or assay).
        searchtype: The advanced search type, one of substructure, superstructure
            or similarity.
        **kwargs: Additional parameters to pass to the request.

    Returns:
        List of CIDs (integers) that match the search criteria. Empty list if no
        matches found.
    """
    results = get_json(identifier, namespace, domain, "cids", searchtype=searchtype, **kwargs)
    if not results:
        return []
    elif "IdentifierList" in results:
        return results["IdentifierList"]["CID"]
    elif "InformationList" in results:
        return results["InformationList"]["Information"]


def get_sids(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> list[int]:
    """Retrieve Substance Identifiers (SIDs) for the specified query from PubChem.

    SIDs are unique numerical identifiers assigned to each substance record
    in the PubChem Substance database. This function is useful for finding
    which substance records are associated with a given compound or other identifier.

    Args:
        identifier: The identifier to use as a search query.
        namespace: The identifier type (e.g., cid, name, smiles for compounds).
        domain: The PubChem domain to search (compound, substance, or assay).
        searchtype: The advanced search type, one of substructure, superstructure
            or similarity.
        **kwargs: Additional parameters to pass to the request.

    Returns:
        List of SIDs (integers) that match the search criteria. Empty list if no
        matches found.
    """
    results = get_json(identifier, namespace, domain, "sids", searchtype=searchtype, **kwargs)
    if not results:
        return []
    elif "IdentifierList" in results:
        return results["IdentifierList"]["SID"]
    elif "InformationList" in results:
        return results["InformationList"]["Information"]


def get_aids(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    searchtype: str | None = None,
    **kwargs: QueryParam,
) -> list[int]:
    """Retrieve Assay Identifiers (AIDs) for the specified query from PubChem.

    AIDs are unique numerical identifiers assigned to each biological assay
    record in the PubChem BioAssay database. This function is useful for finding
    which assays have tested a given compound or substance.

    Args:
        identifier: The identifier to use as a search query.
        namespace: The identifier type (e.g., cid, name, smiles).
        domain: The PubChem domain to search (compound, substance, or assay).
        searchtype: The advanced search type, one of substructure, superstructure
            or similarity.
        **kwargs: Additional parameters to pass to the request.

    Returns:
        List of AIDs (integers) that match the search criteria. Empty list if no
        matches found.
    """
    results = get_json(identifier, namespace, domain, "aids", searchtype=searchtype, **kwargs)
    if not results:
        return []
    elif "IdentifierList" in results:
        return results["IdentifierList"]["AID"]
    elif "InformationList" in results:
        return results["InformationList"]["Information"]


def get_all_sources(domain: str = "substance") -> list[str]:
    """Return a list of all current depositors of substances or assays."""
    results = json.loads(get(domain, None, "sources").decode())
    return results["InformationList"]["SourceName"]


def download(
    outformat: str,
    path: str | os.PathLike,
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    domain: str = "compound",
    operation: str | None = None,
    searchtype: str | None = None,
    overwrite: bool = False,
    **kwargs: QueryParam,
) -> None:
    """Format can be  XML, ASNT/B, JSON, SDF, CSV, PNG, TXT."""
    response = get(identifier, namespace, domain, operation, outformat, searchtype, **kwargs)
    if not overwrite and os.path.isfile(path):
        raise OSError(f"{path} already exists. Use 'overwrite=True' to overwrite it.")
    with open(path, "wb") as f:
        f.write(response)


class Substance:
    """Represents a raw chemical record as originally deposited to PubChem.

    The PubChem Substance database contains chemical records in their original deposited
    form, before standardization or processing. As a result, it contains duplicates,
    mixtures, and some records that don't make chemical sense. This means that Substance
    records contain fewer calculated properties, however they do have additional
    information about the original source that deposited the record.

    During PubChem's standardization process, Substances are processed to create
    standardized Compound records. Multiple Substances may map to the same Compound
    if they represent the same unique chemical structure. Some Substances may not
    map to any Compound if they cannot be standardized.

    Examples:
        >>> substance = Substance.from_sid(12345)
        >>> print(f"Source: {substance.source_name}")
        Source: KEGG
        >>> print(f"Depositor ID: {substance.source_id}")
        Depositor ID: C10159
        >>> print(f"Standardized to CID: {substance.standardized_cid}")
        Standardized to CID: 169683
    """

    def __init__(self, record: dict[str, t.Any]) -> None:
        """Initialize a Substance with a record dict from the PubChem PUG REST service.

        Args:
            record: Substance record returned by the PubChem PUG REST service.

        Note:
            Most users will not need to instantiate a Substance instance directly from a
            record. The :meth:`from_sid()` class method and the
            :func:`~get_substances()` function offer more convenient ways to obtain
            Substance instances, as they also handle the retrieval of the record from
            PubChem.
        """
        self._record = record

    @classmethod
    def from_sid(cls, sid: int, **kwargs: QueryParam) -> Substance:
        """Retrieve the Substance record for the specified SID.

        Args:
            sid: The PubChem Substance Identifier (SID).
            **kwargs: Additional parameters to pass to the request.

        Example:
            s = Substance.from_sid(12345)
        """
        response = request(sid, "sid", "substance", **kwargs).read().decode()
        record = json.loads(response)["PC_Substances"][0]
        return cls(record)

    @property
    def record(self) -> dict[str, t.Any]:
        """The full substance record returned by the PubChem PUG REST service."""
        return self._record

    def __repr__(self) -> str:
        return f"Substance({self.sid if self.sid else ''})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, type(self)) and self.record == other.record

    def to_dict(self, properties: list[str] | None = None) -> dict[str, t.Any]:
        """Return a dict containing Substance property data.

        Optionally specify a list of the desired properties to include. If
        ``properties`` is not specified, all properties are included, with the following
        exceptions:

        :attr:`cids` and :attr:`aids` are not included unless explicitly specified. This
        is because they each require an extra request to the PubChem API to retrieve.

        Args:
            properties: List of desired properties.

        Returns:
            Dictionary of substance data.
        """
        if not properties:
            skip = {
                "record",
                "deposited_compound",
                "standardized_compound",
                "cids",
                "aids",
            }
            properties = [p for p, v in Substance.__dict__.items() if isinstance(v, property) and p not in skip]
        return {p: getattr(self, p) for p in properties}

    def to_series(self, properties: list[str] | None = None) -> pd.Series:
        """Return a pandas :class:`~pandas.Series` containing Substance data.

        Optionally specify a list of the desired properties to include as columns. If
        ``properties`` is not specified, all properties are included, with the following
        exceptions:

        :attr:`cids` and :attr:`aids` are not included unless explicitly specified. This
        is because they each require an extra request to the PubChem API to retrieve.

        Args:
            properties: List of desired properties.
        """
        import pandas as pd

        return pd.Series(self.to_dict(properties))

    @property
    def sid(self) -> int:
        """The PubChem Substance Idenfitier (SID)."""
        return self.record["sid"]["id"]

    @property
    def synonyms(self) -> list[str] | None:
        """A ranked list of all the names associated with this Substance."""
        if "synonyms" in self.record:
            return self.record["synonyms"]

    @property
    def source_name(self) -> str:
        """The name of the PubChem depositor that was the source of this Substance."""
        return self.record["source"]["db"]["name"]

    @property
    def source_id(self) -> str:
        """Unique ID for this Substance from the PubChem depositor source."""
        return self.record["source"]["db"]["source_id"]["str"]

    @property
    def standardized_cid(self) -> int | None:
        """The CID of the Compound that was standardized from this Substance.

        May not exist if this Substance was not standardizable.
        """
        for c in self.record.get("compound", []):
            if c["id"]["type"] == CompoundIdType.STANDARDIZED:
                return c["id"]["id"]["cid"]

    @memoized_property
    def standardized_compound(self) -> Compound | None:
        """The :class:`~pubchempy.Compound` that was standardized from this Substance.

        Requires an extra request. Result is cached. May not exist if this Substance was
        not standardizable.
        """
        cid = self.standardized_cid
        if cid:
            return Compound.from_cid(cid)

    @property
    def deposited_compound(self) -> None:
        """Not supported with Rust Compound backend.

        The deposited compound from the unstandardized Substance record cannot be
        constructed as a Rust Compound object. Use ``standardized_compound`` instead,
        or access the raw record directly via ``self.record['compound']``.
        """
        log.debug("deposited_compound is not supported with Rust Compound backend")
        return None

    @memoized_property
    def cids(self) -> list[int]:
        """A list of all CIDs for Compounds that were standardized from this Substance.

        Requires an extra request. Result is cached.
        """
        results = get_json(self.sid, "sid", "substance", "cids")
        return results["InformationList"]["Information"][0]["CID"] if results else []

    @memoized_property
    def aids(self) -> list[int]:
        """A list of all AIDs for Assays associated with this Substance.

        Requires an extra request. Result is cached.
        """
        results = get_json(self.sid, "sid", "substance", "aids")
        return results["InformationList"]["Information"][0]["AID"] if results else []


class Assay:
    """Represents a biological assay record from the PubChem BioAssay database.

    The PubChem BioAssay database contains experimental data from biological screening
    and testing programs. Each assay record describes the experimental conditions,
    methodology, and results for testing chemical compounds against biological targets.

    BioAssay records include:

    - Assay protocol and experimental conditions
    - Target information (proteins, genes, pathways)
    - Activity outcome definitions and thresholds
    - Results data linking compounds to biological activities
    - Source information and literature references

    Assays are identified by their AID (Assay Identifier) and can be retrieved
    using the ``from_aid()`` class method. The assay data provides the experimental
    context for understanding compound bioactivity data stored in PubChem.
    """

    def __init__(self, record: dict[str, t.Any]) -> None:
        """Initialize an Assay with a record dict from the PubChem PUG REST service.

        Args:
            record: Assay record returned by the PubChem PUG REST service.

        Note:
            Most users will not need to instantiate an Assay instance directly from a
            record. The :meth:`from_aid()` class method and the :func:`~get_assays()`
            function offer more convenient ways to obtain Assay instances, as they
            also handle the retrieval of the record from PubChem.
        """
        self._record = record

    @classmethod
    def from_aid(cls, aid: int, **kwargs: QueryParam) -> Assay:
        """Retrieve the Assay record for the specified AID.

        Args:
            aid: The PubChem Assay Identifier (AID).
            **kwargs: Additional parameters to pass to the request.

        Example:
            a = Assay.from_aid(1234)
        """
        response = request(aid, "aid", "assay", "description", **kwargs).read().decode()
        record = json.loads(response)["PC_AssayContainer"][0]
        return cls(record)

    @property
    def record(self) -> dict[str, t.Any]:
        """The full assay record returned by the PubChem PUG REST service."""
        return self._record

    def __repr__(self) -> str:
        return f"Assay({self.aid if self.aid else ''})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, type(self)) and self.record == other.record

    def to_dict(self, properties: list[str] | None = None) -> dict[str, t.Any]:
        """Return a dict containing Assay property data.

        Optionally specify a list of the desired properties to include. If
        ``properties`` is not specified, all properties are included.

        Args:
            properties: List of desired properties.

        Returns:
            Dictionary of assay data.
        """
        if not properties:
            skip = {"record"}
            properties = [p for p, v in Assay.__dict__.items() if isinstance(v, property) and p not in skip]
        return {p: getattr(self, p) for p in properties}

    @property
    def aid(self) -> int:
        """The PubChem Assay Idenfitier (AID)."""
        return self.record["assay"]["descr"]["aid"]["id"]

    @property
    def name(self) -> str:
        """The short assay name, used for display purposes."""
        return self.record["assay"]["descr"]["name"]

    @property
    def description(self) -> str:
        """Description."""
        return self.record["assay"]["descr"]["description"]

    @property
    def project_category(self) -> ProjectCategory | None:
        """Category to distinguish projects funded through MLSCN, MLPCN or other.

        Possible values include mlscn, mlpcn, mlscn-ap, mlpcn-ap, literature-extracted,
        literature-author, literature-publisher, rnaigi.
        """
        if "project_category" in self.record["assay"]["descr"]:
            return ProjectCategory(self.record["assay"]["descr"]["project_category"])

    @property
    def comments(self) -> list[str]:
        """Comments and additional information."""
        return [comment for comment in self.record["assay"]["descr"]["comment"] if comment]

    @property
    def results(self) -> list[dict[str, t.Any]]:
        """A list of dictionaries containing details of the results from this Assay."""
        return self.record["assay"]["descr"]["results"]

    @property
    def target(self) -> list[dict[str, t.Any]] | None:
        """A list of dictionaries containing details of the Assay targets."""
        if "target" in self.record["assay"]["descr"]:
            return self.record["assay"]["descr"]["target"]

    @property
    def revision(self) -> int:
        """Revision identifier for textual description."""
        return self.record["assay"]["descr"]["revision"]

    @property
    def aid_version(self) -> int:
        """Incremented when the original depositor updates the record."""
        return self.record["assay"]["descr"]["aid"]["version"]


def substances_to_frame(substances: list[Substance] | Substance, properties: list[str] | None = None) -> pd.DataFrame:
    """Create a :class:`~pandas.DataFrame` from a :class:`~pubchempy.Substance` list.

    Optionally specify a list of the desired :class:`~pubchempy.Substance` properties to
    include as columns in the pandas DataFrame.
    """
    import pandas as pd

    if isinstance(substances, Substance):
        substances = [substances]
    properties = set(properties) | {"sid"} if properties else None
    return pd.DataFrame.from_records([s.to_dict(properties) for s in substances], index="sid")


if __name__ == "__main__":
    print(__version__)
