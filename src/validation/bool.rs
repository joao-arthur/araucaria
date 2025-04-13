use crate::operation::{Operand, OperandValue, OperationEq};

#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub operation: Option<OperationEq>,
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
        BoolValidation { operation: Some(OperationEq::Eq(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn ne(self, value: bool) -> Self {
        BoolValidation { operation: Some(OperationEq::Ne(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn eq_field(self, field: String) -> Self {
        BoolValidation { operation: Some(OperationEq::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        BoolValidation { operation: Some(OperationEq::Ne(Operand::FieldPath(field))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, OperationEq};

    use super::BoolValidation;

    #[test]
    fn test_bool_validation() {
        assert_eq!(BoolValidation::default(), BoolValidation { required: true, operation: None });
        assert_eq!(BoolValidation::default().optional(), BoolValidation { required: false, operation: None });
        assert_eq!(
            BoolValidation::default().eq(false),
            BoolValidation { required: true, operation: Some(OperationEq::Eq(Operand::Value(OperandValue::Bool(false)))) }
        );
        assert_eq!(
            BoolValidation::default().ne(true),
            BoolValidation { required: true, operation: Some(OperationEq::Ne(Operand::Value(OperandValue::Bool(true)))) }
        );
        assert_eq!(
            BoolValidation::default().eq_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(OperationEq::Eq(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
        assert_eq!(
            BoolValidation::default().ne_field(String::from("user.info.details.is_alive")),
            BoolValidation { required: true, operation: Some(OperationEq::Ne(Operand::FieldPath(String::from("user.info.details.is_alive")))) }
        );
    }
}
