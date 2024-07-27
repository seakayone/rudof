use std::collections::HashSet;

use oxigraph::{model::Term, store::Store};
use srdf::RDFNode;

use crate::{
    constraints::{constraint_error::ConstraintError, Evaluate},
    validation_report::report::ValidationReport,
};

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct NotConstraintComponent {
    shape: RDFNode,
}

impl NotConstraintComponent {
    pub fn new(shape: RDFNode) -> Self {
        NotConstraintComponent { shape }
    }
}

impl Evaluate for NotConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct AndConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl AndConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        AndConstraintComponent { shapes }
    }
}

impl Evaluate for AndConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct OrConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl OrConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        OrConstraintComponent { shapes }
    }
}

impl Evaluate for OrConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct XoneConstraintComponent {
    shapes: Vec<RDFNode>,
}

impl XoneConstraintComponent {
    pub fn new(shapes: Vec<RDFNode>) -> Self {
        XoneConstraintComponent { shapes }
    }
}

impl Evaluate for XoneConstraintComponent {
    fn evaluate(
        &self,
        _store: &Store,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        todo!()
    }
}
