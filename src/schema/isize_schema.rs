use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct ISizeSchema {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for ISizeSchema {
    fn default() -> Self {
        ISizeSchema { required: true, operation: None }
    }
}

impl ISizeSchema {
    pub fn optional(self) -> Self {
        ISizeSchema { required: false, ..self }
    }

    pub fn eq(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Eq(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ne(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Ne(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn gt(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Gt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ge(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Ge(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn lt(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Lt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn le(self, value: isize) -> Self {
        ISizeSchema { operation: Some(Operation::Le(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn btwn(self, value_a: isize, value_b: isize) -> Self {
        ISizeSchema {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::ISize(value_a)), Operand::Value(OperandValue::ISize(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        ISizeSchema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        ISizeSchema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::ISizeSchema;

    const OPERATION_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::ISize(-1)));
    const OPERATION_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::ISize(-2)));
    const OPERATION_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::ISize(-3)));
    const OPERATION_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::ISize(-4)));
    const OPERATION_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::ISize(-5)));
    const OPERATION_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::ISize(-6)));
    const OPERATION_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::ISize(-42)), Operand::Value(OperandValue::ISize(42)));

    const FIELD: &str = "user.data.info.score";
    const FIELD_B: &str = "user.data.info.max_score";

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
    fn isize_schema() {
        assert_eq!(ISizeSchema::default(), ISizeSchema { required: true, operation: None });
        assert_eq!(ISizeSchema::default().optional(), ISizeSchema { required: false, operation: None });
    }

    #[test]
    fn isize_schema_operation_value() {
        assert_eq!(ISizeSchema::default().eq(-1), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_EQ) });
        assert_eq!(ISizeSchema::default().ne(-2), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_NE) });
        assert_eq!(ISizeSchema::default().gt(-3), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_GT) });
        assert_eq!(ISizeSchema::default().ge(-4), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_GE) });
        assert_eq!(ISizeSchema::default().lt(-5), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_LT) });
        assert_eq!(ISizeSchema::default().le(-6), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_LE) });
        assert_eq!(ISizeSchema::default().btwn(-42, 42), ISizeSchema { required: true, operation: Some(OPERATION_VALUE_BTWN) });
    }

    #[test]
    fn isize_schema_operation_field() {
        let validation_eq = ISizeSchema::default().eq_field(FIELD.into());
        let validation_ne = ISizeSchema::default().ne_field(FIELD.into());
        let validation_gt = ISizeSchema::default().gt_field(FIELD.into());
        let validation_ge = ISizeSchema::default().ge_field(FIELD.into());
        let validation_lt = ISizeSchema::default().lt_field(FIELD.into());
        let validation_le = ISizeSchema::default().le_field(FIELD.into());
        let validation_btwn = ISizeSchema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_EQ.clone()) });
        assert_eq!(validation_ne, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_NE.clone()) });
        assert_eq!(validation_gt, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_GT.clone()) });
        assert_eq!(validation_ge, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_GE.clone()) });
        assert_eq!(validation_lt, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_LT.clone()) });
        assert_eq!(validation_le, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_LE.clone()) });
        assert_eq!(validation_btwn, ISizeSchema { required: true, operation: Some(OPERATION_FIELD_BTWN.clone()) });
    }
}
