use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct I64Schema {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for I64Schema {
    fn default() -> Self {
        I64Schema { required: true, operation: None }
    }
}

impl I64Schema {
    pub fn optional(self) -> Self {
        I64Schema { required: false, ..self }
    }

    pub fn eq(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Eq(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ne(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Ne(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn gt(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Gt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn ge(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Ge(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn lt(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Lt(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn le(self, value: i64) -> Self {
        I64Schema { operation: Some(Operation::Le(Operand::Value(OperandValue::I64(value)))), ..self }
    }

    pub fn btwn(self, value_a: i64, value_b: i64) -> Self {
        I64Schema { operation: Some(Operation::Btwn(Operand::Value(OperandValue::I64(value_a)), Operand::Value(OperandValue::I64(value_b)))), ..self }
    }

    pub fn eq_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        I64Schema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        I64Schema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::I64Schema;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::I64(-1)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::I64(-2)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::I64(-3)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::I64(-4)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::I64(-5)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::I64(-6)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::I64(-42)), Operand::Value(OperandValue::I64(42)));

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
    fn i64_schema() {
        assert_eq!(I64Schema::default(), I64Schema { required: true, operation: None });
        assert_eq!(I64Schema::default().optional(), I64Schema { required: false, operation: None });
    }

    #[test]
    fn i64_schema_operation_value() {
        assert_eq!(I64Schema::default().eq(-1), I64Schema { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(I64Schema::default().ne(-2), I64Schema { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(I64Schema::default().gt(-3), I64Schema { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(I64Schema::default().ge(-4), I64Schema { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(I64Schema::default().lt(-5), I64Schema { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(I64Schema::default().le(-6), I64Schema { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(I64Schema::default().btwn(-42, 42), I64Schema { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn i64_schema_operation_field() {
        let validation_eq = I64Schema::default().eq_field(FIELD.into());
        let validation_ne = I64Schema::default().ne_field(FIELD.into());
        let validation_gt = I64Schema::default().gt_field(FIELD.into());
        let validation_ge = I64Schema::default().ge_field(FIELD.into());
        let validation_lt = I64Schema::default().lt_field(FIELD.into());
        let validation_le = I64Schema::default().le_field(FIELD.into());
        let validation_btwn = I64Schema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, I64Schema { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, I64Schema { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, I64Schema { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, I64Schema { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, I64Schema { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, I64Schema { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, I64Schema { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
