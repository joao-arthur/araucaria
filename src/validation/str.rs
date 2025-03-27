#[derive(Debug, PartialEq, Clone)]
pub struct StrValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub min_bytes_len: Option<usize>,
    pub max_bytes_len: Option<usize>,
    pub min_graphemes_len: Option<usize>,
    pub max_graphemes_len: Option<usize>,
    pub min_lowercase_len: Option<usize>,
    pub max_lowercase_len: Option<usize>,
    pub min_uppercase_len: Option<usize>,
    pub max_uppercase_len: Option<usize>,
    pub min_number_len: Option<usize>,
    pub max_number_len: Option<usize>,
    pub min_symbols_len: Option<usize>,
    pub max_symbols_len: Option<usize>,
}

impl Default for StrValidation {
    fn default() -> Self {
        StrValidation {
            required: true,
            eq: None,
            ne: None,
            min_bytes_len: None,
            max_bytes_len: None,
            min_graphemes_len: None,
            max_graphemes_len: None,
            min_lowercase_len: None,
            max_lowercase_len: None,
            min_uppercase_len: None,
            max_uppercase_len: None,
            min_number_len: None,
            max_number_len: None,
            min_symbols_len: None,
            max_symbols_len: None,
        }
    }
}

impl StrValidation {
    pub fn optional(self) -> Self {
        StrValidation {
            required: false,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn eq(self, value: String) -> Self {
        StrValidation {
            required: self.required,
            eq: Some(value),
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn ne(self, value: String) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: Some(value),
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_bytes_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: Some(value),
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_bytes_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: Some(value),
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_graphemes_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: Some(value),
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_graphemes_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: Some(value),
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_lowercase_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: Some(value),
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_lowercase_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: Some(value),
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_uppercase_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: Some(value),
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_uppercase_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: Some(value),
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_number_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: Some(value),
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_number_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: Some(value),
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn min_symbols_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: Some(value),
            max_symbols_len: self.max_symbols_len,
        }
    }

    pub fn max_symbols_len(self, value: usize) -> Self {
        StrValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            min_bytes_len: self.min_bytes_len,
            max_bytes_len: self.max_bytes_len,
            min_graphemes_len: self.min_graphemes_len,
            max_graphemes_len: self.max_graphemes_len,
            min_lowercase_len: self.min_lowercase_len,
            max_lowercase_len: self.max_lowercase_len,
            min_uppercase_len: self.min_uppercase_len,
            max_uppercase_len: self.max_uppercase_len,
            min_number_len: self.min_number_len,
            max_number_len: self.max_number_len,
            min_symbols_len: self.min_symbols_len,
            max_symbols_len: Some(value),
        }
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
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().optional(),
            StrValidation {
                required: false,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().eq(String::from("Avalon")),
            StrValidation {
                required: true,
                eq: Some(String::from("Avalon")),
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().ne(String::from("Mu")),
            StrValidation {
                required: true,
                eq: None,
                ne: Some(String::from("Mu")),
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_bytes_len(1),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: Some(1),
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_bytes_len(2),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: Some(2),
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_graphemes_len(33),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: Some(33),
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_graphemes_len(44),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: Some(44),
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_lowercase_len(555),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: Some(555),
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_lowercase_len(666),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: Some(666),
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_uppercase_len(7_777),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: Some(7_777),
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_uppercase_len(8_888),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: Some(8_888),
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_number_len(99_999),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: Some(99_999),
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_number_len(88_888),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: Some(88_888),
                min_symbols_len: None,
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().min_symbols_len(777_777),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: Some(777_777),
                max_symbols_len: None,
            }
        );
        assert_eq!(
            StrValidation::default().max_symbols_len(666_666),
            StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: Some(666_666),
            }
        );
    }
}
