use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct U64Schema {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for U64Schema {
    fn default() -> Self {
        U64Schema { required: true, operation: None }
    }
}

impl U64Schema {
    pub fn optional(self) -> Self {
        U64Schema { required: false, ..self }
    }

    pub fn eq(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Eq(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ne(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Ne(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn gt(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Gt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn ge(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Ge(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn lt(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Lt(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn le(self, value: u64) -> Self {
        U64Schema { operation: Some(Operation::Le(Operand::Value(OperandValue::U64(value)))), ..self }
    }

    pub fn btwn(self, value_a: u64, value_b: u64) -> Self {
        U64Schema {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::U64(value_a)), Operand::Value(OperandValue::U64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        U64Schema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        U64Schema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::U64Schema;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::U64(1)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::U64(2)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::U64(3)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::U64(4)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::U64(5)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::U64(6)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::U64(1)), Operand::Value(OperandValue::U64(9)));

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
    fn u64_schema() {
        assert_eq!(U64Schema::default(), U64Schema { required: true, operation: None });
        assert_eq!(U64Schema::default().optional(), U64Schema { required: false, operation: None });
    }

    #[test]
    fn u64_schema_operation_value() {
        assert_eq!(U64Schema::default().eq(1), U64Schema { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(U64Schema::default().ne(2), U64Schema { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(U64Schema::default().gt(3), U64Schema { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(U64Schema::default().ge(4), U64Schema { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(U64Schema::default().lt(5), U64Schema { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(U64Schema::default().le(6), U64Schema { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(U64Schema::default().btwn(1, 9), U64Schema { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn u64_schema_operation_field() {
        let validation_eq = U64Schema::default().eq_field(FIELD.into());
        let validation_ne = U64Schema::default().ne_field(FIELD.into());
        let validation_gt = U64Schema::default().gt_field(FIELD.into());
        let validation_ge = U64Schema::default().ge_field(FIELD.into());
        let validation_lt = U64Schema::default().lt_field(FIELD.into());
        let validation_le = U64Schema::default().le_field(FIELD.into());
        let validation_btwn = U64Schema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, U64Schema { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, U64Schema { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, U64Schema { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, U64Schema { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, U64Schema { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, U64Schema { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, U64Schema { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
