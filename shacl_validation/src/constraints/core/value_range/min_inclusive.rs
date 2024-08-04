use std::collections::HashSet;

use indoc::formatdoc;
use srdf::literal::Literal;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
pub(crate) struct MinInclusive<S: SRDFBasic> {
    min_inclusive: S::Term,
}

impl<S: SRDFBasic> MinInclusive<S> {
    pub fn new(literal: Literal) -> Self {
        MinInclusive {
            min_inclusive: S::object_as_term(&RDFNode::literal(literal)),
        }
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for MinInclusive<S> {
    fn evaluate_default(
        &self,
        _store: &S,
        _value_nodes: HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for MinInclusive<S> {
    fn evaluate_sparql(
        &self,
        store: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} <= {}) }} ",
                node, self.min_inclusive
            };
            let ans = match store.query_ask(&query) {
                Ok(ans) => ans,
                Err(_) => return Err(ConstraintError::Query),
            };
            if !ans {
                report.make_validation_result(Some(node));
            }
        }
        Ok(())
    }
}
