use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct BoolSchema {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for BoolSchema {
    fn default() -> Self {
        BoolSchema { required: true, operation: None }
    }
}

impl BoolSchema {
    pub fn optional(self) -> Self {
        BoolSchema { required: false, ..self }
    }

    pub fn eq(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Eq(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn ne(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Ne(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn gt(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Gt(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn ge(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Ge(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn lt(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Lt(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn le(self, value: bool) -> Self {
        BoolSchema { operation: Some(Operation::Le(Operand::Value(OperandValue::Bool(value)))), ..self }
    }

    pub fn btwn(self, value_a: bool, value_b: bool) -> Self {
        BoolSchema {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Bool(value_a)), Operand::Value(OperandValue::Bool(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        BoolSchema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        BoolSchema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::BoolSchema;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::Bool(false)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)));

    const FIELD: &str = "user.info.details.is_alive";
    const FIELD_B: &str = "user.info.details.is_alive";

    static OPERAND_FIELD: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD.into()));
    static OPERAND_FIELD_B: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_B.into()));

    static OPERATION_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_FIELD.clone()));
    static OPERATION_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_FIELD.clone(), OPERAND_FIELD_B.clone()));

    #[test]
    fn bool_schema() {
        assert_eq!(BoolSchema::default(), BoolSchema { required: true, operation: None });
        assert_eq!(BoolSchema::default().optional(), BoolSchema { required: false, operation: None });
    }

    #[test]
    fn bool_schema_operation_value() {
        assert_eq!(BoolSchema::default().eq(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(BoolSchema::default().ne(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(BoolSchema::default().gt(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(BoolSchema::default().ge(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(BoolSchema::default().lt(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(BoolSchema::default().le(false), BoolSchema { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(BoolSchema::default().btwn(false, true), BoolSchema { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn bool_schema_operation_field() {
        let validation_eq = BoolSchema::default().eq_field(FIELD.into());
        let validation_ne = BoolSchema::default().ne_field(FIELD.into());
        let validation_gt = BoolSchema::default().gt_field(FIELD.into());
        let validation_ge = BoolSchema::default().ge_field(FIELD.into());
        let validation_lt = BoolSchema::default().lt_field(FIELD.into());
        let validation_le = BoolSchema::default().le_field(FIELD.into());
        let validation_btwn = BoolSchema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, BoolSchema { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, BoolSchema { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, BoolSchema { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, BoolSchema { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, BoolSchema { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, BoolSchema { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, BoolSchema { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
