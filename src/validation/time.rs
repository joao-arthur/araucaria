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
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::TimeValidation;

    #[test]
    fn test_time_validation() {
        assert_eq!(TimeValidation::default(), TimeValidation { required: true, operation: None });
        assert_eq!(TimeValidation::default().optional(), TimeValidation { required: false, operation: None });
        assert_eq!(
            TimeValidation::default().eq("08:10".into()),
            TimeValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::Str("08:10".into())))) }
        );
        assert_eq!(
            TimeValidation::default().ne("10:27".into()),
            TimeValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::Str("10:27".into())))) }
        );
        assert_eq!(
            TimeValidation::default().gt("19:41".into()),
            TimeValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::Str("19:41".into())))) }
        );
        assert_eq!(
            TimeValidation::default().ge("03:01".into()),
            TimeValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Str("03:01".into())))) }
        );
        assert_eq!(
            TimeValidation::default().lt("00:00".into()),
            TimeValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::Str("00:00".into())))) }
        );
        assert_eq!(
            TimeValidation::default().le("01:01".into()),
            TimeValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::Str("01:01".into())))) }
        );
        assert_eq!(
            TimeValidation::default().btwn("00:00".into(), "23:59".into()),
            TimeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::Value(OperandValue::Str("00:00".into())),
                    Operand::Value(OperandValue::Str("23:59".into()))
                ))
            }
        );
        assert_eq!(
            TimeValidation::default().eq_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().ne_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().gt_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().ge_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().lt_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().le_field("user.info.details.last_login".into()),
            TimeValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.info.details.last_login".into()))) }
        );
        assert_eq!(
            TimeValidation::default().btwn_field("user.info.details.last_login".into(), "user.info.details.last_logout".into()),
            TimeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.info.details.last_login".into()),
                    Operand::FieldPath("user.info.details.last_logout".into())
                ))
            }
        );
    }
}
