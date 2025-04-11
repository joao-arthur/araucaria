use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub struct NumIValidation {
    pub required: bool,
    pub operation: Option<Operation<i64>>,
}

impl Default for NumIValidation {
    fn default() -> Self {
        NumIValidation { required: true, operation: None }
    }
}

impl NumIValidation {
    pub fn optional(self) -> Self {
        NumIValidation { required: false, ..self }
    }

    pub fn eq(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Eq(value)), ..self }
    }

    pub fn ne(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Ne(value)), ..self }
    }

    pub fn gt(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Gt(value)), ..self }
    }

    pub fn ge(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Ge(value)), ..self }
    }

    pub fn lt(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Lt(value)), ..self }
    }

    pub fn le(self, value: i64) -> Self {
        NumIValidation { operation: Some(Operation::Le(value)), ..self }
    }

    pub fn btwn(self, value_a: i64, value_b: i64) -> Self {
        NumIValidation { operation: Some(Operation::Btwn(value_a, value_b)), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::Operation;

    use super::NumIValidation;

    #[test]
    fn test_num_i_validation() {
        assert_eq!(NumIValidation::default(), NumIValidation { required: true, operation: None });
        assert_eq!(NumIValidation::default().optional(), NumIValidation { required: false, operation: None });
        assert_eq!(NumIValidation::default().eq(-1), NumIValidation { required: true, operation: Some(Operation::Eq(-1)) });
        assert_eq!(NumIValidation::default().ne(-2), NumIValidation { required: true, operation: Some(Operation::Ne(-2)) });
        assert_eq!(NumIValidation::default().gt(-3), NumIValidation { required: true, operation: Some(Operation::Gt(-3)) });
        assert_eq!(NumIValidation::default().ge(-4), NumIValidation { required: true, operation: Some(Operation::Ge(-4)) });
        assert_eq!(NumIValidation::default().lt(-5), NumIValidation { required: true, operation: Some(Operation::Lt(-5)) });
        assert_eq!(NumIValidation::default().le(-6), NumIValidation { required: true, operation: Some(Operation::Le(-6)) });
        assert_eq!(NumIValidation::default().btwn(-42, 42), NumIValidation { required: true, operation: Some(Operation::Btwn(-42, 42)) });
    }
}
