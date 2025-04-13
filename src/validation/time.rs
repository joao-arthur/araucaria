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
        TimeValidation { operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))), ..self }
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
            TimeValidation::default().eq(String::from("08:10")),
            TimeValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(String::from("08:10"))))) }
        );
        assert_eq!(
            TimeValidation::default().ne(String::from("10:27")),
            TimeValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(String::from("10:27"))))) }
        );
        assert_eq!(
            TimeValidation::default().gt(String::from("19:41")),
            TimeValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(String::from("19:41"))))) }
        );
        assert_eq!(
            TimeValidation::default().ge(String::from("03:01")),
            TimeValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(String::from("03:01"))))) }
        );
        assert_eq!(
            TimeValidation::default().lt(String::from("00:00")),
            TimeValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(String::from("00:00"))))) }
        );
        assert_eq!(
            TimeValidation::default().le(String::from("01:01")),
            TimeValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::Str(String::from("01:01"))))) }
        );
        assert_eq!(
            TimeValidation::default().btwn(String::from("00:00"), String::from("23:59")),
            TimeValidation { required: true, operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(String::from("00:00"))), Operand::Value(OperandValue::Str(String::from("23:59"))))) }
        );
        assert_eq!(
            TimeValidation::default().eq_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().ne_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().gt_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().ge_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().lt_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().le_field(String::from("user.info.details.last_login")),
            TimeValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath(String::from("user.info.details.last_login")))) }
        );
        assert_eq!(
            TimeValidation::default().btwn_field(String::from("user.info.details.last_login"), String::from("user.info.details.last_logout")),
            TimeValidation { required: true, operation: Some(Operation::Btwn(Operand::FieldPath(String::from("user.info.details.last_login")), Operand::FieldPath(String::from("user.info.details.last_logout")))) }
        );
    }
}
