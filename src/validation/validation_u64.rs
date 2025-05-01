use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct U64Validation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for U64Validation {
    fn default() -> Self {
        U64Validation { required: true, operation: None }
    }
}

impl U64Validation {
    pub fn optional(self) -> Self {
        U64Validation { required: false, ..self }
    }

    pub fn eq(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Eq(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ne(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Ne(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn gt(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Gt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ge(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Ge(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn lt(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Lt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn le(self, value: u64) -> Self {
        U64Validation { operation: Some(Operation::Le(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn btwn(self, value_a: u64, value_b: u64) -> Self {
        U64Validation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::U64(value_a)), Operand::Value(OperandValue::U64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        U64Validation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        U64Validation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::U64Validation;

    const FIELD: &str = "user.personal.info.height";
    const FIELD_B: &str = "user.personal.info.max_height";

    const OP_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::U64(1)));
    const OP_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::U64(2)));
    const OP_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::U64(3)));
    const OP_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::U64(4)));
    const OP_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::U64(5)));
    const OP_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::U64(6)));
    const OP_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::U64(1)), Operand::Value(OperandValue::U64(9)));
    const OP_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::FieldPath(FIELD.into()), Operand::FieldPath(FIELD_B.into())));

    #[test]
    fn u64_validation() {
        assert_eq!(U64Validation::default(), U64Validation { required: true, operation: None });
        assert_eq!(U64Validation::default().optional(), U64Validation { required: false, operation: None });
        assert_eq!(U64Validation::default().eq(1), U64Validation { required: true, operation: Some(OP_VALUE_EQ) });
        assert_eq!(U64Validation::default().ne(2), U64Validation { required: true, operation: Some(OP_VALUE_NE) });
        assert_eq!(U64Validation::default().gt(3), U64Validation { required: true, operation: Some(OP_VALUE_GT) });
        assert_eq!(U64Validation::default().ge(4), U64Validation { required: true, operation: Some(OP_VALUE_GE) });
        assert_eq!(U64Validation::default().lt(5), U64Validation { required: true, operation: Some(OP_VALUE_LT) });
        assert_eq!(U64Validation::default().le(6), U64Validation { required: true, operation: Some(OP_VALUE_LE) });
        assert_eq!(U64Validation::default().btwn(1, 9), U64Validation { required: true, operation: Some(OP_VALUE_BTWN) });
        assert_eq!(U64Validation::default().eq_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_EQ.clone()) });
        assert_eq!(U64Validation::default().ne_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_NE.clone()) });
        assert_eq!(U64Validation::default().gt_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_GT.clone()) });
        assert_eq!(U64Validation::default().ge_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_GE.clone()) });
        assert_eq!(U64Validation::default().lt_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_LT.clone()) });
        assert_eq!(U64Validation::default().le_field(FIELD.into()), U64Validation { required: true, operation: Some(OP_FIELD_LE.clone()) });
        assert_eq!(U64Validation::default().btwn_field(FIELD.into(), FIELD_B.into()), U64Validation { required: true, operation: Some(OP_FIELD_BTWN.clone()) });
    }
}
