use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub struct NumUValidation {
    pub required: bool,
    pub operation: Option<Operation<u64>>,
}

impl Default for NumUValidation {
    fn default() -> Self {
        NumUValidation { required: true, operation: None }
    }
}

impl NumUValidation {
    pub fn optional(self) -> Self {
        NumUValidation { required: false, ..self }
    }

    pub fn eq(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Eq(value)), ..self }
    }

    pub fn ne(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Ne(value)), ..self }
    }

    pub fn gt(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Gt(value)), ..self }
    }

    pub fn ge(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Ge(value)), ..self }
    }

    pub fn lt(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Lt(value)), ..self }
    }

    pub fn le(self, value: u64) -> Self {
        NumUValidation { operation: Some(Operation::Le(value)), ..self }
    }

    pub fn btwn(self, value_a: u64, value_b: u64) -> Self {
        NumUValidation { operation: Some(Operation::Btwn(value_a, value_b)), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::Operation;

    use super::NumUValidation;

    #[test]
    fn test_num_u_validation() {
        assert_eq!(NumUValidation::default(), NumUValidation { required: true, operation: None });
        assert_eq!(NumUValidation::default().optional(), NumUValidation { required: false, operation: None });
        assert_eq!(NumUValidation::default().eq(1), NumUValidation { required: true, operation: Some(Operation::Eq(1)) });
        assert_eq!(NumUValidation::default().ne(2), NumUValidation { required: true, operation: Some(Operation::Ne(2)) });
        assert_eq!(NumUValidation::default().gt(3), NumUValidation { required: true, operation: Some(Operation::Gt(3)) });
        assert_eq!(NumUValidation::default().ge(4), NumUValidation { required: true, operation: Some(Operation::Ge(4)) });
        assert_eq!(NumUValidation::default().lt(5), NumUValidation { required: true, operation: Some(Operation::Lt(5)) });
        assert_eq!(NumUValidation::default().le(6), NumUValidation { required: true, operation: Some(Operation::Le(6)) });
        assert_eq!(NumUValidation::default().btwn(1, 9), NumUValidation { required: true, operation: Some(Operation::Btwn(1, 9)) });
    }
}
