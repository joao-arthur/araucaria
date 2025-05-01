use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct TimeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for TimeValidation {
    fn default() -> Self {
        TimeValidation { required: true, operation: None }
    }
}

impl TimeValidation {
    pub fn optional(self) -> Self {
        TimeValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn le(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        TimeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        TimeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        TimeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::TimeValidation;

    const FIELD: &str = "user.info.details.last_login";
    const FIELD_B: &str = "user.info.details.last_logout";

    const OP_VALUE_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::Value(OperandValue::from("08:10"))));
    const OP_VALUE_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::Value(OperandValue::from("10:27"))));
    const OP_VALUE_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::Value(OperandValue::from("19:41"))));
    const OP_VALUE_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::Value(OperandValue::from("03:01"))));
    const OP_VALUE_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::Value(OperandValue::from("00:00"))));
    const OP_VALUE_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::Value(OperandValue::from("01:01"))));
    const OP_VALUE_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::Value(OperandValue::from("00:00")), Operand::Value(OperandValue::from("23:59"))));
    const OP_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::FieldPath(FIELD.into()), Operand::FieldPath(FIELD_B.into())));

    #[test]
    fn time_validation() {
        assert_eq!(TimeValidation::default(), TimeValidation { required: true, operation: None });
        assert_eq!(TimeValidation::default().optional(), TimeValidation { required: false, operation: None });
        assert_eq!(TimeValidation::default().eq("08:10".into()), TimeValidation { required: true, operation: Some(OP_VALUE_EQ.clone()) });
        assert_eq!(TimeValidation::default().ne("10:27".into()), TimeValidation { required: true, operation: Some(OP_VALUE_NE.clone()) });
        assert_eq!(TimeValidation::default().gt("19:41".into()), TimeValidation { required: true, operation: Some(OP_VALUE_GT.clone()) });
        assert_eq!(TimeValidation::default().ge("03:01".into()), TimeValidation { required: true, operation: Some(OP_VALUE_GE.clone()) });
        assert_eq!(TimeValidation::default().lt("00:00".into()), TimeValidation { required: true, operation: Some(OP_VALUE_LT.clone()) });
        assert_eq!(TimeValidation::default().le("01:01".into()), TimeValidation { required: true, operation: Some(OP_VALUE_LE.clone()) });
        assert_eq!(TimeValidation::default().btwn("00:00".into(), "23:59".into()), TimeValidation { required: true, operation: Some(OP_VALUE_BTWN.clone()) });
        assert_eq!(TimeValidation::default().eq_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_EQ.clone()) });
        assert_eq!(TimeValidation::default().ne_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_NE.clone()) });
        assert_eq!(TimeValidation::default().gt_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_GT.clone()) });
        assert_eq!(TimeValidation::default().ge_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_GE.clone()) });
        assert_eq!(TimeValidation::default().lt_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_LT.clone()) });
        assert_eq!(TimeValidation::default().le_field(FIELD.into()), TimeValidation { required: true, operation: Some(OP_FIELD_LE.clone()) });
        assert_eq!(TimeValidation::default().btwn_field(FIELD.into(), FIELD_B.into()), TimeValidation { required: true, operation: Some(OP_FIELD_BTWN.clone()) });
    }
}
