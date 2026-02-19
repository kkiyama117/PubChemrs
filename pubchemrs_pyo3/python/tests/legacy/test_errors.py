"""Test errors."""

import pytest

from pubchemrs.legacy import (
    BadRequestError,
    Compound,
    NotFoundError,
    Substance,
    get_compounds,
    get_substances,
)
from pubchemrs import PubChemNotFoundError


def test_invalid_identifier():
    """BadRequestError or TypeError should be raised if identifier is not valid."""
    # Rust Compound.from_cid takes u32, so passing a string raises TypeError
    with pytest.raises((BadRequestError, TypeError)):
        Compound.from_cid("aergaerhg")
    with pytest.raises(BadRequestError):
        get_compounds("srthrthsr")
    with pytest.raises(BadRequestError):
        get_substances("grgrqjksa")


def test_notfound_identifier():
    """NotFoundError should be raised if the record doesn't exist."""
    # Rust Compound.from_cid raises PubChemNotFoundError
    with pytest.raises((NotFoundError, PubChemNotFoundError)):
        Compound.from_cid(999999999)
    with pytest.raises(NotFoundError):
        Substance.from_sid(999999999)


def test_notfound_search():
    """No error should be raised if a search returns no results."""
    get_compounds(999999999)
    get_substances(999999999)
