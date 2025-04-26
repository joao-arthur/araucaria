use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct USizeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for USizeValidation {
    fn default() -> Self {
        USizeValidation { required: true, operation: None }
    }
}

impl USizeValidation {
    pub fn optional(self) -> Self {
        USizeValidation { required: false, ..self }
    }

    pub fn eq(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn ne(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn gt(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn ge(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn lt(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn le(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn btwn(self, value_a: usize, value_b: usize) -> Self {
        USizeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::USize(value_a)), Operand::Value(OperandValue::USize(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        USizeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::USizeValidation;

    #[test]
    fn test_usize_validation() {
        assert_eq!(USizeValidation::default(), USizeValidation { required: true, operation: None });
        assert_eq!(USizeValidation::default().optional(), USizeValidation { required: false, operation: None });
        assert_eq!(
            USizeValidation::default().eq(1),
            USizeValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::USize(1)))) }
        );
        assert_eq!(
            USizeValidation::default().ne(2),
            USizeValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::USize(2)))) }
        );
        assert_eq!(
            USizeValidation::default().gt(3),
            USizeValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::USize(3)))) }
        );
        assert_eq!(
            USizeValidation::default().ge(4),
            USizeValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::USize(4)))) }
        );
        assert_eq!(
            USizeValidation::default().lt(5),
            USizeValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::USize(5)))) }
        );
        assert_eq!(
            USizeValidation::default().le(6),
            USizeValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::USize(6)))) }
        );
        assert_eq!(
            USizeValidation::default().btwn(1, 9),
            USizeValidation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::USize(1)), Operand::Value(OperandValue::USize(9))))
            }
        );
        assert_eq!(
            USizeValidation::default().eq_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().ne_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().gt_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().ge_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().lt_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().le_field("user.personal.info.height".into()),
            USizeValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.personal.info.height".into()))) }
        );
        assert_eq!(
            USizeValidation::default().btwn_field("user.personal.info.min_height".into(), "user.personal.info.max_height".into()),
            USizeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.personal.info.min_height".into()),
                    Operand::FieldPath("user.personal.info.max_height".into())
                ))
            }
        );
    }
}
