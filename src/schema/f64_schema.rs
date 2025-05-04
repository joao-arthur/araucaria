use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct F64Schema {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for F64Schema {
    fn default() -> Self {
        F64Schema { required: true, operation: None }
    }
}

impl F64Schema {
    pub fn optional(self) -> Self {
        F64Schema { required: false, ..self }
    }

    pub fn eq(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Eq(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn ne(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Ne(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn gt(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Gt(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn ge(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Ge(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn lt(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Lt(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn le(self, value: f64) -> Self {
        F64Schema { operation: Some(Operation::Le(Operand::Value(OperandValue::F64(value)))), ..self }
    }

    pub fn btwn(self, value_a: f64, value_b: f64) -> Self {
        F64Schema {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::F64(value_a)), Operand::Value(OperandValue::F64(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        F64Schema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        F64Schema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::F64Schema;

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
    fn f64_schema() {
        assert_eq!(F64Schema::default(), F64Schema { required: true, operation: None });
        assert_eq!(F64Schema::default().optional(), F64Schema { required: false, operation: None });
    }

    #[test]
    fn f64_schema_operation_value() {
        assert_eq!(F64Schema::default().eq(-1.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(F64Schema::default().ne(-2.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(F64Schema::default().gt(-3.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(F64Schema::default().ge(-4.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(F64Schema::default().lt(-5.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(F64Schema::default().le(-6.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(F64Schema::default().btwn(-42.5, 42.5), F64Schema { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn f64_schema_operation_field() {
        let validation_eq = F64Schema::default().eq_field(FIELD.into());
        let validation_ne = F64Schema::default().ne_field(FIELD.into());
        let validation_gt = F64Schema::default().gt_field(FIELD.into());
        let validation_ge = F64Schema::default().ge_field(FIELD.into());
        let validation_lt = F64Schema::default().lt_field(FIELD.into());
        let validation_le = F64Schema::default().le_field(FIELD.into());
        let validation_btwn = F64Schema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, F64Schema { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, F64Schema { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, F64Schema { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, F64Schema { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, F64Schema { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, F64Schema { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, F64Schema { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
