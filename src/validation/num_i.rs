use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct NumIValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for NumIValidation {
    fn default() -> Self {
        NumIValidation { required: true, operation: None }
    }
}

impl NumIValidation {
    pub fn optional(self) -> Self {
        NumIValidation { required: false, ..self }
    }

    pub fn eq(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ne(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn gt(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ge(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn lt(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn le(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn btwn(self, value_a: i64, value_b: i64) -> Self {
        NumIValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::I64(value_a)), Operand::Value(OperandValue::I64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        NumIValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        NumIValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::NumIValidation;

    #[test]
    fn test_num_i_validation() {
        assert_eq!(NumIValidation::default(), NumIValidation { required: true, operation: None });
        assert_eq!(NumIValidation::default().optional(), NumIValidation { required: false, operation: None });
        assert_eq!(
            NumIValidation::default().eq(-1),
            NumIValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::I64(-1)))) }
        );
        assert_eq!(
            NumIValidation::default().ne(-2),
            NumIValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::I64(-2)))) }
        );
        assert_eq!(
            NumIValidation::default().gt(-3),
            NumIValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::I64(-3)))) }
        );
        assert_eq!(
            NumIValidation::default().ge(-4),
            NumIValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::I64(-4)))) }
        );
        assert_eq!(
            NumIValidation::default().lt(-5),
            NumIValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::I64(-5)))) }
        );
        assert_eq!(
            NumIValidation::default().le(-6),
            NumIValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::I64(-6)))) }
        );
        assert_eq!(
            NumIValidation::default().btwn(-42, 42),
            NumIValidation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::I64(-42)), Operand::Value(OperandValue::I64(42))))
            }
        );
        assert_eq!(
            NumIValidation::default().eq_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().ne_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().gt_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().ge_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().lt_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().le_field("user.data.info.score_change".into()),
            NumIValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            NumIValidation::default().btwn_field("user.data.info.min_score".into(), "user.data.info.max_score".into()),
            NumIValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.data.info.min_score".into()),
                    Operand::FieldPath("user.data.info.max_score".into())
                ))
            }
        );
    }
}
