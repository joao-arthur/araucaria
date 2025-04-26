use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct ISizeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for ISizeValidation {
    fn default() -> Self {
        ISizeValidation { required: true, operation: None }
    }
}

impl ISizeValidation {
    pub fn optional(self) -> Self {
        ISizeValidation { required: false, ..self }
    }

    pub fn eq(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ne(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn gt(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ge(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn lt(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn le(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn btwn(self, value_a: isize, value_b: isize) -> Self {
        ISizeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::ISize(value_a)), Operand::Value(OperandValue::ISize(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        ISizeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::ISizeValidation;

    #[test]
    fn test_isize_validation() {
        assert_eq!(ISizeValidation::default(), ISizeValidation { required: true, operation: None });
        assert_eq!(ISizeValidation::default().optional(), ISizeValidation { required: false, operation: None });
        assert_eq!(
            ISizeValidation::default().eq(-1),
            ISizeValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::ISize(-1)))) }
        );
        assert_eq!(
            ISizeValidation::default().ne(-2),
            ISizeValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::ISize(-2)))) }
        );
        assert_eq!(
            ISizeValidation::default().gt(-3),
            ISizeValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::ISize(-3)))) }
        );
        assert_eq!(
            ISizeValidation::default().ge(-4),
            ISizeValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::ISize(-4)))) }
        );
        assert_eq!(
            ISizeValidation::default().lt(-5),
            ISizeValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::ISize(-5)))) }
        );
        assert_eq!(
            ISizeValidation::default().le(-6),
            ISizeValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::ISize(-6)))) }
        );
        assert_eq!(
            ISizeValidation::default().btwn(-42, 42),
            ISizeValidation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::ISize(-42)), Operand::Value(OperandValue::ISize(42))))
            }
        );
        assert_eq!(
            ISizeValidation::default().eq_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().ne_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().gt_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().ge_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().lt_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().le_field("user.data.info.score_change".into()),
            ISizeValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            ISizeValidation::default().btwn_field("user.data.info.min_score".into(), "user.data.info.max_score".into()),
            ISizeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.data.info.min_score".into()),
                    Operand::FieldPath("user.data.info.max_score".into())
                ))
            }
        );
    }
}
