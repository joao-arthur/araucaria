use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for BoolValidation {
    fn default() -> Self {
        BoolValidation { required: true, operation: None }
    }
}

impl BoolValidation {
    pub fn optional(self) -> Self {
        BoolValidation { required: false, ..self }
    }

    pub fn eq(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn ne(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn gt(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn ge(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn lt(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn le(self, value: bool) -> Self {
        BoolValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn btwn(self, value_a: bool, value_b: bool) -> Self {
        BoolValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Bool(value_a)), Operand::Value(OperandValue::Bool(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        BoolValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        BoolValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::BoolValidation;

    #[test]
    fn test_bool_validation() {
        assert_eq!(BoolValidation::default(), BoolValidation { required: true, operation: None });
        assert_eq!(BoolValidation::default().optional(), BoolValidation { required: false, operation: None });
        assert_eq!(
            BoolValidation::default().eq(false),
            BoolValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().ne(false),
            BoolValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().gt(false),
            BoolValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().ge(false),
            BoolValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().lt(false),
            BoolValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().le(false),
            BoolValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().btwn(false, true),
            BoolValidation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true))))
            }
        );
        assert_eq!(
            BoolValidation::default().eq_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().ne_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().gt_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().ge_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().lt_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().le_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().btwn_field(String::from("user.info.details.is_alive"), String::from("user.info.details.is_dead")),
            BoolValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath(String::from("user.info.details.is_alive")),
                    Operand::FieldPath(String::from("user.info.details.is_dead"))
                ))
            }
        );
    }
}
