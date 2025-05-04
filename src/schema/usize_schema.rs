use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct USizeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for USizeValidation {
    fn default() -> Self {
        USizeValidation { required: true, operation: None }
    }
}

impl USizeValidation {
    pub fn optional(self) -> Self {
        USizeValidation { required: false, ..self }
    }

    pub fn eq(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn ne(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn gt(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn ge(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn lt(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn le(self, value: usize) -> Self {
        USizeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::USize(value)))), ..self }
    }

    pub fn btwn(self, value_a: usize, value_b: usize) -> Self {
        USizeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::USize(value_a)), Operand::Value(OperandValue::USize(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        USizeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        USizeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::USizeValidation;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(1)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(2)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(3)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(4)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(5)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(6)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(1)), Operand::Value(OperandValue::USize(9)));

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
    fn usize_validation() {
        assert_eq!(USizeValidation::default(), USizeValidation { required: true, operation: None });
        assert_eq!(USizeValidation::default().optional(), USizeValidation { required: false, operation: None });
    }

    #[test]
    fn usize_validation_operation_value() {
        assert_eq!(USizeValidation::default().eq(1), USizeValidation { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(USizeValidation::default().ne(2), USizeValidation { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(USizeValidation::default().gt(3), USizeValidation { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(USizeValidation::default().ge(4), USizeValidation { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(USizeValidation::default().lt(5), USizeValidation { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(USizeValidation::default().le(6), USizeValidation { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(USizeValidation::default().btwn(1, 9), USizeValidation { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn usize_validation_operation_field() {
        let validation_eq = USizeValidation::default().eq_field(FIELD.into());
        let validation_ne = USizeValidation::default().ne_field(FIELD.into());
        let validation_gt = USizeValidation::default().gt_field(FIELD.into());
        let validation_ge = USizeValidation::default().ge_field(FIELD.into());
        let validation_lt = USizeValidation::default().lt_field(FIELD.into());
        let validation_le = USizeValidation::default().le_field(FIELD.into());
        let validation_btwn = USizeValidation::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, USizeValidation { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, USizeValidation { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, USizeValidation { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, USizeValidation { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, USizeValidation { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, USizeValidation { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, USizeValidation { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
