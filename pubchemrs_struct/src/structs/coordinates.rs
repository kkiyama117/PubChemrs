#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default,
    serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CoordinateType {
    #[serde(rename = "2d")]
    TwoD,
    #[default]
    #[serde(rename = "3d")]
    ThreeD,
}

impl_enum_str!(CoordinateType {
    TwoD => "2d",
    ThreeD => "3d",
});

#[derive(Debug, Copy, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct Coordinate {
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub z: Option<f32>,
}

impl Coordinate {
    pub fn new(x: f32, y: f32, z: Option<f32>) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            z,
        }
    }

    pub fn coordinate_type(&self) -> CoordinateType {
        match self.z {
            Some(_) => CoordinateType::ThreeD,
            None => CoordinateType::TwoD,
        }
    }
}

impl<'a> IntoIterator for &'a Coordinate {
    type Item = (&'a str, f32);
    type IntoIter = CoordinateIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self,
            index: 0,
        }
    }
}

pub struct CoordinateIterator<'a> {
    inner: &'a Coordinate,
    index: usize,
}

impl<'a> Iterator for CoordinateIterator<'a> {
    type Item = (&'a str, f32);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => self.inner.x.map(|v| ("x", v)),
            2 => self.inner.y.map(|v| ("y", v)),
            3 => self.inner.z.map(|v| ("z", v)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let json = r#"{"x":5.0,"y":3.1}"#;
        let de: Coordinate = serde_json::from_str(json).unwrap();
        assert_eq!(
            de,
            Coordinate {
                x: Some(5.),
                y: Some(3.1),
                z: None
            }
        );
        let ser = serde_json::to_string(&de).unwrap();
        assert_eq!(json, ser);
    }
}
