use crate::operation::{Operation, OperationEq};

#[derive(Debug, PartialEq, Clone)]
pub struct StrValidation {
    pub required: bool,
    pub operation: Option<OperationEq<String>>,
    pub bytes_len: Option<Operation<usize>>,
    pub chars_len: Option<Operation<usize>>,
    pub graphemes_len: Option<Operation<usize>>,
    pub lowercase_len: Option<Operation<usize>>,
    pub uppercase_len: Option<Operation<usize>>,
    pub numbers_len: Option<Operation<usize>>,
    pub symbols_len: Option<Operation<usize>>,
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
        StrValidation { operation: Some(OperationEq::Eq(value)), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        StrValidation { operation: Some(OperationEq::Ne(value)), ..self }
    }

    pub fn bytes_len_eq(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Eq(len)), ..self }
    }

    pub fn bytes_len_ne(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Ne(len)), ..self }
    }

    pub fn bytes_len_gt(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Gt(len)), ..self }
    }

    pub fn bytes_len_ge(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Ge(len)), ..self }
    }

    pub fn bytes_len_lt(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Lt(len)), ..self }
    }

    pub fn bytes_len_le(self, len: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Le(len)), ..self }
    }

    pub fn bytes_len_btwn(self, len_a: usize, len_b: usize) -> Self {
        StrValidation { bytes_len: Some(Operation::Btwn(len_a, len_b)), ..self }
    }

    pub fn chars_len(self, operation: Operation<usize>) -> Self {
        StrValidation { chars_len: Some(operation), ..self }
    }

    pub fn graphemes_len(self, operation: Operation<usize>) -> Self {
        StrValidation { graphemes_len: Some(operation), ..self }
    }

    pub fn lowercase_len(self, operation: Operation<usize>) -> Self {
        StrValidation { lowercase_len: Some(operation), ..self }
    }

    pub fn uppercase_len(self, operation: Operation<usize>) -> Self {
        StrValidation { uppercase_len: Some(operation), ..self }
    }

    pub fn numbers_len(self, operation: Operation<usize>) -> Self {
        StrValidation { numbers_len: Some(operation), ..self }
    }

    pub fn symbols_len(self, operation: Operation<usize>) -> Self {
        StrValidation { symbols_len: Some(operation), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::{Operation, OperationEq};

    use super::StrValidation;

    #[test]
    fn test_str_validation() {
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
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().optional(),
            StrValidation {
                required: false,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().eq(String::from("Avalon")),
            StrValidation {
                required: true,
                operation: Some(OperationEq::Eq(String::from("Avalon"))),
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().ne(String::from("Mu")),
            StrValidation {
                required: true,
                operation: Some(OperationEq::Ne(String::from("Mu"))),
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_eq(11),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Eq(11)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ne(12),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Ne(12)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_gt(13),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Gt(13)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_ge(14),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Ge(14)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_lt(15),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Lt(15)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_le(16),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Le(16)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len_btwn(17, 18),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: Some(Operation::Btwn(17, 18)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().chars_len(Operation::Ne(22)),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: Some(Operation::Ne(22)),
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().graphemes_len(Operation::Gt(33)),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: Some(Operation::Gt(33)),
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().lowercase_len(Operation::Lt(44)),
            StrValidation {
                required: true,
                operation: None,
                chars_len: None,
                bytes_len: None,
                graphemes_len: None,
                lowercase_len: Some(Operation::Lt(44)),
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().uppercase_len(Operation::Ge(55)),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: Some(Operation::Ge(55)),
                numbers_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().numbers_len(Operation::Le(66)),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: Some(Operation::Le(66)),
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().symbols_len(Operation::Btwn(77, 88)),
            StrValidation {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: Some(Operation::Btwn(77, 88)),
            }
        );
    }
}
