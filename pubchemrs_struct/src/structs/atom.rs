use crate::error::PubChemError;
use crate::structs::coordinates::{Coordinate, CoordinateType};
use std::collections::HashMap;

/// Represents an atom in a compound's structure.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct Atom {
    pub aid: u32,
    pub number: u8,
    pub element: Element,
    #[serde(flatten)]
    pub coordinate: Option<Coordinate>,
    #[serde(skip_serializing_if = "Self::is_charge_zero")]
    #[serde(default)]
    pub charge: i32,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atom({}, {})", self.aid, self.element)
    }
}

impl Atom {
    pub fn new(
        aid: u32,
        element: Element,
        x: f32,
        y: f32,
        z: Option<f32>,
        charge: Option<i32>,
    ) -> Self {
        let coordinate = Coordinate::new(x, y, z);
        let number = element as u8;
        Self {
            aid,
            number,
            element,
            coordinate: Some(coordinate),
            charge: charge.unwrap_or(0),
        }
    }

    pub(crate) fn _from_record_data(
        aid: u32,
        element: Element,
        coordinate: Option<Coordinate>,
        charge: i32,
    ) -> Self {
        Self {
            aid,
            number: element as u8,
            element,
            coordinate,
            charge,
        }
    }

    pub fn coordinate_type(&self) -> CoordinateType {
        self.coordinate.unwrap_or_default().coordinate_type()
    }

    fn is_charge_zero(charge: &i32) -> bool {
        *charge == 0
    }
}

/// All 118 chemical elements plus PubChem special atom types.
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
    serde::Serialize, serde::Deserialize,
)]
#[repr(u8)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum Element {
    #[default]
    H = 1, He = 2, Li = 3, Be = 4, B = 5, C = 6, N = 7, O = 8, F = 9, Ne = 10,
    Na = 11, Mg = 12, Al = 13, Si = 14, P = 15, S = 16, Cl = 17, Ar = 18, K = 19, Ca = 20,
    Sc = 21, Ti = 22, V = 23, Cr = 24, Mn = 25, Fe = 26, Co = 27, Ni = 28, Cu = 29, Zn = 30,
    Ga = 31, Ge = 32, As = 33, Se = 34, Br = 35, Kr = 36, Rb = 37, Sr = 38, Y = 39, Zr = 40,
    Nb = 41, Mo = 42, Tc = 43, Ru = 44, Rh = 45, Pd = 46, Ag = 47, Cd = 48, In = 49, Sn = 50,
    Sb = 51, Te = 52, I = 53, Xe = 54, Cs = 55, Ba = 56, La = 57, Ce = 58, Pr = 59, Nd = 60,
    Pm = 61, Sm = 62, Eu = 63, Gd = 64, Tb = 65, Dy = 66, Ho = 67, Er = 68, Tm = 69, Yb = 70,
    Lu = 71, Hf = 72, Ta = 73, W = 74, Re = 75, Os = 76, Ir = 77, Pt = 78, Au = 79, Hg = 80,
    Tl = 81, Pb = 82, Bi = 83, Po = 84, At = 85, Rn = 86, Fr = 87, Ra = 88, Ac = 89, Th = 90,
    Pa = 91, U = 92, Np = 93, Pu = 94, Am = 95, Cm = 96, Bk = 97, Cf = 98, Es = 99, Fm = 100,
    Md = 101, No = 102, Lr = 103, Rf = 104, Db = 105, Sg = 106, Bh = 107, Hs = 108, Mt = 109,
    Ds = 110, Rg = 111, Cn = 112, Nh = 113, Fl = 114, Mc = 115, Lv = 116, Ts = 117, Og = 118,
    /// Lone Pair
    Lp = 252,
    /// Rgroup Label
    R = 253,
    /// Dummy atom
    Dummy = 254,
    /// Unspecified atom (asterisk)
    Unspecified = 255,
}

impl_enum_str!(Element {
    H => "H", He => "He", Li => "Li", Be => "Be", B => "B", C => "C", N => "N", O => "O",
    F => "F", Ne => "Ne", Na => "Na", Mg => "Mg", Al => "Al", Si => "Si", P => "P", S => "S",
    Cl => "Cl", Ar => "Ar", K => "K", Ca => "Ca", Sc => "Sc", Ti => "Ti", V => "V", Cr => "Cr",
    Mn => "Mn", Fe => "Fe", Co => "Co", Ni => "Ni", Cu => "Cu", Zn => "Zn", Ga => "Ga",
    Ge => "Ge", As => "As", Se => "Se", Br => "Br", Kr => "Kr", Rb => "Rb", Sr => "Sr",
    Y => "Y", Zr => "Zr", Nb => "Nb", Mo => "Mo", Tc => "Tc", Ru => "Ru", Rh => "Rh",
    Pd => "Pd", Ag => "Ag", Cd => "Cd", In => "In", Sn => "Sn", Sb => "Sb", Te => "Te",
    I => "I", Xe => "Xe", Cs => "Cs", Ba => "Ba", La => "La", Ce => "Ce", Pr => "Pr",
    Nd => "Nd", Pm => "Pm", Sm => "Sm", Eu => "Eu", Gd => "Gd", Tb => "Tb", Dy => "Dy",
    Ho => "Ho", Er => "Er", Tm => "Tm", Yb => "Yb", Lu => "Lu", Hf => "Hf", Ta => "Ta",
    W => "W", Re => "Re", Os => "Os", Ir => "Ir", Pt => "Pt", Au => "Au", Hg => "Hg",
    Tl => "Tl", Pb => "Pb", Bi => "Bi", Po => "Po", At => "At", Rn => "Rn", Fr => "Fr",
    Ra => "Ra", Ac => "Ac", Th => "Th", Pa => "Pa", U => "U", Np => "Np", Pu => "Pu",
    Am => "Am", Cm => "Cm", Bk => "Bk", Cf => "Cf", Es => "Es", Fm => "Fm", Md => "Md",
    No => "No", Lr => "Lr", Rf => "Rf", Db => "Db", Sg => "Sg", Bh => "Bh", Hs => "Hs",
    Mt => "Mt", Ds => "Ds", Rg => "Rg", Cn => "Cn", Nh => "Nh", Fl => "Fl", Mc => "Mc",
    Lv => "Lv", Ts => "Ts", Og => "Og",
    Lp => "Lp", R => "R", Dummy => "Dummy", Unspecified => "*"
});

impl_from_repr!(Element: u8 {
    H = 1, He = 2, Li = 3, Be = 4, B = 5, C = 6, N = 7, O = 8, F = 9, Ne = 10,
    Na = 11, Mg = 12, Al = 13, Si = 14, P = 15, S = 16, Cl = 17, Ar = 18, K = 19, Ca = 20,
    Sc = 21, Ti = 22, V = 23, Cr = 24, Mn = 25, Fe = 26, Co = 27, Ni = 28, Cu = 29, Zn = 30,
    Ga = 31, Ge = 32, As = 33, Se = 34, Br = 35, Kr = 36, Rb = 37, Sr = 38, Y = 39, Zr = 40,
    Nb = 41, Mo = 42, Tc = 43, Ru = 44, Rh = 45, Pd = 46, Ag = 47, Cd = 48, In = 49, Sn = 50,
    Sb = 51, Te = 52, I = 53, Xe = 54, Cs = 55, Ba = 56, La = 57, Ce = 58, Pr = 59, Nd = 60,
    Pm = 61, Sm = 62, Eu = 63, Gd = 64, Tb = 65, Dy = 66, Ho = 67, Er = 68, Tm = 69, Yb = 70,
    Lu = 71, Hf = 72, Ta = 73, W = 74, Re = 75, Os = 76, Ir = 77, Pt = 78, Au = 79, Hg = 80,
    Tl = 81, Pb = 82, Bi = 83, Po = 84, At = 85, Rn = 86, Fr = 87, Ra = 88, Ac = 89, Th = 90,
    Pa = 91, U = 92, Np = 93, Pu = 94, Am = 95, Cm = 96, Bk = 97, Cf = 98, Es = 99, Fm = 100,
    Md = 101, No = 102, Lr = 103, Rf = 104, Db = 105, Sg = 106, Bh = 107, Hs = 108, Mt = 109,
    Ds = 110, Rg = 111, Cn = 112, Nh = 113, Fl = 114, Mc = 115, Lv = 116, Ts = 117, Og = 118,
    Lp = 252, R = 253, Dummy = 254, Unspecified = 255
});

impl_variant_array!(Element {
    H, He, Li, Be, B, C, N, O, F, Ne,
    Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca,
    Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn,
    Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y, Zr,
    Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn,
    Sb, Te, I, Xe, Cs, Ba, La, Ce, Pr, Nd,
    Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb,
    Lu, Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg,
    Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th,
    Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm,
    Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt,
    Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
    Lp, R, Dummy, Unspecified
});

impl Element {
    pub fn get_hashmap() -> HashMap<usize, &'static str> {
        let mut base: HashMap<_, _> = Element::VARIANTS
            .iter()
            .map(|item| ((*item as usize), item.as_ref()))
            .collect();
        base.entry(Element::Dummy as usize).and_modify(|a| *a = "*");
        base
    }
}

impl TryFrom<u8> for Element {
    type Error = PubChemError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(PubChemError::ParseEnum(
            crate::error::ParseEnumError::VariantNotFound,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_serialize() {
        let atom = Atom::new(3, Element::from_repr(5).unwrap(), 0.0, 0.0, Some(1.0), None);
        let ser = serde_json::to_string(&atom).unwrap();
        let de: Atom = serde_json::from_str(&ser).unwrap();
        assert_eq!(atom, de);
    }

    #[test]
    fn test_element_display() {
        assert_eq!(Element::H.to_string(), "H");
        assert_eq!(Element::Fe.to_string(), "Fe");
        assert_eq!(Element::Unspecified.to_string(), "*");
    }

    #[test]
    fn test_element_from_repr() {
        assert_eq!(Element::from_repr(1), Some(Element::H));
        assert_eq!(Element::from_repr(6), Some(Element::C));
        assert_eq!(Element::from_repr(255), Some(Element::Unspecified));
        assert_eq!(Element::from_repr(0), None);
    }

    #[test]
    fn test_get_hashmap() {
        let result = Element::get_hashmap();
        assert_eq!(result[&1], "H");
        assert_eq!(result[&6], "C");
        assert_eq!(result[&254], "*"); // Dummy overridden to "*"
        assert_eq!(result[&255], "*"); // Unspecified
    }
}
