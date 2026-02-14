use pyo3::exceptions::{PyConnectionError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::{PyErr, create_exception};

use pubchemrs_tokio::error::Error;

// Custom Python exceptions
create_exception!(pubchemrs, PubChemError, pyo3::exceptions::PyException);
create_exception!(pubchemrs, PubChemAPIError, PubChemError);
create_exception!(pubchemrs, PubChemNotFoundError, PubChemAPIError);

pub fn register_exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("PubChemError", m.py().get_type::<PubChemError>())?;
    m.add("PubChemAPIError", m.py().get_type::<PubChemAPIError>())?;
    m.add("PubChemNotFoundError", m.py().get_type::<PubChemNotFoundError>())?;
    Ok(())
}

pub fn to_pyerr(err: Error) -> PyErr {
    match err {
        Error::ApiFault { ref code, ref message } => {
            if code.contains("NotFound") {
                PubChemNotFoundError::new_err(message.clone())
            } else {
                PubChemAPIError::new_err(format!("{code}: {message}"))
            }
        }
        Error::Http(e) => PyConnectionError::new_err(e.to_string()),
        Error::HttpStatus { status, ref body } => {
            if status == 404 {
                PubChemNotFoundError::new_err(body.clone())
            } else {
                PubChemAPIError::new_err(format!("HTTP {status}: {body}"))
            }
        }
        Error::Json(e) => PyValueError::new_err(e.to_string()),
        Error::PubChem(e) => match e {
            pubchemrs_struct::error::PubChemError::InvalidInput(msg) => {
                PyValueError::new_err(msg)
            }
            other => PyRuntimeError::new_err(other.to_string()),
        },
    }
}
