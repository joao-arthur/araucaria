use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct ISizeValidation {
    pub required: bool,
    pub operation: Option<Operation>,
}

impl Default for ISizeValidation {
    fn default() -> Self {
        ISizeValidation { required: true, operation: None }
    }
}

impl ISizeValidation {
    pub fn optional(self) -> Self {
        ISizeValidation { required: false, ..self }
    }

    pub fn eq(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ne(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn gt(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn ge(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn lt(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn le(self, value: isize) -> Self {
        ISizeValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::ISize(value)))), ..self }
    }

    pub fn btwn(self, value_a: isize, value_b: isize) -> Self {
        ISizeValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::ISize(value_a)), Operand::Value(OperandValue::ISize(value_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        ISizeValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        ISizeValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::ISizeValidation;

    const FIELD: &str = "user.data.info.score";
    const FIELD_B: &str = "user.data.info.max_score";

    const OP_VALUE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::ISize(-1)));
    const OP_VALUE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::ISize(-2)));
    const OP_VALUE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::ISize(-3)));
    const OP_VALUE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::ISize(-4)));
    const OP_VALUE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::ISize(-5)));
    const OP_VALUE_LE: Operation = Operation::Le(Operand::Value(OperandValue::ISize(-6)));
    const OP_VALUE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::ISize(-42)), Operand::Value(OperandValue::ISize(42)));
    const OP_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::FieldPath(FIELD.into()), Operand::FieldPath(FIELD_B.into())));

    #[test]
    fn isize_validation() {
        assert_eq!(ISizeValidation::default(), ISizeValidation { required: true, operation: None });
        assert_eq!(ISizeValidation::default().optional(), ISizeValidation { required: false, operation: None });
        assert_eq!(ISizeValidation::default().eq(-1), ISizeValidation { required: true, operation: Some(OP_VALUE_EQ) });
        assert_eq!(ISizeValidation::default().ne(-2), ISizeValidation { required: true, operation: Some(OP_VALUE_NE) });
        assert_eq!(ISizeValidation::default().gt(-3), ISizeValidation { required: true, operation: Some(OP_VALUE_GT) });
        assert_eq!(ISizeValidation::default().ge(-4), ISizeValidation { required: true, operation: Some(OP_VALUE_GE) });
        assert_eq!(ISizeValidation::default().lt(-5), ISizeValidation { required: true, operation: Some(OP_VALUE_LT) });
        assert_eq!(ISizeValidation::default().le(-6), ISizeValidation { required: true, operation: Some(OP_VALUE_LE) });
        assert_eq!(ISizeValidation::default().btwn(-42, 42), ISizeValidation { required: true, operation: Some(OP_VALUE_BTWN) });
        assert_eq!(ISizeValidation::default().eq_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_EQ.clone()) });
        assert_eq!(ISizeValidation::default().ne_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_NE.clone()) });
        assert_eq!(ISizeValidation::default().gt_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_GT.clone()) });
        assert_eq!(ISizeValidation::default().ge_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_GE.clone()) });
        assert_eq!(ISizeValidation::default().lt_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_LT.clone()) });
        assert_eq!(ISizeValidation::default().le_field(FIELD.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_LE.clone()) });
        assert_eq!(ISizeValidation::default().btwn_field(FIELD.into(), FIELD_B.into()), ISizeValidation { required: true, operation: Some(OP_FIELD_BTWN.clone()) });
    }
}
