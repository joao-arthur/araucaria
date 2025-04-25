use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct F64Validation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for F64Validation {
    fn default() -> Self {
        F64Validation { required: true, operation: None }
    }
}

impl F64Validation {
    pub fn optional(self) -> Self {
        F64Validation { required: false, ..self }
    }

    pub fn eq(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Eq(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn ne(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Ne(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn gt(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Gt(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn ge(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Ge(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn lt(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Lt(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn le(self, value: f64) -> Self {
        F64Validation { operation: Some(Operation::Le(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn btwn(self, value_a: f64, value_b: f64) -> Self {
        F64Validation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::F64(value_a)), Operand::Value(OperandValue::F64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        F64Validation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        F64Validation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::F64Validation;

    #[test]
    fn test_num_f_validation() {
        assert_eq!(F64Validation::default(), F64Validation { required: true, operation: None });
        assert_eq!(F64Validation::default().optional(), F64Validation { required: false, operation: None });
        assert_eq!(
            F64Validation::default().eq(-1.5),
            F64Validation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::F64(-1.5)))) }
        );
        assert_eq!(
            F64Validation::default().ne(-2.5),
            F64Validation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::F64(-2.5)))) }
        );
        assert_eq!(
            F64Validation::default().gt(-3.5),
            F64Validation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::F64(-3.5)))) }
        );
        assert_eq!(
            F64Validation::default().ge(-4.5),
            F64Validation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::F64(-4.5)))) }
        );
        assert_eq!(
            F64Validation::default().lt(-5.5),
            F64Validation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::F64(-5.5)))) }
        );
        assert_eq!(
            F64Validation::default().le(-6.5),
            F64Validation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::F64(-6.5)))) }
        );
        assert_eq!(
            F64Validation::default().btwn(-42.5, 42.5),
            F64Validation {
                required: true,
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::F64(-42.5)), Operand::Value(OperandValue::F64(42.5))))
            }
        );
        assert_eq!(
            F64Validation::default().eq_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().ne_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().gt_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().ge_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().lt_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().le_field("user.balance.value".into()),
            F64Validation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.balance.value".into()))) }
        );
        assert_eq!(
            F64Validation::default().btwn_field("user.balance.value".into(), "user.balance.limit".into()),
            F64Validation {
                required: true,
                operation: Some(Operation::Btwn(Operand::FieldPath("user.balance.value".into()), Operand::FieldPath("user.balance.limit".into())))
            }
        );
    }
}
