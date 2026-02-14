"""Test that all public API imports work correctly."""


def test_import_pubchemrs():
    import pubchemrs

    assert pubchemrs.__version__ == "0.1.0"


def test_import_client():
    from pubchemrs import PubChemClient

    client = PubChemClient()
    assert client is not None


def test_import_compound_class():
    from pubchemrs import Compound

    assert Compound is not None


def test_import_compound_properties():
    from pubchemrs import CompoundProperties

    assert CompoundProperties is not None


def test_import_exceptions():
    from pubchemrs import PubChemError, PubChemAPIError, PubChemNotFoundError

    assert issubclass(PubChemAPIError, PubChemError)
    assert issubclass(PubChemNotFoundError, PubChemAPIError)


def test_import_legacy():
    from pubchemrs.legacy import (
        get_compounds,
        get_properties,
        get_synonyms,
        get_all_sources,
    )

    assert callable(get_compounds)
    assert callable(get_properties)
    assert callable(get_synonyms)
    assert callable(get_all_sources)


def test_import_convenience_functions():
    from pubchemrs import (
        get_compounds,
        get_properties,
        get_synonyms,
        get_all_sources,
    )

    assert callable(get_compounds)
    assert callable(get_properties)
    assert callable(get_synonyms)
    assert callable(get_all_sources)


def test_client_with_custom_config():
    from pubchemrs import PubChemClient

    client = PubChemClient(timeout_secs=60, max_retries=5)
    assert client is not None
