use crate::operation::{Operand, OperandValue, Operation};

#[derive(Debug, PartialEq, Clone)]
pub struct StrSchema {
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

impl Default for StrSchema {
    fn default() -> Self {
        StrSchema {
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

impl StrSchema {
    pub fn optional(self) -> Self {
        StrSchema { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Eq(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Ne(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Gt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Ge(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Lt(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn le(self, value: String) -> Self {
        StrSchema { operation: Some(Operation::Le(Operand::Value(OperandValue::Str(value)))), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        StrSchema { operation: Some(Operation::Btwn(Operand::Value(OperandValue::Str(value_a)), Operand::Value(OperandValue::Str(value_b)))), ..self }
    }

    pub fn bytes_len_eq(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_ne(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_gt(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_ge(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_lt(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_le(self, len: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn bytes_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema { bytes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))), ..self }
    }

    pub fn chars_len_eq(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_ne(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_gt(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_ge(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_lt(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_le(self, len: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn chars_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema { chars_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))), ..self }
    }

    pub fn graphemes_len_eq(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_ne(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_gt(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_ge(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_lt(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_le(self, len: usize) -> Self {
        StrSchema { graphemes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn graphemes_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema {
            graphemes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn lowercase_len_eq(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_ne(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_gt(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_ge(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_lt(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_le(self, len: usize) -> Self {
        StrSchema { lowercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn lowercase_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema {
            lowercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn uppercase_len_eq(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_ne(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_gt(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_ge(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_lt(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_le(self, len: usize) -> Self {
        StrSchema { uppercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn uppercase_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema {
            uppercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn numbers_len_eq(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_ne(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_gt(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_ge(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_lt(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_le(self, len: usize) -> Self {
        StrSchema { numbers_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn numbers_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema {
            numbers_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn symbols_len_eq(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_ne(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_gt(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_ge(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_lt(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_le(self, len: usize) -> Self {
        StrSchema { symbols_len: Some(Operation::Le(Operand::Value(OperandValue::USize(len)))), ..self }
    }

    pub fn symbols_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrSchema {
            symbols_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(len_a)), Operand::Value(OperandValue::USize(len_b)))),
            ..self
        }
    }

    pub fn eq_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn ne_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn gt_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn ge_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lt_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn le_field(self, field: String) -> Self {
        StrSchema { operation: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { operation: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn bytes_len_eq_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_ne_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_gt_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_ge_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_lt_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_le_field(self, field: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn bytes_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { bytes_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn chars_len_eq_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_ne_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_gt_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_ge_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_lt_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_le_field(self, field: String) -> Self {
        StrSchema { chars_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn chars_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { chars_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn graphemes_len_eq_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_ne_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_gt_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_ge_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_lt_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_le_field(self, field: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn graphemes_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { graphemes_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn lowercase_len_eq_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_ne_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_gt_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_ge_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_lt_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_le_field(self, field: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn lowercase_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { lowercase_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn uppercase_len_eq_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_ne_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_gt_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_ge_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_lt_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_le_field(self, field: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn uppercase_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { uppercase_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn numbers_len_eq_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_ne_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_gt_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_ge_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_lt_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_le_field(self, field: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn numbers_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { numbers_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }

    pub fn symbols_len_eq_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Eq(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_ne_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Ne(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_gt_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Gt(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_ge_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Ge(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_lt_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Lt(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_le_field(self, field: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Le(Operand::FieldPath(field))), ..self }
    }

    pub fn symbols_len_btwn_field(self, field_a: String, field_b: String) -> Self {
        StrSchema { symbols_len: Some(Operation::Btwn(Operand::FieldPath(field_a), Operand::FieldPath(field_b))), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::StrSchema;

    const VALUE: &str = "Avalon";
    const VALUE_B: &str = "Mu";

    const OPERAND_VALUE: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from(VALUE)));
    const OPERAND_VALUE_B: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from(VALUE_B)));

    const OPERATION_VALUE_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_VALUE.clone()));
    const OPERATION_VALUE_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_VALUE.clone(), OPERAND_VALUE_B.clone()));

    const FIELD: &str = "user.info.details.name.0";
    const FIELD_B: &str = "user.info.details.name.1";

    const OPERAND_FIELD: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD.into()));
    const OPERAND_FIELD_B: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_B.into()));

    const OPERATION_FIELD_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_FIELD.clone()));
    const OPERATION_FIELD_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_FIELD.clone(), OPERAND_FIELD_B.clone()));

    const VALUE_LEN: usize = 14;
    const VALUE_LEN_B: usize = 73;

    const OPERAND_VALUE_LEN: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::USize(VALUE_LEN)));
    const OPERAND_VALUE_LEN_B: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::USize(VALUE_LEN_B)));

    const OPERATION_VALUE_LEN_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_VALUE_LEN.clone()));
    const OPERATION_VALUE_LEN_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_VALUE_LEN.clone(), OPERAND_VALUE_LEN_B.clone()));

    const FIELD_LEN: &str = "user.info.clothes.shoesize.min";
    const FIELD_LEN_B: &str = "user.info.clothes.shoesize.min";

    const OPERAND_FIELD_LEN: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN.into()));
    const OPERAND_FIELD_LEN_B: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN_B.into()));

    const OPERATION_FIELD_LEN_EQ: LazyLock<Operation> = LazyLock::new(|| Operation::Eq(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_NE: LazyLock<Operation> = LazyLock::new(|| Operation::Ne(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_GT: LazyLock<Operation> = LazyLock::new(|| Operation::Gt(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_GE: LazyLock<Operation> = LazyLock::new(|| Operation::Ge(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_FIELD_LEN.clone()));
    const OPERATION_FIELD_LEN_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_FIELD_LEN.clone(), OPERAND_FIELD_LEN_B.clone()));

    #[test]
    fn str_schema() {
        assert_eq!(
            StrSchema::default(),
            StrSchema {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None
            }
        );
        assert_eq!(
            StrSchema::default().optional(),
            StrSchema {
                required: false,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None
            }
        );
    }

    #[test]
    fn str_schema_operation_value() {
        let validation_eq = StrSchema::default().eq(VALUE.into());
        let validation_ne = StrSchema::default().ne(VALUE.into());
        let validation_gt = StrSchema::default().gt(VALUE.into());
        let validation_ge = StrSchema::default().ge(VALUE.into());
        let validation_lt = StrSchema::default().lt(VALUE.into());
        let validation_le = StrSchema::default().le(VALUE.into());
        let validation_btwn = StrSchema::default().btwn(VALUE.into(), VALUE_B.into());
        assert_eq!(validation_eq, StrSchema { operation: Some(OPERATION_VALUE_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { operation: Some(OPERATION_VALUE_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { operation: Some(OPERATION_VALUE_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { operation: Some(OPERATION_VALUE_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { operation: Some(OPERATION_VALUE_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { operation: Some(OPERATION_VALUE_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { operation: Some(OPERATION_VALUE_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_operation_field() {
        let validation_eq = StrSchema::default().eq_field(FIELD.into());
        let validation_ne = StrSchema::default().ne_field(FIELD.into());
        let validation_gt = StrSchema::default().gt_field(FIELD.into());
        let validation_ge = StrSchema::default().ge_field(FIELD.into());
        let validation_lt = StrSchema::default().lt_field(FIELD.into());
        let validation_le = StrSchema::default().le_field(FIELD.into());
        let validation_btwn = StrSchema::default().btwn_field(FIELD.into(), FIELD_B.into());
        assert_eq!(validation_eq, StrSchema { operation: Some(OPERATION_FIELD_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { operation: Some(OPERATION_FIELD_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { operation: Some(OPERATION_FIELD_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { operation: Some(OPERATION_FIELD_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { operation: Some(OPERATION_FIELD_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { operation: Some(OPERATION_FIELD_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { operation: Some(OPERATION_FIELD_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_bytes_len_value() {
        let validation_eq = StrSchema::default().bytes_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().bytes_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().bytes_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().bytes_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().bytes_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().bytes_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().bytes_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { bytes_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_chars_len_value() {
        let validation_eq = StrSchema::default().chars_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().chars_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().chars_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().chars_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().chars_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().chars_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().chars_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { chars_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_graphemes_len_value() {
        let validation_eq = StrSchema::default().graphemes_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().graphemes_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().graphemes_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().graphemes_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().graphemes_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().graphemes_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().graphemes_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { graphemes_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_lowercase_len_value() {
        let validation_eq = StrSchema::default().lowercase_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().lowercase_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().lowercase_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().lowercase_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().lowercase_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().lowercase_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().lowercase_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { lowercase_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_uppercase_len_value() {
        let validation_eq = StrSchema::default().uppercase_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().uppercase_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().uppercase_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().uppercase_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().uppercase_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().uppercase_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().uppercase_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { uppercase_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_numbers_len_value() {
        let validation_eq = StrSchema::default().numbers_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().numbers_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().numbers_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().numbers_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().numbers_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().numbers_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().numbers_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { numbers_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_symbols_len_value() {
        let validation_eq = StrSchema::default().symbols_len_eq(VALUE_LEN);
        let validation_ne = StrSchema::default().symbols_len_ne(VALUE_LEN);
        let validation_gt = StrSchema::default().symbols_len_gt(VALUE_LEN);
        let validation_ge = StrSchema::default().symbols_len_ge(VALUE_LEN);
        let validation_lt = StrSchema::default().symbols_len_lt(VALUE_LEN);
        let validation_le = StrSchema::default().symbols_len_le(VALUE_LEN);
        let validation_btwn = StrSchema::default().symbols_len_btwn(VALUE_LEN, VALUE_LEN_B);
        assert_eq!(validation_eq, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { symbols_len: Some(OPERATION_VALUE_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_bytes_len_field() {
        let validation_eq = StrSchema::default().bytes_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().bytes_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().bytes_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().bytes_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().bytes_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().bytes_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().bytes_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { bytes_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_chars_len_field() {
        let validation_eq = StrSchema::default().chars_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().chars_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().chars_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().chars_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().chars_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().chars_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().chars_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { chars_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_graphemes_len_field() {
        let validation_eq = StrSchema::default().graphemes_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().graphemes_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().graphemes_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().graphemes_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().graphemes_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().graphemes_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().graphemes_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { graphemes_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_lowercase_len_field() {
        let validation_eq = StrSchema::default().lowercase_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().lowercase_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().lowercase_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().lowercase_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().lowercase_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().lowercase_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().lowercase_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { lowercase_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_uppercase_len_field() {
        let validation_eq = StrSchema::default().uppercase_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().uppercase_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().uppercase_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().uppercase_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().uppercase_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().uppercase_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().uppercase_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { uppercase_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_numbers_len_field() {
        let validation_eq = StrSchema::default().numbers_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().numbers_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().numbers_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().numbers_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().numbers_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().numbers_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().numbers_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { numbers_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }

    #[test]
    fn str_schema_symbols_len_field() {
        let validation_eq = StrSchema::default().symbols_len_eq_field(FIELD_LEN.into());
        let validation_ne = StrSchema::default().symbols_len_ne_field(FIELD_LEN.into());
        let validation_gt = StrSchema::default().symbols_len_gt_field(FIELD_LEN.into());
        let validation_ge = StrSchema::default().symbols_len_ge_field(FIELD_LEN.into());
        let validation_lt = StrSchema::default().symbols_len_lt_field(FIELD_LEN.into());
        let validation_le = StrSchema::default().symbols_len_le_field(FIELD_LEN.into());
        let validation_btwn = StrSchema::default().symbols_len_btwn_field(FIELD_LEN.into(), FIELD_LEN_B.into());
        assert_eq!(validation_eq, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_EQ.clone()), ..Default::default() });
        assert_eq!(validation_ne, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_NE.clone()), ..Default::default() });
        assert_eq!(validation_gt, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_GT.clone()), ..Default::default() });
        assert_eq!(validation_ge, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_GE.clone()), ..Default::default() });
        assert_eq!(validation_lt, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_LT.clone()), ..Default::default() });
        assert_eq!(validation_le, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_LE.clone()), ..Default::default() });
        assert_eq!(validation_btwn, StrSchema { symbols_len: Some(OPERATION_FIELD_LEN_BTWN.clone()), ..Default::default() });
    }
}
