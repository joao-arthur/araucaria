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
        DateValidation { operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))), ..self }
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
            DateValidation::default().eq(String::from("2026-08-12")),
            DateValidation { required: true, operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(String::from("2026-08-12"))))) }
        );
        assert_eq!(
            DateValidation::default().ne(String::from("2027-08-02")),
            DateValidation { required: true, operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(String::from("2027-08-02"))))) }
        );
        assert_eq!(
            DateValidation::default().gt(String::from("2028-07-22")),
            DateValidation { required: true, operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(String::from("2028-07-22"))))) }
        );
        assert_eq!(
            DateValidation::default().ge(String::from("2030-11-25")),
            DateValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(String::from("2030-11-25"))))) }
        );
        assert_eq!(
            DateValidation::default().lt(String::from("2031-11-14")),
            DateValidation { required: true, operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(String::from("2031-11-14"))))) }
        );
        assert_eq!(
            DateValidation::default().le(String::from("2033-03-30")),
            DateValidation { required: true, operation: Some(Operation::Le(Operand::Value(OperandValue::Str(String::from("2033-03-30"))))) }
        );
        assert_eq!(
            DateValidation::default().btwn(String::from("2031-11-14"), String::from("2033-03-30")),
            DateValidation { required: true, operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(String::from("2031-11-14"))), Operand::Value(OperandValue::Str(String::from("2033-03-30"))))) }
        );
    }
}
