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
            DateTimeValidation::default().eq(String::from("2026-08-12T08:10Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(String::from("2026-08-12T08:10Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().ne(String::from("2027-08-02T10:27Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(String::from("2027-08-02T10:27Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().gt(String::from("2028-07-22T19:41Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(String::from("2028-07-22T19:41Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().ge(String::from("2030-11-25T03:01Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(String::from("2030-11-25T03:01Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().lt(String::from("2031-11-14T00:00Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(String::from("2031-11-14T00:00Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().le(String::from("2033-03-30T01:01Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Le(Operand::Value(OperandValue::Str(String::from("2033-03-30T01:01Z")))))
            }
        );
        assert_eq!(
            DateTimeValidation::default().btwn(String::from("2031-11-14T00:00Z"), String::from("2033-03-30T01:01Z")),
            DateTimeValidation {
                required: true,
                operation: Some(Operation::Btwn(
                    Operand::Value(OperandValue::Str(String::from("2031-11-14T00:00Z"))),
                    Operand::Value(OperandValue::Str(String::from("2033-03-30T01:01Z")))
                ))
            }
        );
    }
}
