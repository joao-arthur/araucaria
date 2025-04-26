use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct I64Validation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for I64Validation {
    fn default() -> Self {
        I64Validation { required: true, operation: None }
    }
}

impl I64Validation {
    pub fn optional(self) -> Self {
        I64Validation { required: false, ..self }
    }

    pub fn eq(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Eq(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ne(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Ne(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn gt(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Gt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ge(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Ge(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn lt(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Lt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn le(self, value: i64) -> Self {
        I64Validation { operation: Some(Operation::Le(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn btwn(self, value_a: i64, value_b: i64) -> Self {
        I64Validation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::I64(value_a)), Operand::Value(OperandValue::I64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        I64Validation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        I64Validation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::I64Validation;

    #[test]
    fn test_i64_validation() {
        assert_eq!(I64Validation::default(), I64Validation { required: true, operation: None });
        assert_eq!(I64Validation::default().optional(), I64Validation { required: false, operation: None });
        assert_eq!(
            I64Validation::default().eq(-1),
            I64Validation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::I64(-1)))) }
        );
        assert_eq!(
            I64Validation::default().ne(-2),
            I64Validation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::I64(-2)))) }
        );
        assert_eq!(
            I64Validation::default().gt(-3),
            I64Validation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::I64(-3)))) }
        );
        assert_eq!(
            I64Validation::default().ge(-4),
            I64Validation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::I64(-4)))) }
        );
        assert_eq!(
            I64Validation::default().lt(-5),
            I64Validation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::I64(-5)))) }
        );
        assert_eq!(
            I64Validation::default().le(-6),
            I64Validation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::I64(-6)))) }
        );
        assert_eq!(
            I64Validation::default().btwn(-42, 42),
            I64Validation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::I64(-42)), Operand::Value(OperandValue::I64(42))))
            }
        );
        assert_eq!(
            I64Validation::default().eq_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().ne_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().gt_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().ge_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().lt_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().le_field("user.data.info.score_change".into()),
            I64Validation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.data.info.score_change".into()))) }
        );
        assert_eq!(
            I64Validation::default().btwn_field("user.data.info.min_score".into(), "user.data.info.max_score".into()),
            I64Validation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.data.info.min_score".into()),
                    Operand::FieldPath("user.data.info.max_score".into())
                ))
            }
        );
    }
}
