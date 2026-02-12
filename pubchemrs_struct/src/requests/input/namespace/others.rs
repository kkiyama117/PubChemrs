use crate::requests::input::Namespace;

/// Namespace for the gene domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum GeneNamespace {
    /// NCBI gene ID (API value: `geneid`)
    #[default]
    GeneID,
    /// Gene symbol (API value: `genesymbol`)
    GeneSymbol,
    /// GenBank/RefSeq accession (API value: `accession`)
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

/// Namespace for the protein domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum ProteinNamespace {
    /// Protein accession number (API value: `accession`)
    #[default]
    Accession,
    /// NCBI protein GI number (API value: `gi`)
    GI,
    /// Protein synonym (API value: `synonym`)
    Synonym,
}

impl_enum_str!(ProteinNamespace {
    Accession => "accession",
    GI => "gi",
    Synonym => "synonym",
});

impl From<ProteinNamespace> for Namespace {
    fn from(value: ProteinNamespace) -> Self {
        Self::Protein(value)
    }
}

/// Namespace for the pathway domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum PathWayNamespace {
    /// Pathway accession number (API value: `pwacc`)
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

/// Namespace for the taxonomy domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum TaxonomyNamespace {
    /// NCBI taxonomy ID (API value: `taxid`)
    #[default]
    TaxID,
    /// Taxonomy synonym (API value: `synonym`)
    Synonym,
}

impl_enum_str!(TaxonomyNamespace {
    TaxID => "taxid",
    Synonym => "synonym",
});

impl From<TaxonomyNamespace> for Namespace {
    fn from(value: TaxonomyNamespace) -> Self {
        Self::Taxonomy(value)
    }
}

/// Namespace for the cell line domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub enum CellNamespace {
    /// Cell line accession number (API value: `cellacc`)
    #[default]
    CellAcc,
    /// Cell line synonym (API value: `synonym`)
    Synonym,
}

impl_enum_str!(CellNamespace {
    CellAcc => "cellacc",
    Synonym => "synonym",
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
            ProteinNamespace::from_str("synonym").unwrap(),
            ProteinNamespace::Synonym
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
            TaxonomyNamespace::from_str("synonym").unwrap(),
            TaxonomyNamespace::Synonym
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
            CellNamespace::from_str("synonym").unwrap(),
            CellNamespace::Synonym
        );
    }
}
