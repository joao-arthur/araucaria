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

    pub fn unix_epoch(self) -> Self {
        DateValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str("1970-01-01".into())))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::DateValidation;

    const VALUE: &str = "2027-08-02";
    const VALUE_B: &str = "2030-11-25";

    const OPERAND: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from(VALUE)));
    const OPERAND_B: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from(VALUE_B)));

    const OPERATION_VALUE_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND.clone()));
    const OPERATION_VALUE_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND.clone()));
    const OPERATION_VALUE_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND.clone()));
    const OPERATION_VALUE_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND.clone()));
    const OPERATION_VALUE_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND.clone()));
    const OPERATION_VALUE_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND.clone()));
    const OPERATION_VALUE_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND.clone(), OPERAND_B.clone()));

    const FIELD: &str = "user.info.details.birthdate";
    const FIELD_B: &str = "user.info.details.deathdate";

    const OPERAND_FIELD: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD.into()));
    const OPERAND_FIELD_B: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_B.into()));

    const OPERATION_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_FIELD.clone(), OPERAND_FIELD_B.clone()));

    #[test]
    fn date_validation() {
        assert_eq!(DateValidation::default(), DateValidation { required: true, operation: None });
        assert_eq!(DateValidation::default().optional(), DateValidation { required: false, operation: None });
    }

    #[test]
    fn date_validation_operation_value() {
        let validation_eq = DateValidation::default().eq(VALUE.into());
        let validation_ne = DateValidation::default().ne(VALUE.into());
        let validation_gt = DateValidation::default().gt(VALUE.into());
        let validation_ge = DateValidation::default().ge(VALUE.into());
        let validation_lt = DateValidation::default().lt(VALUE.into());
        let validation_le = DateValidation::default().le(VALUE.into());
        let validation_btwn = DateValidation::default().btwn(VALUE.into(), VALUE_B.into());

        assert_eq!(validation_eq, DateValidation { required: true, operation: Some(OPERATION_VALUE_EQ.clone()) });
        assert_eq!(validation_ne, DateValidation { required: true, operation: Some(OPERATION_VALUE_NE.clone()) });
        assert_eq!(validation_gt, DateValidation { required: true, operation: Some(OPERATION_VALUE_GT.clone()) });
        assert_eq!(validation_ge, DateValidation { required: true, operation: Some(OPERATION_VALUE_GE.clone()) });
        assert_eq!(validation_lt, DateValidation { required: true, operation: Some(OPERATION_VALUE_LT.clone()) });
        assert_eq!(validation_le, DateValidation { required: true, operation: Some(OPERATION_VALUE_LE.clone()) });
        assert_eq!(validation_btwn, DateValidation { required: true, operation: Some(OPERATION_VALUE_BTWN.clone()) });
    }

    #[test]
    fn date_validation_operation_field() {
        let validation_eq = DateValidation::default().eq_field(FIELD.into());
        let validation_ne = DateValidation::default().ne_field(FIELD.into());
        let validation_gt = DateValidation::default().gt_field(FIELD.into());
        let validation_ge = DateValidation::default().ge_field(FIELD.into());
        let validation_lt = DateValidation::default().lt_field(FIELD.into());
        let validation_le = DateValidation::default().le_field(FIELD.into());
        let validation_btwn = DateValidation::default().btwn_field(FIELD.into(), FIELD_B.into());

        assert_eq!(validation_eq, DateValidation { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, DateValidation { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, DateValidation { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, DateValidation { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, DateValidation { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, DateValidation { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, DateValidation { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }

    #[test]
    fn date_validation_unix_epoch() {
        assert_eq!(
            DateValidation::default().unix_epoch(),
            DateValidation { required: true, operation: Some(Operation::Ge(Operand::Value(OperandValue::Str("1970-01-01".into())))) }
        );
    }
}
