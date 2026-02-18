//! Output format specification for PubChem API responses.

use crate::requests::common::UrlParts;
use std::fmt::Display;
use std::str::FromStr;

/// Output format for API responses.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum OutputFormat {
    /// XML format (API value: `XML`)
    XML(),
    /// ASN.1 text format (API value: `ASNT`)
    ASNT(),
    /// ASN.1 binary format (API value: `ASNB`)
    ASNB(),
    /// JSON format (API value: `JSON`). This is the default.
    JSON(),
    /// JSONP format with callback function name (API value: `JSONP?<callback>`)
    JSONP(String),
    /// SDF (Structure-Data File) format (API value: `SDF`)
    SDF(),
    /// CSV format (API value: `CSV`)
    CSV(),
    /// PNG image format (API value: `PNG`)
    PNG(),
    /// Plain text format (API value: `TXT`)
    TXT(),
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::XML() => write!(f, "XML"),
            OutputFormat::ASNT() => write!(f, "ASNT"),
            OutputFormat::ASNB() => write!(f, "ASNB"),
            OutputFormat::JSON() => write!(f, "JSON"),
            OutputFormat::JSONP(s) => write!(f, "JSONP?{}", s),
            OutputFormat::SDF() => write!(f, "SDF"),
            OutputFormat::CSV() => write!(f, "CSV"),
            OutputFormat::PNG() => write!(f, "PNG"),
            OutputFormat::TXT() => write!(f, "TXT"),
        }
    }
}

impl UrlParts for OutputFormat {
    fn to_url_parts(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl FromStr for OutputFormat {
    type Err = crate::error::ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("JSONP?") {
            let inner = s.trim_start_matches("JSONP?");
            Ok(Self::JSONP(inner.into()))
        } else {
            Ok(match s {
                "XML" => Self::XML(),
                "ASNT" => Self::ASNT(),
                "ASNB" => Self::ASNB(),
                "JSON" => Self::JSON(),
                "SDF" => Self::SDF(),
                "CSV" => Self::CSV(),
                "PNG" => Self::PNG(),
                "TXT" => Self::TXT(),
                _ => Err(crate::error::ParseEnumError::VariantNotFound)?,
            })
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::JSON()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parse_basic() {
        assert_eq!(OutputFormat::from_str("XML").unwrap(), OutputFormat::XML());
        assert_eq!(
            OutputFormat::from_str("ASNT").unwrap(),
            OutputFormat::ASNT()
        );
        assert_eq!(
            OutputFormat::from_str("ASNB").unwrap(),
            OutputFormat::ASNB()
        );
        assert_eq!(
            OutputFormat::from_str("JSON").unwrap(),
            OutputFormat::JSON()
        );
        assert_eq!(OutputFormat::from_str("SDF").unwrap(), OutputFormat::SDF());
        assert_eq!(OutputFormat::from_str("CSV").unwrap(), OutputFormat::CSV());
        assert_eq!(OutputFormat::from_str("PNG").unwrap(), OutputFormat::PNG());
        assert_eq!(OutputFormat::from_str("TXT").unwrap(), OutputFormat::TXT());
    }

    #[test]
    fn test_output_format_parse_jsonp() {
        assert_eq!(
            OutputFormat::from_str("JSONP?callback").unwrap(),
            OutputFormat::JSONP("callback".to_string())
        );
        assert_eq!(
            OutputFormat::from_str("JSONP?myCallback").unwrap(),
            OutputFormat::JSONP("myCallback".to_string())
        );
        assert_eq!(
            OutputFormat::from_str("JSONP?foo.bar.baz").unwrap(),
            OutputFormat::JSONP("foo.bar.baz".to_string())
        );
    }

    #[test]
    fn test_output_format_parse_invalid() {
        assert!(OutputFormat::from_str("invalid").is_err());
        assert!(OutputFormat::from_str("json").is_err()); // Case sensitive
        assert!(OutputFormat::from_str("xml").is_err()); // Case sensitive
        assert!(OutputFormat::from_str("").is_err());
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::XML().to_string(), "XML");
        assert_eq!(OutputFormat::ASNT().to_string(), "ASNT");
        assert_eq!(OutputFormat::ASNB().to_string(), "ASNB");
        assert_eq!(OutputFormat::JSON().to_string(), "JSON");
        assert_eq!(OutputFormat::SDF().to_string(), "SDF");
        assert_eq!(OutputFormat::CSV().to_string(), "CSV");
        assert_eq!(OutputFormat::PNG().to_string(), "PNG");
        assert_eq!(OutputFormat::TXT().to_string(), "TXT");
        assert_eq!(
            OutputFormat::JSONP("callback".to_string()).to_string(),
            "JSONP?callback"
        );
    }

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::JSON());
    }

    #[test]
    fn test_output_format_roundtrip() {
        let test_cases = vec![
            "XML",
            "ASNT",
            "ASNB",
            "JSON",
            "SDF",
            "CSV",
            "PNG",
            "TXT",
            "JSONP?callback",
        ];

        for case in test_cases {
            let parsed = OutputFormat::from_str(case).unwrap();
            assert_eq!(parsed.to_string(), case);
        }
    }
}
