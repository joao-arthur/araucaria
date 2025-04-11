use crate::operation::OperationEq;

#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub operation: Option<OperationEq<bool>>,
}

impl Default for BoolValidation {
    fn default() -> Self {
        BoolValidation { required: true, operation: None }
    }
}

impl BoolValidation {
    pub fn optional(self) -> Self {
        BoolValidation { required: false, ..self }
    }

    pub fn eq(self, value: bool) -> Self {
        BoolValidation { operation: Some(OperationEq::Eq(value)), ..self }
    }

    pub fn ne(self, value: bool) -> Self {
        BoolValidation { operation: Some(OperationEq::Ne(value)), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::OperationEq;

    use super::BoolValidation;

    #[test]
    fn test_bool_validation() {
        assert_eq!(BoolValidation::default(), BoolValidation { required: true, operation: None });
        assert_eq!(BoolValidation::default().optional(), BoolValidation { required: false, operation: None });
        assert_eq!(BoolValidation::default().eq(false), BoolValidation { required: true, operation: Some(OperationEq::Eq(false)) });
        assert_eq!(BoolValidation::default().ne(true), BoolValidation { required: true, operation: Some(OperationEq::Ne(true)) });
    }
}
