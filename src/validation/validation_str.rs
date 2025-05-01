use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct StrValidation {
    pub required: bool,
    pub operation: Option<Operation>,
    pub bytes_len: Option<Operation>,
    pub chars_len: Option<Operation>,
    pub graphemes_len: Option<Operation>,
    pub lowercase_len: Option<Operation>,
    pub uppercase_len: Option<Operation>,
    pub numbers_len: Option<Operation>,
    pub symbols_len: Option<Operation>,
}

impl Default for StrValidation {
    fn default() -> Self {
        StrValidation {
            required: true,
            operation: None,
            bytes_len: None,
            chars_len: None,
            graphemes_len: None,
            lowercase_len: None,
            uppercase_len: None,
            numbers_len: None,
            symbols_len: None,
        }
    }
}

impl StrValidation {
    pub fn optional(self) -> Self {
        StrValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn le(self, value: String) -> Self {
        StrValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        StrValidation {
            operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))),
            ..self
        }
    }

    pub fn bytes_len_eq(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_ne(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_gt(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_ge(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_lt(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_le(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            bytes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn chars_len_eq(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_ne(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_gt(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_ge(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_lt(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_le(self, len: usize) -> Self {
        StrValidation { chars_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            chars_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn graphemes_len_eq(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_ne(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_gt(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_ge(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_lt(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_le(self, len: usize) -> Self {
        StrValidation { graphemes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            graphemes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn lowercase_len_eq(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_ne(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_gt(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_ge(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_lt(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_le(self, len: usize) -> Self {
        StrValidation { lowercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            lowercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn uppercase_len_eq(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_ne(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_gt(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_ge(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_lt(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_le(self, len: usize) -> Self {
        StrValidation { uppercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            uppercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn numbers_len_eq(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_ne(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_gt(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_ge(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_lt(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_le(self, len: usize) -> Self {
        StrValidation { numbers_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            numbers_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn symbols_len_eq(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_ne(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_gt(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_ge(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_lt(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_le(self, len: usize) -> Self {
        StrValidation { symbols_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation {
            symbols_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        StrValidation { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn bytes_len_eq_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_ne_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_gt_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_ge_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_lt_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_le_field(self, field: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { bytes_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn chars_len_eq_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_ne_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_gt_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_ge_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_lt_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_le_field(self, field: String) -> Self {
        StrValidation { chars_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { chars_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn graphemes_len_eq_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_ne_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_gt_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_ge_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_lt_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_le_field(self, field: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { graphemes_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn lowercase_len_eq_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_ne_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_gt_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_ge_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_lt_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_le_field(self, field: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { lowercase_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn uppercase_len_eq_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_ne_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_gt_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_ge_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_lt_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_le_field(self, field: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { uppercase_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn numbers_len_eq_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_ne_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_gt_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_ge_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_lt_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_le_field(self, field: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { numbers_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn symbols_len_eq_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_ne_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_gt_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_ge_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_lt_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_le_field(self, field: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrValidation { symbols_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::StrValidation;

    const FIELD: &str = "foo.bar";
    const FIELD_B: &str = "bar.foo";

    const OP_VALUE_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::Value(OperandValue::from("Avalon"))));
    const OP_VALUE_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::Value(OperandValue::from("Avalon")), Operand::Value(OperandValue::from("Mu"))));
    const OP_BYTES_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(11)));
    const OP_BYTES_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(12)));
    const OP_BYTES_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(13)));
    const OP_BYTES_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(14)));
    const OP_BYTES_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(15)));
    const OP_BYTES_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(16)));
    const OP_BYTES_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(17)), Operand::Value(OperandValue::USize(18)));
    const OP_CHARS_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(21)));
    const OP_CHARS_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(22)));
    const OP_CHARS_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(23)));
    const OP_CHARS_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(24)));
    const OP_CHARS_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(25)));
    const OP_CHARS_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(26)));
    const OP_CHARS_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(27)), Operand::Value(OperandValue::USize(28)));
    const OP_GRAPHEMES_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(31)));
    const OP_GRAPHEMES_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(32)));
    const OP_GRAPHEMES_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(33)));
    const OP_GRAPHEMES_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(34)));
    const OP_GRAPHEMES_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(35)));
    const OP_GRAPHEMES_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(36)));
    const OP_GRAPHEMES_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(37)), Operand::Value(OperandValue::USize(38)));
    const OP_LOWERCASE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(41)));
    const OP_LOWERCASE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(42)));
    const OP_LOWERCASE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(43)));
    const OP_LOWERCASE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(44)));
    const OP_LOWERCASE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(45)));
    const OP_LOWERCASE_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(46)));
    const OP_LOWERCASE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(47)), Operand::Value(OperandValue::USize(48)));
    const OP_UPPERCASE_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(51)));
    const OP_UPPERCASE_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(52)));
    const OP_UPPERCASE_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(53)));
    const OP_UPPERCASE_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(54)));
    const OP_UPPERCASE_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(55)));
    const OP_UPPERCASE_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(56)));
    const OP_UPPERCASE_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(57)), Operand::Value(OperandValue::USize(58)));
    const OP_NUMBERS_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(61)));
    const OP_NUMBERS_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(62)));
    const OP_NUMBERS_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(63)));
    const OP_NUMBERS_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(64)));
    const OP_NUMBERS_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(65)));
    const OP_NUMBERS_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(66)));
    const OP_NUMBERS_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(67)), Operand::Value(OperandValue::USize(68)));
    const OP_SYMBOLS_EQ: Operation = Operation::Eq(Operand::Value(OperandValue::USize(71)));
    const OP_SYMBOLS_NE: Operation = Operation::Ne(Operand::Value(OperandValue::USize(72)));
    const OP_SYMBOLS_GT: Operation = Operation::Gt(Operand::Value(OperandValue::USize(73)));
    const OP_SYMBOLS_GE: Operation = Operation::Ge(Operand::Value(OperandValue::USize(74)));
    const OP_SYMBOLS_LT: Operation = Operation::Lt(Operand::Value(OperandValue::USize(75)));
    const OP_SYMBOLS_LE: Operation = Operation::Le(Operand::Value(OperandValue::USize(76)));
    const OP_SYMBOLS_BTWN: Operation = Operation::Btwn(Operand::Value(OperandValue::USize(77)), Operand::Value(OperandValue::USize(78)));
    const OP_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(Operand::FieldPath(FIELD.into())));
    const OP_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(Operand::FieldPath(FIELD.into()), Operand::FieldPath(FIELD_B.into())));

    #[test]
    fn str_validation() {
        assert_eq!(StrValidation::default(),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None
            });
        assert_eq!(StrValidation::default().optional(), StrValidation { required: false, ..Default::default() });
        assert_eq!(StrValidation::default().eq("Avalon".into()), StrValidation { operation: Some(OP_VALUE_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().ne("Avalon".into()), StrValidation { operation: Some(OP_VALUE_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().gt("Avalon".into()), StrValidation { operation: Some(OP_VALUE_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().ge("Avalon".into()), StrValidation { operation: Some(OP_VALUE_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lt("Avalon".into()), StrValidation { operation: Some(OP_VALUE_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().le("Avalon".into()), StrValidation { operation: Some(OP_VALUE_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().btwn("Avalon".into(), "Mu".into()), StrValidation { operation: Some(OP_VALUE_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_eq(11), StrValidation { bytes_len: Some(OP_BYTES_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_ne(12), StrValidation { bytes_len: Some(OP_BYTES_NE), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_gt(13), StrValidation { bytes_len: Some(OP_BYTES_GT), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_ge(14), StrValidation { bytes_len: Some(OP_BYTES_GE), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_lt(15), StrValidation { bytes_len: Some(OP_BYTES_LT), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_le(16), StrValidation { bytes_len: Some(OP_BYTES_LE), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_btwn(17, 18), StrValidation { bytes_len: Some(OP_BYTES_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_eq(21), StrValidation { chars_len: Some(OP_CHARS_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_ne(22), StrValidation { chars_len: Some(OP_CHARS_NE), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_gt(23), StrValidation { chars_len: Some(OP_CHARS_GT), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_ge(24), StrValidation { chars_len: Some(OP_CHARS_GE), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_lt(25), StrValidation { chars_len: Some(OP_CHARS_LT), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_le(26), StrValidation { chars_len: Some(OP_CHARS_LE), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_btwn(27, 28), StrValidation { chars_len: Some(OP_CHARS_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_eq(31), StrValidation { graphemes_len: Some(OP_GRAPHEMES_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_ne(32), StrValidation { graphemes_len: Some(OP_GRAPHEMES_NE), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_gt(33), StrValidation { graphemes_len: Some(OP_GRAPHEMES_GT), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_ge(34), StrValidation { graphemes_len: Some(OP_GRAPHEMES_GE), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_lt(35), StrValidation { graphemes_len: Some(OP_GRAPHEMES_LT), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_le(36), StrValidation { graphemes_len: Some(OP_GRAPHEMES_LE), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_btwn(37, 38), StrValidation { graphemes_len: Some(OP_GRAPHEMES_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_eq(41), StrValidation { lowercase_len: Some(OP_LOWERCASE_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_ne(42), StrValidation { lowercase_len: Some(OP_LOWERCASE_NE), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_gt(43), StrValidation { lowercase_len: Some(OP_LOWERCASE_GT), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_ge(44), StrValidation { lowercase_len: Some(OP_LOWERCASE_GE), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_lt(45), StrValidation { lowercase_len: Some(OP_LOWERCASE_LT), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_le(46), StrValidation { lowercase_len: Some(OP_LOWERCASE_LE), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_btwn(47, 48), StrValidation { lowercase_len: Some(OP_LOWERCASE_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_eq(51), StrValidation { uppercase_len: Some(OP_UPPERCASE_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_ne(52), StrValidation { uppercase_len: Some(OP_UPPERCASE_NE), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_gt(53), StrValidation { uppercase_len: Some(OP_UPPERCASE_GT), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_ge(54), StrValidation { uppercase_len: Some(OP_UPPERCASE_GE), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_lt(55), StrValidation { uppercase_len: Some(OP_UPPERCASE_LT), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_le(56), StrValidation { uppercase_len: Some(OP_UPPERCASE_LE), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_btwn(57, 58), StrValidation { uppercase_len: Some(OP_UPPERCASE_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_eq(61), StrValidation { numbers_len: Some(OP_NUMBERS_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_ne(62), StrValidation { numbers_len: Some(OP_NUMBERS_NE), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_gt(63), StrValidation { numbers_len: Some(OP_NUMBERS_GT), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_ge(64), StrValidation { numbers_len: Some(OP_NUMBERS_GE), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_lt(65), StrValidation { numbers_len: Some(OP_NUMBERS_LT), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_le(66), StrValidation { numbers_len: Some(OP_NUMBERS_LE), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_btwn(67, 68), StrValidation { numbers_len: Some(OP_NUMBERS_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_eq(71), StrValidation { symbols_len: Some(OP_SYMBOLS_EQ), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_ne(72), StrValidation { symbols_len: Some(OP_SYMBOLS_NE), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_gt(73), StrValidation { symbols_len: Some(OP_SYMBOLS_GT), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_ge(74), StrValidation { symbols_len: Some(OP_SYMBOLS_GE), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_lt(75), StrValidation { symbols_len: Some(OP_SYMBOLS_LT), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_le(76), StrValidation { symbols_len: Some(OP_SYMBOLS_LE), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_btwn(77, 78), StrValidation { symbols_len: Some(OP_SYMBOLS_BTWN), ..Default::default() });
        assert_eq!(StrValidation::default().eq_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().ne_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().gt_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().ge_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lt_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().le_field(FIELD.into()), StrValidation { operation: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { operation: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_eq_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_ne_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_gt_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_ge_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_lt_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_le_field(FIELD.into()), StrValidation { bytes_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().bytes_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { bytes_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_eq_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_ne_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_gt_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_ge_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_lt_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_le_field(FIELD.into()), StrValidation { chars_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().chars_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { chars_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_eq_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_ne_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_gt_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_ge_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_lt_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_le_field(FIELD.into()), StrValidation { graphemes_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().graphemes_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { graphemes_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_eq_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_ne_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_gt_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_ge_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_lt_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_le_field(FIELD.into()), StrValidation { lowercase_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().lowercase_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { lowercase_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_eq_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_ne_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_gt_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_ge_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_lt_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_le_field(FIELD.into()), StrValidation { uppercase_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().uppercase_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { uppercase_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_eq_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_ne_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_gt_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_ge_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_lt_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_le_field(FIELD.into()), StrValidation { numbers_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().numbers_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { numbers_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_eq_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_ne_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_gt_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_ge_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_lt_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_le_field(FIELD.into()), StrValidation { symbols_len: Some(OP_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(StrValidation::default().symbols_len_btwn_field(FIELD.into(), FIELD_B.into()), StrValidation { symbols_len: Some(OP_FIELD_BTWN.clone()), ..Default::default() });
    }
}
