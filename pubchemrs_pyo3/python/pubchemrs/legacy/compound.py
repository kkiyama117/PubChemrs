from __future__ import annotations

import enum
import functools
import logging
import typing as t
import warnings
from itertools import zip_longest

from pubchemrs._core import _get_default_client as _rust_client
from pubchemrs._pubchemrs import Atom, Bond
from pubchemrs._pubchemrs import PubChemAPIError as _RustPubChemAPIError
from pubchemrs._pubchemrs import PubChemNotFoundError as _RustNotFoundError
from pubchemrs.legacy.errors import _rust_api_error_to_legacy

if t.TYPE_CHECKING:
    import pandas as pd

log = logging.getLogger("pubchempy")

#: Type alias for URL query parameters.
QueryParam = str | int | float | bool | list[str] | None


class CompoundIdType(enum.IntEnum):
    """Compound record type."""

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
    """Bond Type Information."""

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
    """Coordinate Set Type Distinctions."""

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
) -> list:
    """Fetch compound records using the Rust backend and convert to legacy Compound objects."""
    client = _rust_client()
    try:
        rust_compounds = client.get_compounds_sync(identifier, namespace)
    except _RustNotFoundError:
        return []
    except _RustPubChemAPIError as e:
        raise _rust_api_error_to_legacy(e) from e
    return [Compound(c.to_dict()) for c in rust_compounds]


class Compound:
    """Represents a standardized chemical structure record from PubChem.

    The PubChem Compound database contains standardized and deduplicated chemical
    structures derived from the Substance database. Each Compound is uniquely identified
    by a CID (Compound Identifier) and represents a unique chemical structure with
    calculated properties, descriptors, and associated experimental data.

    Examples:
        >>> compound = Compound.from_cid(2244)  # Aspirin
        >>> print(f"Formula: {compound.molecular_formula}")
        Formula: C9H8O4
        >>> print(f"IUPAC: {compound.iupac_name}")
        IUPAC: 2-acetyloxybenzoic acid
        >>> print(f"MW: {compound.molecular_weight}")
        MW: 180.16
    """

    def __init__(self, record: dict[str, t.Any]) -> None:
        """Initialize a Compound with a record dict from the PubChem PUG REST service.

        Args:
            record: Compound record returned by the PubChem PUG REST service.

        Note:
            Most users will not need to instantiate a Compound instance directly from a
            record. The :meth:`from_cid()` class method and the :func:`~get_compounds()`
            function offer more convenient ways to obtain Compound instances, as they
            also handle the retrieval of the record from PubChem.
        """
        self._record = None
        self._atoms = {}
        self._bonds = {}
        self.record = record

    def _setup_atoms(self) -> None:
        """Derive Atom objects from the record."""
        from pubchemrs.legacy import ResponseParseError

        # Delete existing atoms
        self._atoms = {}
        # Create atoms
        aids = self.record["atoms"]["aid"]
        elements = self.record["atoms"]["element"]
        if not len(aids) == len(elements):
            raise ResponseParseError("Error parsing atom elements")
        for aid, element in zip(aids, elements):
            self._atoms[aid] = Atom(aid=aid, number=element)
        # Add coordinates
        if "coords" in self.record:
            coord_ids = self.record["coords"][0]["aid"]
            xs = self.record["coords"][0]["conformers"][0]["x"]
            ys = self.record["coords"][0]["conformers"][0]["y"]
            zs = self.record["coords"][0]["conformers"][0].get("z", [])
            if not len(coord_ids) == len(xs) == len(ys) == len(self._atoms) or (zs and not len(zs) == len(coord_ids)):
                raise ResponseParseError("Error parsing atom coordinates")
            for aid, x, y, z in zip_longest(coord_ids, xs, ys, zs):
                self._atoms[aid].set_coordinates(x, y, z)
        # Add charges
        if "charge" in self.record["atoms"]:
            for charge in self.record["atoms"]["charge"]:
                self._atoms[charge["aid"]].charge = charge["value"]

    def _setup_bonds(self) -> None:
        """Derive Bond objects from the record."""
        from pubchemrs.legacy import ResponseParseError

        self._bonds = {}
        if "bonds" not in self.record:
            return
        # Create bonds
        aid1s = self.record["bonds"]["aid1"]
        aid2s = self.record["bonds"]["aid2"]
        orders = self.record["bonds"]["order"]
        if not len(aid1s) == len(aid2s) == len(orders):
            raise ResponseParseError("Error parsing bonds")
        for aid1, aid2, order in zip(aid1s, aid2s, orders):
            self._bonds[frozenset((aid1, aid2))] = Bond(aid1=aid1, aid2=aid2, order=order)
        # Add styles
        if "coords" in self.record and "style" in self.record["coords"][0]["conformers"][0]:
            aid1s = self.record["coords"][0]["conformers"][0]["style"]["aid1"]
            aid2s = self.record["coords"][0]["conformers"][0]["style"]["aid2"]
            styles = self.record["coords"][0]["conformers"][0]["style"]["annotation"]
            for aid1, aid2, style in zip(aid1s, aid2s, styles):
                self._bonds[frozenset((aid1, aid2))].style = style

    @classmethod
    def from_cid(cls, cid: int, **kwargs: QueryParam) -> Compound:
        """Retrieve the Compound record for the specified CID.

        Args:
            cid: The PubChem Compound Identifier (CID) to retrieve.
            **kwargs: Additional parameters to pass to the request.

        Example:
            c = Compound.from_cid(6819)
        """
        from pubchemrs.legacy import NotFoundError

        compounds = _get_compounds_via_rust(cid, "cid")
        if not compounds:
            raise NotFoundError(404, "Not Found", [f"No compound found for CID {cid}"])
        return compounds[0]

    @property
    def record(self) -> dict[str, t.Any]:
        """The full compound record returned by the PubChem PUG REST service."""
        return self._record

    @record.setter
    def record(self, record: dict[str, t.Any]) -> None:
        self._record = record
        log.debug(f"Created {self}")
        self._setup_atoms()
        self._setup_bonds()

    def __repr__(self) -> str:
        return f"Compound({self.cid if self.cid else ''})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, type(self)) and self.record == other.record

    def to_dict(self, properties: list[str] | None = None) -> dict[str, t.Any]:
        """Return a dict containing Compound property data.

        Optionally specify a list of the desired properties to include. If
        ``properties`` is not specified, all properties are included, with the following
        exceptions:

        :attr:`synonyms`, :attr:`aids` and :attr:`sids` are not included unless
        explicitly specified. This is because they each require an extra request to the
        PubChem API to retrieve.

        :attr:`canonical_smiles` and :attr:`isomeric_smiles` are not included by
        default, as they are deprecated and have been replaced by
        :attr:`connectivity_smiles` and :attr:`smiles` respectively.

        Args:
            properties: List of desired properties.

        Returns:
            Dictionary of compound data.
        """
        if not properties:
            skip = {
                "record",
                "aids",
                "sids",
                "synonyms",
                "canonical_smiles",
                "isomeric_smiles",
            }
            properties = [p for p, v in Compound.__dict__.items() if isinstance(v, property) and p not in skip]
        return {
            p: [i.to_dict() for i in getattr(self, p)] if p in {"atoms", "bonds"} else getattr(self, p)
            for p in properties
        }

    def to_series(self, properties: list[str] | None = None) -> pd.Series:
        """Return a pandas :class:`~pandas.Series` containing Compound data.

        Optionally specify a list of the desired properties to include as columns. If
        ``properties`` is not specified, all properties are included, with the following
        exceptions:

        :attr:`synonyms`, :attr:`aids` and :attr:`sids` are not included unless
        explicitly specified. This is because they each require an extra request to the
        PubChem API to retrieve.

        :attr:`canonical_smiles` and :attr:`isomeric_smiles` are not included by
        default, as they are deprecated and have been replaced by
        :attr:`connectivity_smiles` and :attr:`smiles` respectively.

        Args:
            properties: List of desired properties.
        """
        import pandas as pd

        return pd.Series(self.to_dict(properties))

    @property
    def cid(self) -> int | None:
        """The PubChem Compound Identifier (CID).

        .. note::

            When searching using a SMILES or InChI query that is not present in the
            PubChem Compound database, an automatically generated record may be returned
            that contains properties that have been calculated on the fly. These records
            will not have a CID property.
        """
        try:
            return self.record["id"]["id"]["cid"]
        except KeyError:
            return None

    @property
    def elements(self) -> list[str]:
        """List of element symbols for atoms in this Compound."""
        return [a.element for a in self.atoms]

    @property
    def atoms(self) -> list[Atom]:
        """List of :class:`Atoms <pubchempy.Atom>` in this Compound."""
        return sorted(self._atoms.values(), key=lambda x: x.aid)

    @property
    def bonds(self) -> list[Bond]:
        """List of :class:`Bonds <pubchempy.Bond>` in this Compound."""
        return sorted(self._bonds.values(), key=lambda x: (x.aid1, x.aid2))

    @memoized_property
    def synonyms(self) -> list[str] | None:
        """Ranked list of all the names associated with this Compound.

        Requires an extra request. Result is cached.
        """
        if self.cid:
            from pubchemrs.legacy import get_json

            results = get_json(self.cid, operation="synonyms")
            return results["InformationList"]["Information"][0]["Synonym"] if results else []

    @memoized_property
    def sids(self) -> list[int] | None:
        """List of Substance Identifiers associated with this Compound.

        Requires an extra request. Result is cached.
        """
        if self.cid:
            from pubchemrs.legacy import get_json

            results = get_json(self.cid, operation="sids")
            return results["InformationList"]["Information"][0]["SID"] if results else []

    @memoized_property
    def aids(self) -> list[int] | None:
        """List of Assay Identifiers associated with this Compound.

        Requires an extra request. Result is cached.
        """
        if self.cid:
            from pubchemrs.legacy import get_json

            results = get_json(self.cid, operation="aids")
            return results["InformationList"]["Information"][0]["AID"] if results else []

    @property
    def coordinate_type(self) -> str | None:
        """Whether this Compound has 2D or 3D coordinates."""
        if CoordinateType.TWO_D in self.record["coords"][0]["type"]:
            return "2d"
        elif CoordinateType.THREE_D in self.record["coords"][0]["type"]:
            return "3d"

    @property
    def charge(self) -> int:
        """Formal charge on this Compound."""
        return self.record["charge"] if "charge" in self.record else 0

    @property
    def molecular_formula(self) -> str | None:
        """Molecular formula.

        The molecular formula represents the number of atoms of each element in a
        compound. It does not contain any information about connectivity or structure.
        """
        return _parse_prop({"label": "Molecular Formula"}, self.record["props"])

    @property
    def molecular_weight(self) -> float | None:
        """Molecular weight in g/mol.

        The molecular weight is the sum of all atomic weights of the constituent
        atoms in a compound, measured in g/mol. In the absence of explicit isotope
        labelling, averaged natural abundance is assumed. If an atom bears an
        explicit isotope label, 100% isotopic purity is assumed at this location.
        """
        sval = _parse_prop({"label": "Molecular Weight"}, self.record["props"])
        return float(sval) if sval else None

    @property
    @deprecated("Use connectivity_smiles instead")
    def canonical_smiles(self) -> str | None:
        """Canonical SMILES, with no stereochemistry information (deprecated).

        .. deprecated:: 1.0.5
           :attr:`canonical_smiles` is deprecated, use :attr:`connectivity_smiles`
           instead.
        """
        return self.connectivity_smiles

    @property
    @deprecated("Use smiles instead")
    def isomeric_smiles(self) -> str | None:
        """Isomeric SMILES.

        .. deprecated:: 1.0.5
           :attr:`isomeric_smiles` is deprecated, use :attr:`smiles` instead.
        """
        return self.smiles

    @property
    def connectivity_smiles(self) -> str | None:
        """Connectivity SMILES string.

        A canonical SMILES string that includes connectivity information only. It
        excludes stereochemical and isotopic information.

        Replaces the deprecated :attr:`canonical_smiles` property.
        """
        return _parse_prop({"label": "SMILES", "name": "Connectivity"}, self.record["props"])

    @property
    def smiles(self) -> str | None:
        """Absolute SMILES string (isomeric and canonical).

        A canonical SMILES string that includes both stereochemical and isotopic
        information. This provides the most complete linear representation of the
        molecular structure.

        Replaces the deprecated :attr:`isomeric_smiles` property.
        """
        return _parse_prop({"label": "SMILES", "name": "Absolute"}, self.record["props"])

    @property
    def inchi(self) -> str | None:
        """Standard IUPAC International Chemical Identifier (InChI).

        The InChI provides a unique, standardized representation of molecular
        structure that is not dependent on the software used to generate it.
        It includes connectivity, stereochemistry, and isotopic information
        in a layered format. This standard version does not allow for user
        selectable options in dealing with stereochemistry and tautomer layers.
        """
        return _parse_prop({"label": "InChI", "name": "Standard"}, self.record["props"])

    @property
    def inchikey(self) -> str | None:
        """Standard InChIKey.

        A hashed version of the full standard InChI, consisting of 27 characters
        divided into three blocks separated by hyphens. The InChIKey provides a
        fixed-length identifier that is more suitable for database indexing and
        web searches than the full InChI string.
        """
        return _parse_prop({"label": "InChIKey", "name": "Standard"}, self.record["props"])

    @property
    def iupac_name(self) -> str | None:
        """Preferred IUPAC name.

        The chemical name systematically determined according to IUPAC
        (International Union of Pure and Applied Chemistry) nomenclature rules.
        This is the preferred systematic name among the available IUPAC naming
        styles (Allowed, CAS-like Style, Preferred, Systematic, Traditional).
        """
        # Note: record has Allowed, CAS-like Style, Preferred, Systematic, Traditional
        return _parse_prop({"label": "IUPAC Name", "name": "Preferred"}, self.record["props"])

    @property
    def xlogp(self) -> float | None:
        """XLogP octanol-water partition coefficient.

        A computationally generated octanol-water partition coefficient that
        measures the hydrophilicity or hydrophobicity of a molecule. Higher
        values indicate more lipophilic (fat-soluble) compounds, while lower
        values indicate more hydrophilic (water-soluble) compounds.
        """
        return _parse_prop({"label": "Log P"}, self.record["props"])

    @property
    def exact_mass(self) -> float | None:
        """Exact mass in Da (Daltons).

        The mass of the most likely isotopic composition for a single molecule,
        corresponding to the most intense ion/molecule peak in a mass spectrum.
        This differs from molecular weight in that it uses the exact masses of
        specific isotopes rather than averaged atomic weights.
        """
        sval = _parse_prop({"label": "Mass", "name": "Exact"}, self.record["props"])
        return float(sval) if sval else None

    @property
    def monoisotopic_mass(self) -> float | None:
        """Monoisotopic mass in Da (Daltons).

        The mass of a molecule calculated using the mass of the most abundant
        isotope of each element. This provides a single, well-defined mass value
        useful for high-resolution mass spectrometry applications.
        """
        sval = _parse_prop({"label": "Weight", "name": "MonoIsotopic"}, self.record["props"])
        return float(sval) if sval else None

    @property
    def tpsa(self) -> float | None:
        """Topological Polar Surface Area (TPSA).

        The topological polar surface area computed using the algorithm described
        by Ertl et al. TPSA is a commonly used descriptor for predicting drug
        absorption, as it correlates well with passive molecular transport through
        membranes. Values are typically expressed in square Ångströms.
        """
        return _parse_prop({"implementation": "E_TPSA"}, self.record["props"])

    @property
    def complexity(self) -> float | None:
        """Molecular complexity rating.

        A measure of molecular complexity computed using the Bertz/Hendrickson/
        Ihlenfeldt formula. This descriptor quantifies the structural complexity
        of a molecule based on factors such as the number of atoms, bonds,
        rings, and branching patterns.
        """
        return _parse_prop({"implementation": "E_COMPLEXITY"}, self.record["props"])

    @property
    def h_bond_donor_count(self) -> int | None:
        """Number of hydrogen-bond donors in the structure.

        Counts functional groups that can donate hydrogen bonds, such as
        -OH, -NH, and -SH groups. This descriptor is important for predicting
        drug-like properties and molecular interactions.
        """
        return _parse_prop({"implementation": "E_NHDONORS"}, self.record["props"])

    @property
    def h_bond_acceptor_count(self) -> int | None:
        """Number of hydrogen-bond acceptors in the structure.

        Counts functional groups that can accept hydrogen bonds, such as
        oxygen and nitrogen atoms with lone pairs. This descriptor is important
        for predicting drug-like properties and molecular interactions.
        """
        return _parse_prop({"implementation": "E_NHACCEPTORS"}, self.record["props"])

    @property
    def rotatable_bond_count(self) -> int | None:
        """Number of rotatable bonds.

        Counts single bonds that can freely rotate, excluding bonds in rings
        and terminal bonds to hydrogen or methyl groups.
        """
        return _parse_prop({"implementation": "E_NROTBONDS"}, self.record["props"])

    @property
    def fingerprint(self) -> str | None:
        """Raw padded and hex-encoded structural fingerprint from PubChem.

        Returns the raw padded and hex-encoded fingerprint as returned by the PUG REST
        API. This is the underlying data used to generate the human-readable binary
        fingerprint via the ``cactvs_fingerprint`` property. Most users should use
        ``cactvs_fingerprint`` instead for substructure analysis and similarity
        calculations.

        The PubChem fingerprint data is 881 bits in length. Binary data is stored in one
        byte increments. This fingerprint is, therefore, 111 bytes in length (888 bits),
        which includes padding of seven bits at the end to complete the last byte. A
        four-byte prefix, containing the bit length of the fingerprint (881 bits),
        increases the stored PubChem fingerprint size to 115 bytes (920 bits). This is
        then hex-encoded, resulting in a 230-character string.

        More information at:
        ftp://ftp.ncbi.nlm.nih.gov/pubchem/specifications/pubchem_fingerprints.txt
        """
        return _parse_prop({"implementation": "E_SCREEN"}, self.record["props"])

    @property
    def cactvs_fingerprint(self) -> str | None:
        """PubChem CACTVS structural fingerprint as 881-bit binary string.

        Returns a binary fingerprint string where each character is a bit representing
        the presence (1) or absence (0) of specific chemical substructures and features.
        The 881-bit fingerprint is organized into sections covering:

        - Section 1: Hierarchical element counts (1-115)
        - Section 2: Rings in a canonical ring set (116-163)
        - Section 3: Simple atom pairs (164-218)
        - Section 4: Simple atom nearest neighbors (219-242)
        - Section 5: Detailed atom neighborhoods (243-707)
        - Section 6: Simple SMARTS patterns (708-881)

        This fingerprint enables efficient substructure searching, similarity
        calculations, and chemical clustering.

        More information at:
        ftp://ftp.ncbi.nlm.nih.gov/pubchem/specifications/pubchem_fingerprints.txt
        """
        # Skip first 4 bytes (contain length of fingerprint) and last 7 bits (padding)
        # then re-pad to 881 bits
        return f"{int(self.fingerprint[8:], 16):020b}"[:-7].zfill(881)

    @property
    def heavy_atom_count(self) -> int | None:
        """Number of heavy atoms (non-hydrogen atoms).

        Counts all atoms in the molecule except hydrogen. This is a basic descriptor of
        molecular size and is used in various chemical calculations and molecular
        property predictions.
        """
        if "count" in self.record and "heavy_atom" in self.record["count"]:
            return self.record["count"]["heavy_atom"]

    @property
    def isotope_atom_count(self) -> int | None:
        """Number of atoms with enriched isotopes.

        Counts atoms that are specified with non-standard isotopes (e.g., ²H, ¹³C). Most
        organic molecules have a value of 0 unless they are isotopically labeled for
        research or analytical purposes.
        """
        if "count" in self.record and "isotope_atom" in self.record["count"]:
            return self.record["count"]["isotope_atom"]

    @property
    def atom_stereo_count(self) -> int | None:
        """Total number of atoms with tetrahedral (sp³) stereochemistry.

        Counts atoms that have tetrahedral stereochemistry. This includes both defined
        and undefined stereocenters in the molecule.
        """
        if "count" in self.record and "atom_chiral" in self.record["count"]:
            return self.record["count"]["atom_chiral"]

    @property
    def defined_atom_stereo_count(self) -> int | None:
        """Number of atoms with defined tetrahedral (sp³) stereochemistry.

        Counts stereocenters where the absolute configuration is explicitly specified
        (e.g. R or S). This excludes stereocenters where the  configuration is unknown
        or unspecified.
        """
        if "count" in self.record and "atom_chiral_def" in self.record["count"]:
            return self.record["count"]["atom_chiral_def"]

    @property
    def undefined_atom_stereo_count(self) -> int | None:
        """Number of atoms with undefined tetrahedral (sp³) stereochemistry.

        Counts stereocenters where the absolute configuration is not specified or is
        unknown. These represent potential stereocenters that could have either R or S
        configuration, but this is not explicitly defined.
        """
        if "count" in self.record and "atom_chiral_undef" in self.record["count"]:
            return self.record["count"]["atom_chiral_undef"]

    @property
    def bond_stereo_count(self) -> int | None:
        """Bond stereocenter count."""
        if "count" in self.record and "bond_chiral" in self.record["count"]:
            return self.record["count"]["bond_chiral"]

    @property
    def defined_bond_stereo_count(self) -> int | None:
        """Defined bond stereocenter count."""
        if "count" in self.record and "bond_chiral_def" in self.record["count"]:
            return self.record["count"]["bond_chiral_def"]

    @property
    def undefined_bond_stereo_count(self) -> int | None:
        """Undefined bond stereocenter count."""
        if "count" in self.record and "bond_chiral_undef" in self.record["count"]:
            return self.record["count"]["bond_chiral_undef"]

    @property
    def covalent_unit_count(self) -> int | None:
        """Covalently-bonded unit count."""
        if "count" in self.record and "covalent_unit" in self.record["count"]:
            return self.record["count"]["covalent_unit"]

    @property
    def volume_3d(self) -> float | None:
        """Analytic volume of the first diverse conformer.

        The 3D molecular volume calculated for the default (first diverse) conformer.
        This descriptor provides information about the space occupied by the molecule in
        three dimensions.
        """
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Shape", "name": "Volume"}, conf["data"])

    @property
    def multipoles_3d(self) -> list[float] | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Shape", "name": "Multipoles"}, conf["data"])

    @property
    def conformer_rmsd_3d(self) -> float | None:
        """Conformer sampling RMSD in Å.

        The root-mean-square deviation of atomic positions between different conformers
        in the conformer model. This measures the structural diversity of the generated
        conformer ensemble.
        """
        coords = self.record["coords"][0]
        if "data" in coords:
            return _parse_prop({"label": "Conformer", "name": "RMSD"}, coords["data"])

    @property
    def effective_rotor_count_3d(self) -> int | None:
        """Number of effective rotors in the 3D structure.

        A count of rotatable bonds that significantly contribute to conformational
        flexibility. This is often less than the total rotatable bond count as it
        excludes rotors that have restricted rotation due to steric or electronic
        effects.
        """
        return _parse_prop({"label": "Count", "name": "Effective Rotor"}, self.record["props"])

    @property
    def pharmacophore_features_3d(self) -> list[str] | None:
        """3D pharmacophore features present in the molecule.

        A list of pharmacophore feature types identified in the 3D structure, such as
        hydrogen bond donors, acceptors, aromatic rings, and hydrophobic regions. These
        features are important for drug-target interactions.
        """
        return _parse_prop({"label": "Features", "name": "Pharmacophore"}, self.record["props"])

    @property
    def mmff94_partial_charges_3d(self) -> list[str] | None:
        return _parse_prop({"label": "Charge", "name": "MMFF94 Partial"}, self.record["props"])

    @property
    def mmff94_energy_3d(self) -> float | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Energy", "name": "MMFF94 NoEstat"}, conf["data"])

    @property
    def conformer_id_3d(self) -> str | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Conformer", "name": "ID"}, conf["data"])

    @property
    def shape_selfoverlap_3d(self) -> float | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Shape", "name": "Self Overlap"}, conf["data"])

    @property
    def feature_selfoverlap_3d(self) -> float | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Feature", "name": "Self Overlap"}, conf["data"])

    @property
    def shape_fingerprint_3d(self) -> list[str] | None:
        conf = self.record["coords"][0]["conformers"][0]
        if "data" in conf:
            return _parse_prop({"label": "Fingerprint", "name": "Shape"}, conf["data"])


def _parse_prop(search: dict[str, str], proplist: list[dict[str, t.Any]]) -> t.Any:
    """Extract property value from record using the given urn search filter."""
    props = [i for i in proplist if all(item in i["urn"].items() for item in search.items())]
    if len(props) > 0:
        return props[0]["value"][list(props[0]["value"].keys())[0]]


def get_compounds(
    identifier: str | int | list[str | int],
    namespace: str = "cid",
    searchtype: str | None = None,
    as_dataframe: bool = False,
    **kwargs: QueryParam,
) -> list | pd.DataFrame:
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
    from pubchemrs.legacy import get_json

    if searchtype is not None:
        # Searchtype requires listkey polling; fall back to legacy HTTP
        results = get_json(identifier, namespace, searchtype=searchtype, **kwargs)
        compounds = [Compound(r) for r in results["PC_Compounds"]] if results else []
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

    if isinstance(compounds, Compound):
        compounds = [compounds]
    properties = set(properties) | {"cid"} if properties else None
    return pd.DataFrame.from_records([c.to_dict(properties) for c in compounds], index="cid")
