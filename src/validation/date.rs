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
mod test {
    use crate::operation::{Operand, OperandValue, Operation};

    use super::DateValidation;

    #[test]
    fn test_date_validation() {
        assert_eq!(DateValidation::default(), DateValidation { required: true, operation: None });
        assert_eq!(DateValidation::default().optional(), DateValidation { required: false, operation: None });
        assert_eq!(
            DateValidation::default().eq("2026-08-12".into()),
            DateValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::Str("2026-08-12".into())))) }
        );
        assert_eq!(
            DateValidation::default().ne("2027-08-02".into()),
            DateValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::Str("2027-08-02".into())))) }
        );
        assert_eq!(
            DateValidation::default().gt("2028-07-22".into()),
            DateValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::Str("2028-07-22".into())))) }
        );
        assert_eq!(
            DateValidation::default().ge("2030-11-25".into()),
            DateValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Str("2030-11-25".into())))) }
        );
        assert_eq!(
            DateValidation::default().lt("2031-11-14".into()),
            DateValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::Str("2031-11-14".into())))) }
        );
        assert_eq!(
            DateValidation::default().le("2033-03-30".into()),
            DateValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::Str("2033-03-30".into())))) }
        );
        assert_eq!(
            DateValidation::default().btwn("2031-11-14".into(), "2033-03-30".into()),
            DateValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::Value(OperandValue::Str("2031-11-14".into())),
                    Operand::Value(OperandValue::Str("2033-03-30".into()))
                ))
            }
        );
        assert_eq!(
            DateValidation::default().eq_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Eq(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().ne_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Ne(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().gt_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Gt(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().ge_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Ge(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().lt_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Lt(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().le_field("user.info.details.birthdate".into()),
            DateValidation { required: true, operation: Some(Operation::Le(Operand::FieldPath("user.info.details.birthdate".into()))) }
        );
        assert_eq!(
            DateValidation::default().btwn_field("user.info.details.birthdate".into(), "user.info.details.deathdate".into()),
            DateValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::FieldPath("user.info.details.birthdate".into()),
                    Operand::FieldPath("user.info.details.deathdate".into())
                ))
            }
        );
    }
}
