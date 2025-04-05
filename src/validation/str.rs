use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub struct StrValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub bytes_len: Option<Operation>,
    pub chars_len: Option<Operation>,
    pub graphemes_len: Option<Operation>,
    pub lowercase_len: Option<Operation>,
    pub uppercase_len: Option<Operation>,
    pub number_len: Option<Operation>,
    pub symbols_len: Option<Operation>,
}

impl Default for StrValidation {
    fn default() -> Self {
        StrValidation {
            required: true,
            eq: None,
            ne: None,
            bytes_len: None,
            chars_len: None,
            graphemes_len: None,
            lowercase_len: None,
            uppercase_len: None,
            number_len: None,
            symbols_len: None,
        }
    }
}

impl StrValidation {
    pub fn optional(self) -> Self {
        StrValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        StrValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        StrValidation { ne: Some(value), ..self }
    }

    pub fn bytes_len(self, operation: Operation) -> Self {
        StrValidation { bytes_len: Some(operation), ..self }
    }

    pub fn chars_len(self, operation: Operation) -> Self {
        StrValidation { chars_len: Some(operation), ..self }
    }

    pub fn graphemes_len(self, operation: Operation) -> Self {
        StrValidation { graphemes_len: Some(operation), ..self }
    }

    pub fn lowercase_len(self, operation: Operation) -> Self {
        StrValidation { lowercase_len: Some(operation), ..self }
    }

    pub fn uppercase_len(self, operation: Operation) -> Self {
        StrValidation { uppercase_len: Some(operation), ..self }
    }

    pub fn number_len(self, operation: Operation) -> Self {
        StrValidation { number_len: Some(operation), ..self }
    }

    pub fn symbols_len(self, operation: Operation) -> Self {
        StrValidation { symbols_len: Some(operation), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_validation() {
        assert_eq!(
            StrValidation::default(),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().optional(),
            StrValidation {
                required: false,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().eq(String::from("Avalon")),
            StrValidation {
                required: true,
                eq: Some(String::from("Avalon")),
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().ne(String::from("Mu")),
            StrValidation {
                required: true,
                eq: None,
                ne: Some(String::from("Mu")),
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().bytes_len(Operation::Eq(11)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: Some(Operation::Eq(11)),
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().chars_len(Operation::Ne(22)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: Some(Operation::Ne(22)),
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().graphemes_len(Operation::Gt(33)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: Some(Operation::Gt(33)),
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().lowercase_len(Operation::Lt(44)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                chars_len: None,
                bytes_len: None,
                graphemes_len: None,
                lowercase_len: Some(Operation::Lt(44)),
                uppercase_len: None,
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().uppercase_len(Operation::Ge(55)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: Some(Operation::Ge(55)),
                number_len: None,
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().number_len(Operation::Le(66)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: Some(Operation::Le(66)),
                symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().symbols_len(Operation::Btwn(77, 88)),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                number_len: None,
                symbols_len: Some(Operation::Btwn(77, 88)),
            }
        );
    }
}
