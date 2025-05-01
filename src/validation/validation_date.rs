use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct DateValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for DateValidation {
    fn default() -> Self {
        DateValidation { required: true, operation: None }
    }
}

impl DateValidation {
    pub fn optional(self) -> Self {
        DateValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn le(self, value: String) -> Self {
        DateValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        DateValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        DateValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        DateValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::DateValidation;

    const FIELD: &str = "user.info.details.birthdate";
    const FIELD_B: &str = "user.info.details.deathdate";

    const OP_VALUE_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::Value(OperandValue::from("2026-08-12"))));
    const OP_VALUE_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::Value(OperandValue::from("2027-08-02"))));
    const OP_VALUE_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::Value(OperandValue::from("2028-07-22"))));
    const OP_VALUE_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::Value(OperandValue::from("2030-11-25"))));
    const OP_VALUE_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::Value(OperandValue::from("2031-11-14"))));
    const OP_VALUE_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::Value(OperandValue::from("2033-03-30"))));
    const OP_VALUE_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::Value(OperandValue::from("2031-11-14")), Operand::Value(OperandValue::from("2033-03-30"))));
    const OP_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::FieldPath(FIELD.into()), Operand::FieldPath(FIELD_B.into())));

    #[test]
    fn date_validation() {
        assert_eq!(DateValidation::default(), DateValidation { required: true, operation: None });
        assert_eq!(DateValidation::default().optional(), DateValidation { required: false, operation: None });
        assert_eq!(DateValidation::default().eq("2026-08-12".into()), DateValidation { required: true, operation: Some(OP_VALUE_EQ.clone()) });
        assert_eq!(DateValidation::default().ne("2027-08-02".into()), DateValidation { required: true, operation: Some(OP_VALUE_NE.clone()) });
        assert_eq!(DateValidation::default().gt("2028-07-22".into()), DateValidation { required: true, operation: Some(OP_VALUE_GT.clone()) });
        assert_eq!(DateValidation::default().ge("2030-11-25".into()), DateValidation { required: true, operation: Some(OP_VALUE_GE.clone()) });
        assert_eq!(DateValidation::default().lt("2031-11-14".into()), DateValidation { required: true, operation: Some(OP_VALUE_LT.clone()) });
        assert_eq!(DateValidation::default().le("2033-03-30".into()), DateValidation { required: true, operation: Some(OP_VALUE_LE.clone()) });
        assert_eq!(DateValidation::default().btwn("2031-11-14".into(), "2033-03-30".into()), DateValidation { required: true, operation: Some(OP_VALUE_BTWN.clone()) });
        assert_eq!(DateValidation::default().eq_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_EQ.clone()) });
        assert_eq!(DateValidation::default().ne_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_NE.clone()) });
        assert_eq!(DateValidation::default().gt_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_GT.clone()) });
        assert_eq!(DateValidation::default().ge_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_GE.clone()) });
        assert_eq!(DateValidation::default().lt_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_LT.clone()) });
        assert_eq!(DateValidation::default().le_field(FIELD.into()), DateValidation { required: true, operation: Some(OP_FIELD_LE.clone()) });
        assert_eq!(DateValidation::default().btwn_field(FIELD.into(), FIELD_B.into()), DateValidation { required: true, operation: Some(OP_FIELD_BTWN.clone()) });
    }
}
