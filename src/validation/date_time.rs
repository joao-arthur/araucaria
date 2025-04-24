use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for DateTimeValidation {
    fn default() -> Self {
        DateTimeValidation { required: true, operation: None }
    }
}

impl DateTimeValidation {
    pub fn optional(self) -> Self {
        DateTimeValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn le(self, value: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        DateTimeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        DateTimeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::DateTimeValidation;

    #[test]
    fn test_date_time_validation() {
        assert_eq!(DateTimeValidation::default(), DateTimeValidation { required: true, operation: None });
        assert_eq!(DateTimeValidation::default().optional(), DateTimeValidation { required: false, operation: None });
        assert_eq!(
            DateTimeValidation::default().eq("2026-08-12T08:10Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::from("2026-08-12T08:10Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().ne("2027-08-02T10:27Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::from("2027-08-02T10:27Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().gt("2028-07-22T19:41Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::from("2028-07-22T19:41Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().ge("2030-11-25T03:01Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::from("2030-11-25T03:01Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().lt("2031-11-14T00:00Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::from("2031-11-14T00:00Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().le("2033-03-30T01:01Z".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::from("2033-03-30T01:01Z")))) }
        );
        assert_eq!(
            DateTimeValidation::default().btwn("2031-11-14T00:00Z".into(), "2033-03-30T01:01Z".into()),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::Value(OperandValue::from("2031-11-14T00:00Z")),
                    Operand::Value(OperandValue::from("2033-03-30T01:01Z"))
                ))
            }
        );
        assert_eq!(
            DateTimeValidation::default().eq_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().ne_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().gt_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().ge_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().lt_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().le_field("user.info.details.birthdate".into()),
            DateTimeValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateTimeValidation::default().btwn_field("user.info.details.birthdate".into(), "user.info.details.deathdate".into()),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.info.details.birthdate".into()),
                    Operand::FieldPath("user.info.details.deathdate".into())
                ))
            }
        );
    }
}
