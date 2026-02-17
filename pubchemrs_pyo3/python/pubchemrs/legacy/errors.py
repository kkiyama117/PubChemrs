from __future__ import annotations

import typing as t

from pubchemrs._pubchemrs import PubChemAPIError as _RustAPIError

if t.TYPE_CHECKING:
    from pubchemrs.legacy import PubChemHTTPError


def _rust_api_error_to_legacy(e: _RustAPIError) -> PubChemHTTPError:
    """Convert a Rust PubChemAPIError to the appropriate legacy HTTP exception."""
    from pubchemrs.legacy import (
        BadRequestError,
        NotFoundError,
        PubChemHTTPError,
        ServerBusyError,
        ServerError,
        TimeoutError,
    )

    msg = str(e)
    _CODE_MAP: dict[str, tuple[int, type[PubChemHTTPError]]] = {
        "BadRequest": (400, BadRequestError),
        "NotFound": (404, NotFoundError),
        "ServerBusy": (503, ServerBusyError),
        "Timeout": (504, TimeoutError),
        "ServerError": (500, ServerError),
    }
    for key, (code, exc_cls) in _CODE_MAP.items():
        if key in msg:
            return exc_cls(code, msg, [])
    return PubChemHTTPError(0, msg, [])
