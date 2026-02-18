//! Operations for Gene, Protein, PathWay, Taxonomy, and Cell domains.
//!
//! These domains share similar operation patterns with fewer variants
//! than compound, substance, or assay.

/// Operations available for the gene domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum GeneOperationSpecification {
    /// Retrieve gene summary (API value: `summary`)
    #[default]
    Summary,
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids,
    /// Retrieve concise gene data (API value: `concise`)
    Concise,
    /// Retrieve pathway accessions (API value: `pwaccs`)
    Pwaccs,
}

impl_enum_str!(GeneOperationSpecification {
    Summary => "summary",
    Aids => "aids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

/// Operations available for the protein domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum ProteinOperationSpecification {
    /// Retrieve protein summary (API value: `summary`)
    #[default]
    Summary,
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids,
    /// Retrieve concise protein data (API value: `concise`)
    Concise,
    /// Retrieve pathway accessions (API value: `pwaccs`)
    Pwaccs,
}

impl_enum_str!(ProteinOperationSpecification {
    Summary => "summary",
    Aids => "aids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

/// Operations available for the pathway domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum PathWayOperationSpecification {
    /// Retrieve pathway summary (API value: `summary`)
    #[default]
    Summary,
    /// Retrieve associated compound IDs (API value: `cids`)
    Cids,
    /// Retrieve concise pathway data (API value: `concise`)
    Concise,
    /// Retrieve pathway accessions (API value: `pwaccs`)
    Pwaccs,
}

impl_enum_str!(PathWayOperationSpecification {
    Summary => "summary",
    Cids => "cids",
    Concise => "concise",
    Pwaccs => "pwaccs",
});

/// Operations available for the taxonomy domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum TaxonomyOperationSpecification {
    /// Retrieve taxonomy summary (API value: `summary`)
    #[default]
    Summary,
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids,
}

impl_enum_str!(TaxonomyOperationSpecification {
    Summary => "summary",
    Aids => "aids",
});

/// Operations available for the cell line domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pyo3", pyo3::pyclass(from_py_object))]
pub enum CellOperationSpecification {
    /// Retrieve cell line summary (API value: `summary`)
    #[default]
    Summary,
    /// Retrieve associated assay IDs (API value: `aids`)
    Aids,
}

impl_enum_str!(CellOperationSpecification {
    Summary => "summary",
    Aids => "aids",
});

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_gene_operation_parse() {
        assert_eq!(
            GeneOperationSpecification::from_str("summary").unwrap(),
            GeneOperationSpecification::Summary
        );
        assert_eq!(
            GeneOperationSpecification::from_str("aids").unwrap(),
            GeneOperationSpecification::Aids
        );
        assert_eq!(
            GeneOperationSpecification::from_str("concise").unwrap(),
            GeneOperationSpecification::Concise
        );
        assert_eq!(
            GeneOperationSpecification::from_str("pwaccs").unwrap(),
            GeneOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_gene_operation_display() {
        assert_eq!(GeneOperationSpecification::Summary.to_string(), "summary");
        assert_eq!(GeneOperationSpecification::Aids.to_string(), "aids");
        assert_eq!(GeneOperationSpecification::Concise.to_string(), "concise");
        assert_eq!(GeneOperationSpecification::Pwaccs.to_string(), "pwaccs");
    }

    #[test]
    fn test_gene_operation_default() {
        assert_eq!(
            GeneOperationSpecification::default(),
            GeneOperationSpecification::Summary
        );
    }

    #[test]
    fn test_protein_operation_parse() {
        assert_eq!(
            ProteinOperationSpecification::from_str("summary").unwrap(),
            ProteinOperationSpecification::Summary
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("aids").unwrap(),
            ProteinOperationSpecification::Aids
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("concise").unwrap(),
            ProteinOperationSpecification::Concise
        );
        assert_eq!(
            ProteinOperationSpecification::from_str("pwaccs").unwrap(),
            ProteinOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_protein_operation_default() {
        assert_eq!(
            ProteinOperationSpecification::default(),
            ProteinOperationSpecification::Summary
        );
    }

    #[test]
    fn test_pathway_operation_parse() {
        assert_eq!(
            PathWayOperationSpecification::from_str("summary").unwrap(),
            PathWayOperationSpecification::Summary
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("cids").unwrap(),
            PathWayOperationSpecification::Cids
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("concise").unwrap(),
            PathWayOperationSpecification::Concise
        );
        assert_eq!(
            PathWayOperationSpecification::from_str("pwaccs").unwrap(),
            PathWayOperationSpecification::Pwaccs
        );
    }

    #[test]
    fn test_pathway_operation_default() {
        assert_eq!(
            PathWayOperationSpecification::default(),
            PathWayOperationSpecification::Summary
        );
    }

    #[test]
    fn test_taxonomy_operation_parse() {
        assert_eq!(
            TaxonomyOperationSpecification::from_str("summary").unwrap(),
            TaxonomyOperationSpecification::Summary
        );
        assert_eq!(
            TaxonomyOperationSpecification::from_str("aids").unwrap(),
            TaxonomyOperationSpecification::Aids
        );
    }

    #[test]
    fn test_taxonomy_operation_default() {
        assert_eq!(
            TaxonomyOperationSpecification::default(),
            TaxonomyOperationSpecification::Summary
        );
    }

    #[test]
    fn test_cell_operation_parse() {
        assert_eq!(
            CellOperationSpecification::from_str("summary").unwrap(),
            CellOperationSpecification::Summary
        );
        assert_eq!(
            CellOperationSpecification::from_str("aids").unwrap(),
            CellOperationSpecification::Aids
        );
    }

    #[test]
    fn test_cell_operation_default() {
        assert_eq!(
            CellOperationSpecification::default(),
            CellOperationSpecification::Summary
        );
    }
}
