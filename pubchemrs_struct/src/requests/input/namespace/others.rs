use crate::requests::input::Namespace;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum GeneNamespace {
    #[default]
    GeneID,
    GeneSymbol,
    Accession,
}

impl_enum_str!(GeneNamespace {
    GeneID => "geneid",
    GeneSymbol => "genesymbol",
    Accession => "accession",
});

impl From<GeneNamespace> for Namespace {
    fn from(value: GeneNamespace) -> Self {
        Self::Gene(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum ProteinNamespace {
    #[default]
    Accession,
    GI,
    Synonim,
}

impl_enum_str!(ProteinNamespace {
    Accession => "accession",
    GI => "gi",
    Synonim => "synonim",
});

impl From<ProteinNamespace> for Namespace {
    fn from(value: ProteinNamespace) -> Self {
        Self::Protein(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum PathWayNamespace {
    #[default]
    Pwacc,
}

impl_enum_str!(PathWayNamespace {
    Pwacc => "pwacc",
});

impl From<PathWayNamespace> for Namespace {
    fn from(value: PathWayNamespace) -> Self {
        Self::PathWay(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum TaxonomyNamespace {
    #[default]
    TaxID,
    Synonim,
}

impl_enum_str!(TaxonomyNamespace {
    TaxID => "taxid",
    Synonim => "synonim",
});

impl From<TaxonomyNamespace> for Namespace {
    fn from(value: TaxonomyNamespace) -> Self {
        Self::Taxonomy(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CellNamespace {
    #[default]
    CellAcc,
    Synonim,
}

impl_enum_str!(CellNamespace {
    CellAcc => "cellacc",
    Synonim => "synonim",
});

impl From<CellNamespace> for Namespace {
    fn from(value: CellNamespace) -> Self {
        Self::Cell(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // GeneNamespace tests
    #[test]
    fn test_gene_namespace_parse() {
        assert_eq!(
            GeneNamespace::from_str("geneid").unwrap(),
            GeneNamespace::GeneID
        );
        assert_eq!(
            GeneNamespace::from_str("genesymbol").unwrap(),
            GeneNamespace::GeneSymbol
        );
        assert_eq!(
            GeneNamespace::from_str("accession").unwrap(),
            GeneNamespace::Accession
        );
    }

    // ProteinNamespace tests
    #[test]
    fn test_protein_namespace_parse() {
        assert_eq!(
            ProteinNamespace::from_str("accession").unwrap(),
            ProteinNamespace::Accession
        );
        assert_eq!(
            ProteinNamespace::from_str("gi").unwrap(),
            ProteinNamespace::GI
        );
        assert_eq!(
            ProteinNamespace::from_str("synonim").unwrap(),
            ProteinNamespace::Synonim
        );
    }

    // PathWayNamespace tests
    #[test]
    fn test_pathway_namespace_parse() {
        assert_eq!(
            PathWayNamespace::from_str("pwacc").unwrap(),
            PathWayNamespace::Pwacc
        );
    }

    // TaxonomyNamespace tests
    #[test]
    fn test_taxonomy_namespace_parse() {
        assert_eq!(
            TaxonomyNamespace::from_str("taxid").unwrap(),
            TaxonomyNamespace::TaxID
        );
        assert_eq!(
            TaxonomyNamespace::from_str("synonim").unwrap(),
            TaxonomyNamespace::Synonim
        );
    }

    // CellNamespace tests
    #[test]
    fn test_cell_namespace_parse() {
        assert_eq!(
            CellNamespace::from_str("cellacc").unwrap(),
            CellNamespace::CellAcc
        );
        assert_eq!(
            CellNamespace::from_str("synonim").unwrap(),
            CellNamespace::Synonim
        );
    }
}
