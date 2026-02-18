"""Low-level bridge to the Rust extension module."""

from pubchemrs._pubchemrs import PubChemClient as _RustClient

_default_client = None


def _get_default_client():
    """Get or create the module-level default client."""
    global _default_client
    if _default_client is None:
        _default_client = _RustClient()
    return _default_client
