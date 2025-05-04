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
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::F64Validation;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::F64(-1.5)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::F64(-2.5)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::F64(-3.5)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::F64(-4.5)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::F64(-5.5)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::F64(-6.5)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::F64(-42.5)), Operand::Value(OperandValue::F64(42.5)));

    const FIELD: &str = "user.data.info.score";
    const FIELD_B: &str = "user.data.info.max_score";

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
    fn num_f_validation() {
        assert_eq!(F64Validation::default(), F64Validation { required: true, operation: None });
        assert_eq!(F64Validation::default().optional(), F64Validation { required: false, operation: None });
    }

    #[test]
    fn num_f_validation_operation_value() {
        assert_eq!(F64Validation::default().eq(-1.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(F64Validation::default().ne(-2.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(F64Validation::default().gt(-3.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(F64Validation::default().ge(-4.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(F64Validation::default().lt(-5.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(F64Validation::default().le(-6.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(F64Validation::default().btwn(-42.5, 42.5), F64Validation { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn num_f_validation_operation_field() {
        let validation_eq = F64Validation::default().eq_field(FIELD.into());
        let validation_ne = F64Validation::default().ne_field(FIELD.into());
        let validation_gt = F64Validation::default().gt_field(FIELD.into());
        let validation_ge = F64Validation::default().ge_field(FIELD.into());
        let validation_lt = F64Validation::default().lt_field(FIELD.into());
        let validation_le = F64Validation::default().le_field(FIELD.into());
        let validation_btwn = F64Validation::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, F64Validation { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, F64Validation { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, F64Validation { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, F64Validation { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, F64Validation { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, F64Validation { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, F64Validation { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
