use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub struct NumFValidation {
    pub required: bool,
    pub operation: Option<Operation<f64>>,
}

impl Default for NumFValidation {
    fn default() -> Self {
        NumFValidation { required: true, operation: None }
    }
}

impl NumFValidation {
    pub fn optional(self) -> Self {
        NumFValidation { required: false, ..self }
    }

    pub fn eq(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Eq(value)), ..self }
    }

    pub fn ne(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Ne(value)), ..self }
    }

    pub fn gt(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Gt(value)), ..self }
    }

    pub fn ge(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Ge(value)), ..self }
    }

    pub fn lt(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Lt(value)), ..self }
    }

    pub fn le(self, value: f64) -> Self {
        NumFValidation { operation: Some(Operation::Le(value)), ..self }
    }

    pub fn btwn(self, value_a: f64, value_b: f64) -> Self {
        NumFValidation { operation: Some(Operation::Btwn(value_a, value_b)), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::Operation;

    use super::NumFValidation;

    #[test]
    fn test_num_f_validation() {
        assert_eq!(NumFValidation::default(), NumFValidation { required: true, operation: None });
        assert_eq!(NumFValidation::default().optional(), NumFValidation { required: false, operation: None });
        assert_eq!(NumFValidation::default().eq(-1.5), NumFValidation { required: true, operation: Some(Operation::Eq(-1.5)) });
        assert_eq!(NumFValidation::default().ne(-2.5), NumFValidation { required: true, operation: Some(Operation::Ne(-2.5)) });
        assert_eq!(NumFValidation::default().gt(-3.5), NumFValidation { required: true, operation: Some(Operation::Gt(-3.5)) });
        assert_eq!(NumFValidation::default().ge(-4.5), NumFValidation { required: true, operation: Some(Operation::Ge(-4.5)) });
        assert_eq!(NumFValidation::default().lt(-5.5), NumFValidation { required: true, operation: Some(Operation::Lt(-5.5)) });
        assert_eq!(NumFValidation::default().le(-6.5), NumFValidation { required: true, operation: Some(Operation::Le(-6.5)) });
        assert_eq!(NumFValidation::default().btwn(-42.5, 42.5), NumFValidation { required: true, operation: Some(Operation::Btwn(-42.5, 42.5)) });
    }
}
