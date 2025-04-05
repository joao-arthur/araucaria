#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub eq: Option<bool>,
    pub ne: Option<bool>,
}

impl Default for BoolValidation {
    fn default() -> Self {
        BoolValidation { required: true, eq: None, ne: None }
    }
}

impl BoolValidation {
    pub fn optional(self) -> Self {
        BoolValidation { required: false, ..self }
    }

    pub fn eq(self, value: bool) -> Self {
        BoolValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: bool) -> Self {
        BoolValidation { ne: Some(value), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bool_validation() {
        assert_eq!(BoolValidation::default(), BoolValidation { required: true, eq: None, ne: None });
        assert_eq!(BoolValidation::default().optional(), BoolValidation { required: false, eq: None, ne: None });
        assert_eq!(BoolValidation::default().eq(false), BoolValidation { required: true, eq: Some(false), ne: None });
        assert_eq!(BoolValidation::default().ne(true), BoolValidation { required: true, eq: None, ne: Some(true) });
    }
}
