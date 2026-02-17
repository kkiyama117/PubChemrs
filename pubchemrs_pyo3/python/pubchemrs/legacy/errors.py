from __future__ import annotations

import json
import typing as t
from urllib.error import HTTPError

from pubchemrs._pubchemrs import PubChemAPIError as _RustAPIError


class PubChemPyDeprecationWarning(DeprecationWarning):
    """Warning category for deprecated features."""


class PubChemPyError(Exception):
    """Base class for all PubChemPy exceptions."""


class ResponseParseError(PubChemPyError):
    """PubChem response is uninterpretable."""


class PubChemHTTPError(PubChemPyError):
    """Generic error class to handle HTTP error codes."""

    def __init__(self, code: int, msg: str, details: list[str]) -> None:
        """Initialize with HTTP status code, message, and additional details.

        Args:
            code: HTTP status code.
            msg: Error message.
            details: Additional error details from PubChem API.
        """
        super().__init__(msg)
        self.code = code
        self.msg = msg
        self.details = details

    def __str__(self) -> str:
        output = f"PubChem HTTP Error {self.code} {self.msg}"
        if self.details:
            details = ", ".join(self.details)
            output = f"{output} ({details})"
        return output

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.code!r}, {self.msg!r}, {self.details!r})"


class BadRequestError(PubChemHTTPError):
    """400: Request is improperly formed (e.g. syntax error in the URL or POST body)."""


class NotFoundError(PubChemHTTPError):
    """404: The input record was not found (e.g. invalid CID)."""


class MethodNotAllowedError(PubChemHTTPError):
    """405: Request not allowed (e.g. invalid MIME type in the HTTP Accept header)."""


class ServerError(PubChemHTTPError):
    """500: Some problem on the server side (e.g. a database server down)."""


class UnimplementedError(PubChemHTTPError):
    """501: The requested operation has not (yet) been implemented by the server."""


class ServerBusyError(PubChemHTTPError):
    """503: Too many requests or server is busy, retry later."""


class TimeoutError(PubChemHTTPError):
    """504: The request timed out, from server overload or too broad a request.

    See :ref:`Avoiding TimeoutError <avoiding_timeouterror>` for more information.
    """


def create_http_error(e: HTTPError) -> PubChemHTTPError:
    """Create appropriate PubChem HTTP error subclass based on status code."""
    code = e.code
    msg = e.msg
    details: list[str] = []
    try:
        fault = json.loads(e.read().decode())["Fault"]
        msg = fault.get("Code", msg)
        if "Message" in fault:
            msg = f"{msg}: {fault['Message']}"
        details = fault.get("Details", [])
    except (ValueError, IndexError, KeyError):
        pass

    error_map: dict[int, type[PubChemHTTPError]] = {
        400: BadRequestError,
        404: NotFoundError,
        405: MethodNotAllowedError,
        500: ServerError,
        501: UnimplementedError,
        503: ServerBusyError,
        504: TimeoutError,
    }
    error_class = error_map.get(code, PubChemHTTPError)
    return error_class(code, msg, details)


def _rust_api_error_to_legacy(e: _RustAPIError) -> PubChemHTTPError:
    """Convert a Rust PubChemAPIError to the appropriate legacy HTTP exception."""
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
