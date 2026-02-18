use pyo3::types::{PyBool, PyDict, PyDictMethods, PyFloat, PyList, PyNone, PyString};
use pyo3::{Bound, IntoPyObject, PyResult, Python, pymethods};
use serde_json::Value;

use super::Compound;

/// Recursively remove null values from a JSON Value tree (object keys only).
///
/// Null entries inside arrays are preserved since array positions carry meaning.
fn strip_nulls(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, strip_nulls(v)))
                .collect(),
        ),
        Value::Array(arr) => Value::Array(arr.into_iter().map(strip_nulls).collect()),
        other => other,
    }
}

/// Convert a `serde_json::Value` into a Python object.
fn value_to_py<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, pyo3::PyAny>> {
    match value {
        Value::Null => Ok(PyNone::get(py).to_owned().into_any()),
        Value::Bool(b) => Ok(PyBool::new(py, *b).to_owned().into_any()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into_pyobject(py)?.into_any())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err(
                    "unsupported JSON number",
                ))
            }
        }
        Value::String(s) => Ok(PyString::new(py, s).into_pyobject(py)?.into_any()),
        Value::Array(arr) => {
            let items: Vec<Bound<'py, pyo3::PyAny>> = arr
                .iter()
                .map(|v| value_to_py(py, v))
                .collect::<PyResult<_>>()?;
            Ok(PyList::new(py, items)?.into_pyobject(py)?.into_any())
        }
        Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, value_to_py(py, v)?)?;
            }
            Ok(dict.into_pyobject(py)?.into_any())
        }
    }
}

#[pymethods]
impl Compound {
    /// Convert to a Python dict matching the PubChem API JSON format.
    ///
    /// `Option::None` fields are omitted (not serialized as `null`)
    /// to match the original PubChem API behavior where missing keys
    /// are simply absent.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let cleaned = strip_nulls(value);

        // The top-level value is always an object after serde serialization of a struct.
        match &cleaned {
            Value::Object(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, value_to_py(py, v)?)?;
                }
                Ok(dict)
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "expected top-level JSON object",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_nulls_removes_null_values() {
        let input = serde_json::json!({
            "a": 1,
            "b": null,
            "c": {
                "d": null,
                "e": "hello"
            },
            "f": [1, null, {"g": null, "h": 2}]
        });

        let result = strip_nulls(input);

        let expected = serde_json::json!({
            "a": 1,
            "c": {
                "e": "hello"
            },
            "f": [1, null, {"h": 2}]
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn strip_nulls_preserves_non_null_values() {
        let input = serde_json::json!({
            "a": 1,
            "b": "text",
            "c": true,
            "d": [1, 2, 3]
        });

        let result = strip_nulls(input.clone());
        assert_eq!(result, input);
    }
}
