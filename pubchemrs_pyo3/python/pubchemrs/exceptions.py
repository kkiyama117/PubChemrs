"""Exception hierarchy for the pubchemrs package."""

from pubchemrs._pubchemrs import PubChemError, PubChemAPIError, PubChemNotFoundError

__all__ = [
    "PubChemError",
    "PubChemAPIError",
    "PubChemNotFoundError",
]
