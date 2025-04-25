use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct NumUValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for NumUValidation {
    fn default() -> Self {
        NumUValidation { required: true, operation: None }
    }
}

impl NumUValidation {
    pub fn optional(self) -> Self {
        NumUValidation { required: false, ..self }
    }

    pub fn eq(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ne(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn gt(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ge(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn lt(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn le(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn btwn(self, value_a: u64, value_b: u64) -> Self {
        NumUValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::U64(value_a)), Operand::Value(OperandValue::U64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        NumUValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        NumUValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::NumUValidation;

    #[test]
    fn test_num_u_validation() {
        assert_eq!(NumUValidation::default(), NumUValidation { required: true, operation: None });
        assert_eq!(NumUValidation::default().optional(), NumUValidation { required: false, operation: None });
        assert_eq!(
            NumUValidation::default().eq(1),
            NumUValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::U64(1)))) }
        );
        assert_eq!(
            NumUValidation::default().ne(2),
            NumUValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::U64(2)))) }
        );
        assert_eq!(
            NumUValidation::default().gt(3),
            NumUValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::U64(3)))) }
        );
        assert_eq!(
            NumUValidation::default().ge(4),
            NumUValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::U64(4)))) }
        );
        assert_eq!(
            NumUValidation::default().lt(5),
            NumUValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::U64(5)))) }
        );
        assert_eq!(
            NumUValidation::default().le(6),
            NumUValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::U64(6)))) }
        );
        assert_eq!(
            NumUValidation::default().btwn(1, 9),
            NumUValidation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::U64(1)), Operand::Value(OperandValue::U64(9))))
            }
        );
        assert_eq!(
            NumUValidation::default().eq_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().ne_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().gt_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().ge_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().lt_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().le_field("user.personal.info.height".into()),
            NumUValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            NumUValidation::default().btwn_field("user.personal.info.min_height".into(), "user.personal.info.max_height".into()),
            NumUValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.personal.info.min_height".into()),
                    Operand::FieldPath("user.personal.info.max_height".into())
                ))
            }
        );
    }
}
