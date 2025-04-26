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
    use crate::operation::{Operand, OperandValue, Operation};

    use super::StrValidation;

    #[test]
    fn str_validation() {
        assert_eq!(
            StrValidation::default(),
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
            }
        );
        assert_eq!(StrValidation::default().optional(), StrValidation { required: false, ..Default::default() });
        assert_eq!(
            StrValidation::default().eq("Avalon".into()),
            StrValidation { operation: Some(Operation::Eq(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().ne("Avalon".into()),
            StrValidation { operation: Some(Operation::Ne(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().gt("Avalon".into()),
            StrValidation { operation: Some(Operation::Gt(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().ge("Avalon".into()),
            StrValidation { operation: Some(Operation::Ge(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lt("Avalon".into()),
            StrValidation { operation: Some(Operation::Lt(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().le("Avalon".into()),
            StrValidation { operation: Some(Operation::Le(Operand::Value(OperandValue::from("Avalon")))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().btwn("Avalon".into(), "Mu".into()),
            StrValidation {
                operation: Some(Operation::Btwn(Operand::Value(OperandValue::from("Avalon")), Operand::Value(OperandValue::from("Mu")))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_eq(11),
            StrValidation { bytes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(11)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ne(12),
            StrValidation { bytes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(12)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_gt(13),
            StrValidation { bytes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(13)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ge(14),
            StrValidation { bytes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(14)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_lt(15),
            StrValidation { bytes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(15)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_le(16),
            StrValidation { bytes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(16)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_btwn(17, 18),
            StrValidation {
                bytes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(17)), Operand::Value(OperandValue::USize(18)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().chars_len_eq(21),
            StrValidation { chars_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(21)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_ne(22),
            StrValidation { chars_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(22)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_gt(23),
            StrValidation { chars_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(23)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_ge(24),
            StrValidation { chars_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(24)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_lt(25),
            StrValidation { chars_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(25)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_le(26),
            StrValidation { chars_len: Some(Operation::Le(Operand::Value(OperandValue::USize(26)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_btwn(27, 28),
            StrValidation {
                chars_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(27)), Operand::Value(OperandValue::USize(28)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_eq(31),
            StrValidation { graphemes_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(31)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_ne(32),
            StrValidation { graphemes_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(32)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_gt(33),
            StrValidation { graphemes_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(33)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_ge(34),
            StrValidation { graphemes_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(34)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_lt(35),
            StrValidation { graphemes_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(35)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_le(36),
            StrValidation { graphemes_len: Some(Operation::Le(Operand::Value(OperandValue::USize(36)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_btwn(37, 38),
            StrValidation {
                graphemes_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(37)), Operand::Value(OperandValue::USize(38)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_eq(41),
            StrValidation { lowercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(41)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_ne(42),
            StrValidation { lowercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(42)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_gt(43),
            StrValidation { lowercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(43)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_ge(44),
            StrValidation { lowercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(44)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_lt(45),
            StrValidation { lowercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(45)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_le(46),
            StrValidation { lowercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(46)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_btwn(47, 48),
            StrValidation {
                lowercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(47)), Operand::Value(OperandValue::USize(48)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_eq(51),
            StrValidation { uppercase_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(51)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_ne(52),
            StrValidation { uppercase_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(52)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_gt(53),
            StrValidation { uppercase_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(53)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_ge(54),
            StrValidation { uppercase_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(54)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_lt(55),
            StrValidation { uppercase_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(55)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_le(56),
            StrValidation { uppercase_len: Some(Operation::Le(Operand::Value(OperandValue::USize(56)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_btwn(57, 58),
            StrValidation {
                uppercase_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(57)), Operand::Value(OperandValue::USize(58)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().numbers_len_eq(61),
            StrValidation { numbers_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(61)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_ne(62),
            StrValidation { numbers_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(62)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_gt(63),
            StrValidation { numbers_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(63)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_ge(64),
            StrValidation { numbers_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(64)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_lt(65),
            StrValidation { numbers_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(65)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_le(66),
            StrValidation { numbers_len: Some(Operation::Le(Operand::Value(OperandValue::USize(66)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_btwn(67, 68),
            StrValidation {
                numbers_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(67)), Operand::Value(OperandValue::USize(68)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().symbols_len_eq(71),
            StrValidation { symbols_len: Some(Operation::Eq(Operand::Value(OperandValue::USize(71)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_ne(72),
            StrValidation { symbols_len: Some(Operation::Ne(Operand::Value(OperandValue::USize(72)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_gt(73),
            StrValidation { symbols_len: Some(Operation::Gt(Operand::Value(OperandValue::USize(73)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_ge(74),
            StrValidation { symbols_len: Some(Operation::Ge(Operand::Value(OperandValue::USize(74)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_lt(75),
            StrValidation { symbols_len: Some(Operation::Lt(Operand::Value(OperandValue::USize(75)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_le(76),
            StrValidation { symbols_len: Some(Operation::Le(Operand::Value(OperandValue::USize(76)))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_btwn(77, 78),
            StrValidation {
                symbols_len: Some(Operation::Btwn(Operand::Value(OperandValue::USize(77)), Operand::Value(OperandValue::USize(78)))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().eq_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().ne_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().gt_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().ge_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lt_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().le_field("foo.bar.baz".into()),
            StrValidation { operation: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                operation: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_eq_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ne_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_gt_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ge_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_lt_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_le_field("foo.bar.baz".into()),
            StrValidation { bytes_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().bytes_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                bytes_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().chars_len_eq_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_ne_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_gt_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_ge_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_lt_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_le_field("foo.bar.baz".into()),
            StrValidation { chars_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().chars_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                chars_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_eq_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_ne_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_gt_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_ge_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_lt_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_le_field("foo.bar.baz".into()),
            StrValidation { graphemes_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().graphemes_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                graphemes_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_eq_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_ne_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_gt_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_ge_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_lt_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_le_field("foo.bar.baz".into()),
            StrValidation { lowercase_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().lowercase_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                lowercase_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_eq_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_ne_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_gt_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_ge_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_lt_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_le_field("foo.bar.baz".into()),
            StrValidation { uppercase_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().uppercase_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                uppercase_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().numbers_len_eq_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_ne_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_gt_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_ge_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_lt_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_le_field("foo.bar.baz".into()),
            StrValidation { numbers_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().numbers_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                numbers_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
        assert_eq!(
            StrValidation::default().symbols_len_eq_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Eq(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_ne_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Ne(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_gt_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Gt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_ge_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Ge(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_lt_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Lt(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_le_field("foo.bar.baz".into()),
            StrValidation { symbols_len: Some(Operation::Le(Operand::FieldPath("foo.bar.baz".into()))), ..Default::default() }
        );
        assert_eq!(
            StrValidation::default().symbols_len_btwn_field("foo.bar.baz".into(), "baz.bar.foo".into()),
            StrValidation {
                symbols_len: Some(Operation::Btwn(Operand::FieldPath("foo.bar.baz".into()), Operand::FieldPath("baz.bar.foo".into()))),
                ..Default::default()
            }
        );
    }
}
