from __future__ import annotations

import enum
import functools
import logging
import typing as t
import warnings

from pubchemrs._core import _get_default_client as _rust_client
from pubchemrs._pubchemrs import Atom, Bond
from pubchemrs._pubchemrs import Compound
from pubchemrs._pubchemrs import PubChemAPIError as _RustPubChemAPIError
from pubchemrs._pubchemrs import PubChemNotFoundError as _RustNotFoundError
from pubchemrs.legacy.errors import _rust_api_error_to_legacy

if t.TYPE_CHECKING:
    import pandas as pd

log = logging.getLogger("pubchempy")

#: Type alias for URL query parameters.
QueryParam = str | int | float | bool | list[str] | None


class CompoundIdType(enum.IntEnum):
    """Compound record type.

    This is only for pyo3 limitation of not creating `enum.IntEnum.
    """

    #: Original Deposited Compound
    DEPOSITED = 0
    #: Standardized Form of a Deposited Compound
    STANDARDIZED = 1
    #: Component of a Standardized Compound
    COMPONENT = 2
    #: Neutralized Form of a Standardized Compound
    NEUTRALIZED = 3
    #: Substance that is a component of a mixture
    MIXTURE = 4
    #: Predicted Tautomer Form
    TAUTOMER = 5
    #: Predicted Ionized pKa Form
    IONIZED = 6
    #: Unknown Compound Type
    UNKNOWN = 255

    @classmethod
    def from_rust(cls, value) -> CompoundIdType:
        """Convert a Rust CompoundIdType enum to this Python IntEnum."""
        return cls(int(value))


class BondType(enum.IntEnum):
    """Bond Type Information.

    This is only for pyo3 limitation of not creating `enum.IntEnum.
    """

    #: Single Bond
    SINGLE = 1
    #: Double Bond
    DOUBLE = 2
    #: Triple Bond
    TRIPLE = 3
    #: Quadruple Bond
    QUADRUPLE = 4
    #: Dative Bond
    DATIVE = 5
    #: Complex Bond
    COMPLEX = 6
    #: Ionic Bond
    IONIC = 7
    #: Unknown/Unspecified Connectivity
    UNKNOWN = 255

    @classmethod
    def from_rust(cls, value) -> BondType:
        """Convert a Rust BondType enum to this Python IntEnum."""
        return cls(int(value))


class CoordinateType(enum.IntEnum):
    """Coordinate Set Type Distinctions.

    This is only for pyo3 limitation of not creating `enum.IntEnum.
    """

    #: 2D Coordinates
    TWO_D = 1
    #: 3D Coordinates (should also indicate units, below)
    THREE_D = 2
    #: Depositor Provided Coordinates
    SUBMITTED = 3
    #: Experimentally Determined Coordinates
    EXPERIMENTAL = 4
    #: Computed Coordinates
    COMPUTED = 5
    #: Standardized Coordinates
    STANDARDIZED = 6
    #: Hybrid Original with Computed Coordinates (e.g., explicit H)
    AUGMENTED = 7
    #: Template used to align drawing
    ALIGNED = 8
    #: Drawing uses shorthand forms (e.g., COOH, OCH3, Et, etc.)
    COMPACT = 9
    #: (3D) Coordinate units are Angstroms
    UNITS_ANGSTROMS = 10
    #: (3D) Coordinate units are nanometers
    UNITS_NANOMETERS = 11
    #: (2D) Coordinate units are pixels
    UNITS_PIXEL = 12
    #: (2D) Coordinate units are points
    UNITS_POINTS = 13
    #: (2D) Coordinate units are standard bond lengths (1.0)
    UNITS_STDBONDS = 14
    #: Coordinate units are unknown or unspecified
    UNITS_UNKNOWN = 255

    @classmethod
    def from_rust(cls, value) -> CoordinateType:
        """Convert a Rust ResponseCoordinateType enum to this Python IntEnum."""
        return cls(int(value))


#: Dictionary mapping atomic numbers to their element symbols.
#:
#: This dictionary includes 118 standard chemical elements from Hydrogen (1) to
#: Oganesson (118), plus special atom types used by PubChem for non-standard entities
#: like dummy atoms, R-group labels, and lone pairs.
ELEMENTS: dict[int, str] = {
    # Standard chemical elements
    1: "H",  # Hydrogen
    2: "He",  # Helium
    3: "Li",  # Lithium
    4: "Be",  # Beryllium
    5: "B",  # Boron
    6: "C",  # Carbon
    7: "N",  # Nitrogen
    8: "O",  # Oxygen
    9: "F",  # Fluorine
    10: "Ne",  # Neon
    11: "Na",  # Sodium
    12: "Mg",  # Magnesium
    13: "Al",  # Aluminium
    14: "Si",  # Silicon
    15: "P",  # Phosphorus
    16: "S",  # Sulfur
    17: "Cl",  # Chlorine
    18: "Ar",  # Argon
    19: "K",  # Potassium
    20: "Ca",  # Calcium
    21: "Sc",  # Scandium
    22: "Ti",  # Titanium
    23: "V",  # Vanadium
    24: "Cr",  # Chromium
    25: "Mn",  # Manganese
    26: "Fe",  # Iron
    27: "Co",  # Cobalt
    28: "Ni",  # Nickel
    29: "Cu",  # Copper
    30: "Zn",  # Zinc
    31: "Ga",  # Gallium
    32: "Ge",  # Germanium
    33: "As",  # Arsenic
    34: "Se",  # Selenium
    35: "Br",  # Bromine
    36: "Kr",  # Krypton
    37: "Rb",  # Rubidium
    38: "Sr",  # Strontium
    39: "Y",  # Yttrium
    40: "Zr",  # Zirconium
    41: "Nb",  # Niobium
    42: "Mo",  # Molybdenum
    43: "Tc",  # Technetium
    44: "Ru",  # Ruthenium
    45: "Rh",  # Rhodium
    46: "Pd",  # Palladium
    47: "Ag",  # Silver
    48: "Cd",  # Cadmium
    49: "In",  # Indium
    50: "Sn",  # Tin
    51: "Sb",  # Antimony
    52: "Te",  # Tellurium
    53: "I",  # Iodine
    54: "Xe",  # Xenon
    55: "Cs",  # Cesium
    56: "Ba",  # Barium
    57: "La",  # Lanthanum
    58: "Ce",  # Cerium
    59: "Pr",  # Praseodymium
    60: "Nd",  # Neodymium
    61: "Pm",  # Promethium
    62: "Sm",  # Samarium
    63: "Eu",  # Europium
    64: "Gd",  # Gadolinium
    65: "Tb",  # Terbium
    66: "Dy",  # Dysprosium
    67: "Ho",  # Holmium
    68: "Er",  # Erbium
    69: "Tm",  # Thulium
    70: "Yb",  # Ytterbium
    71: "Lu",  # Lutetium
    72: "Hf",  # Hafnium
    73: "Ta",  # Tantalum
    74: "W",  # Tungsten
    75: "Re",  # Rhenium
    76: "Os",  # Osmium
    77: "Ir",  # Iridium
    78: "Pt",  # Platinum
    79: "Au",  # Gold
    80: "Hg",  # Mercury
    81: "Tl",  # Thallium
    82: "Pb",  # Lead
    83: "Bi",  # Bismuth
    84: "Po",  # Polonium
    85: "At",  # Astatine
    86: "Rn",  # Radon
    87: "Fr",  # Francium
    88: "Ra",  # Radium
    89: "Ac",  # Actinium
    90: "Th",  # Thorium
    91: "Pa",  # Protactinium
    92: "U",  # Uranium
    93: "Np",  # Neptunium
    94: "Pu",  # Plutonium
    95: "Am",  # Americium
    96: "Cm",  # Curium
    97: "Bk",  # Berkelium
    98: "Cf",  # Californium
    99: "Es",  # Einsteinium
    100: "Fm",  # Fermium
    101: "Md",  # Mendelevium
    102: "No",  # Nobelium
    103: "Lr",  # Lawrencium
    104: "Rf",  # Rutherfordium
    105: "Db",  # Dubnium
    106: "Sg",  # Seaborgium
    107: "Bh",  # Bohrium
    108: "Hs",  # Hassium
    109: "Mt",  # Meitnerium
    110: "Ds",  # Darmstadtium
    111: "Rg",  # Roentgenium
    112: "Cn",  # Copernicium
    113: "Nh",  # Nihonium
    114: "Fl",  # Flerovium
    115: "Mc",  # Moscovium
    116: "Lv",  # Livermorium
    117: "Ts",  # Tennessine
    118: "Og",  # Oganesson
    # Special atom types
    252: "Lp",  # Lone Pair
    253: "R",  # Rgroup Label
    254: "*",  # Dummy Atom
    255: "*",  # Unspecified Atom (Asterisk)
}


def memoized_property(fget: t.Callable[[t.Any], t.Any]) -> property:
    """Decorator to create memoized properties.

    Used to cache :class:`~pubchempy.Compound` and :class:`~pubchempy.Substance`
    properties that require an additional request.
    """
    attr_name = f"_{fget.__name__}"

    @functools.wraps(fget)
    def fget_memoized(self):
        if not hasattr(self, attr_name):
            setattr(self, attr_name, fget(self))
        return getattr(self, attr_name)

    return property(fget_memoized)


def deprecated(message: str) -> t.Callable[[t.Callable], t.Callable]:
    """Decorator to mark as deprecated and emit a warning when used."""

    def deco(func):
        @functools.wraps(func)
        def wrapped(*args, **kwargs):
            from pubchemrs.legacy import PubChemPyDeprecationWarning

            warnings.warn(
                f"{func.__name__} is deprecated: {message}",
                category=PubChemPyDeprecationWarning,
                stacklevel=2,
            )
            return func(*args, **kwargs)

        return wrapped

    return deco


def _get_compounds_via_rust(
    identifier: str | int | list[str | int],
    namespace: str,
    **kwargs: QueryParam,
) -> list[Compound]:
    """Fetch compound records using the Rust backend and return Rust Compound objects."""
    client = _rust_client()
    str_kwargs = {k: str(v) for k, v in kwargs.items() if v is not None}
    try:
        return client.get_compounds_sync(identifier, namespace, **str_kwargs)
    except _RustNotFoundError:
        return []
    except _RustPubChemAPIError as e:
        raise _rust_api_error_to_legacy(e) from e


def get_compounds(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    searchtype: str | None = None,
    as_dataframe: bool = False,
    **kwargs: QueryParam,
) -> list[Compound] | pd.DataFrame:
    """Retrieve the specified compound records from PubChem.

    Args:
        identifier: The compound identifier to use as a search query.
        namespace: The identifier type, one of cid, name, smiles, sdf, inchi,
            inchikey or formula.
        searchtype: The advanced search type, one of substructure,
            superstructure or similarity.
        as_dataframe: Automatically extract the Compound properties into a pandas
            DataFrame and return that.
        **kwargs: Additional query parameters to pass to the API request.

    Returns:
        List of Compound objects, or a pandas DataFrame if ``as_dataframe=True``.
    """
    if searchtype is not None or namespace == "formula":
        # Searchtype and formula require listkey polling; fall back to legacy HTTP
        from pubchemrs.legacy import get_json

        results = get_json(identifier, namespace, searchtype=searchtype, **kwargs)
        if not results:
            compounds = []
        else:
            # Convert raw dict records to Rust Compound via from_cid when possible
            compounds = []
            for r in results["PC_Compounds"]:
                try:
                    cid = r["id"]["id"]["cid"]
                    compounds.append(Compound.from_cid(cid))
                except (KeyError, Exception):
                    log.warning("Could not convert legacy record to Rust Compound")
    else:
        compounds = _get_compounds_via_rust(identifier, namespace)
    if as_dataframe:
        return compounds_to_frame(compounds)
    return compounds


def compounds_to_frame(compounds: list[Compound] | Compound, properties: list[str] | None = None) -> pd.DataFrame:
    """Create a :class:`~pandas.DataFrame` from a :class:`~pubchempy.Compound` list.

    Optionally specify the desired :class:`~pubchempy.Compound` properties to include as
    columns in the pandas DataFrame.
    """
    import pandas as pd

    if not isinstance(compounds, list):
        compounds = [compounds]
    if properties:
        properties = list(set(properties) | {"cid"})
    return pd.DataFrame.from_records([c.to_dict(properties) for c in compounds], index="cid")
