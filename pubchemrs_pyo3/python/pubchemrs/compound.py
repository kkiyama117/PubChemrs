"""Pythonic Compound class wrapping Rust types."""

from __future__ import annotations

from pubchemrs._core import _get_default_client


class Compound:
    """A PubChem compound with convenient property access.

    Wraps the Rust ``CompoundProperties`` type with snake_case attributes
    and helper methods.
    """

    def __init__(self, props):
        self._props = props

    @classmethod
    def from_cid(cls, cid: int) -> Compound:
        """Fetch a compound by its CID.

        Args:
            cid: PubChem Compound ID.

        Returns:
            A Compound instance with properties populated.
        """
        client = _get_default_client()
        props_list = client.get_properties_sync(
            cid,
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
                "AtomStereoCount",
                "BondStereoCount",
                "Charge",
            ],
            "cid",
        )
        if not props_list:
            raise ValueError(f"No compound found for CID {cid}")
        return cls(props_list[0])

    @property
    def cid(self) -> int:
        return self._props.cid

    @property
    def molecular_formula(self) -> str | None:
        return self._props.molecular_formula

    @property
    def molecular_weight(self) -> float | None:
        return self._props.molecular_weight

    @property
    def inchikey(self) -> str | None:
        return self._props.inchikey

    @property
    def inchi(self) -> str | None:
        return self._props.inchi

    @property
    def isomeric_smiles(self) -> str | None:
        return self._props.smiles

    @property
    def canonical_smiles(self) -> str | None:
        return self._props.connectivity_smiles

    @property
    def iupac_name(self) -> str | None:
        return self._props.iupac_name

    @property
    def exact_mass(self) -> float | None:
        return self._props.exact_mass

    @property
    def monoisotopic_mass(self) -> float | None:
        return self._props.monoisotopic_mass

    @property
    def tpsa(self) -> float | None:
        return self._props.tpsa

    @property
    def xlogp(self) -> float | None:
        return self._props.xlogp

    @property
    def complexity(self) -> float | None:
        return self._props.complexity

    @property
    def h_bond_donor_count(self) -> int | None:
        return self._props.h_bond_donor_count

    @property
    def h_bond_acceptor_count(self) -> int | None:
        return self._props.h_bond_acceptor_count

    @property
    def rotatable_bond_count(self) -> int | None:
        return self._props.rotatable_bond_count

    @property
    def heavy_atom_count(self) -> int | None:
        return self._props.heavy_atom_count

    @property
    def charge(self) -> int | None:
        return self._props.charge

    def to_dict(self) -> dict:
        """Convert compound properties to a dictionary."""
        return {
            "cid": self.cid,
            "molecular_formula": self.molecular_formula,
            "molecular_weight": self.molecular_weight,
            "inchikey": self.inchikey,
            "inchi": self.inchi,
            "isomeric_smiles": self.isomeric_smiles,
            "canonical_smiles": self.canonical_smiles,
            "iupac_name": self.iupac_name,
            "exact_mass": self.exact_mass,
            "monoisotopic_mass": self.monoisotopic_mass,
            "tpsa": self.tpsa,
            "xlogp": self.xlogp,
            "complexity": self.complexity,
            "h_bond_donor_count": self.h_bond_donor_count,
            "h_bond_acceptor_count": self.h_bond_acceptor_count,
            "rotatable_bond_count": self.rotatable_bond_count,
            "heavy_atom_count": self.heavy_atom_count,
            "charge": self.charge,
        }

    def __repr__(self) -> str:
        name = self.iupac_name or "unknown"
        return f"Compound(cid={self.cid}, name={name!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Compound):
            return NotImplemented
        return self.cid == other.cid
