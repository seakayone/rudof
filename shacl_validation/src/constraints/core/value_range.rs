use srdf::literal::Literal;

use crate::{constraints::Evaluate, validation_report::ValidationResult};

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MinExclusiveConstraintComponent {
    literal: Literal,
}

impl MinExclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MinExclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MinExclusiveConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
pub(crate) struct MinInclusiveConstraintComponent {
    literal: Literal,
}

impl MinInclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MinInclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MinInclusiveConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MaxExclusiveConstraintComponent {
    literal: Literal,
}

impl MaxExclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MaxExclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MaxExclusiveConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
pub(crate) struct MaxInclusiveConstraintComponent {
    literal: Literal,
}

impl MaxInclusiveConstraintComponent {
    pub fn new(literal: Literal) -> Self {
        MaxInclusiveConstraintComponent { literal }
    }
}

impl Evaluate for MaxInclusiveConstraintComponent {
    fn evaluate(&self) -> Option<ValidationResult> {
        todo!()
    }
}
