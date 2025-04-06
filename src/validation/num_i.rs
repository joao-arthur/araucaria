#[derive(Debug, PartialEq, Clone)]
pub struct NumIValidation {
    pub required: bool,
    pub eq: Option<i64>,
    pub ne: Option<i64>,
    pub gt: Option<i64>,
    pub lt: Option<i64>,
    pub ge: Option<i64>,
    pub le: Option<i64>,
}

impl Default for NumIValidation {
    fn default() -> Self {
        NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl NumIValidation {
    pub fn optional(self) -> Self {
        NumIValidation { required: false, ..self }
    }

    pub fn eq(self, value: i64) -> Self {
        NumIValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: i64) -> Self {
        NumIValidation { ne: Some(value), ..self }
    }

    pub fn gt(self, value: i64) -> Self {
        NumIValidation { gt: Some(value), ..self }
    }

    pub fn lt(self, value: i64) -> Self {
        NumIValidation { lt: Some(value), ..self }
    }

    pub fn ge(self, value: i64) -> Self {
        NumIValidation { ge: Some(value), ..self }
    }

    pub fn le(self, value: i64) -> Self {
        NumIValidation { le: Some(value), ..self }
    }

    pub fn btwn(self, a: i64, b: i64) -> Self {
        NumIValidation { ge: Some(a), le: Some(b), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::NumIValidation;

    #[test]
    fn test_num_i_validation() {
        assert_eq!(NumIValidation::default(), NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            NumIValidation::default().optional(),
            NumIValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumIValidation::default().eq(-1),
            NumIValidation { required: true, eq: Some(-1), ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumIValidation::default().ne(-2),
            NumIValidation { required: true, eq: None, ne: Some(-2), gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumIValidation::default().gt(-3),
            NumIValidation { required: true, eq: None, ne: None, gt: Some(-3), lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumIValidation::default().lt(-4),
            NumIValidation { required: true, eq: None, ne: None, gt: None, lt: Some(-4), ge: None, le: None }
        );
        assert_eq!(
            NumIValidation::default().ge(-5),
            NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(-5), le: None }
        );
        assert_eq!(
            NumIValidation::default().le(-6),
            NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(-6) }
        );
        assert_eq!(
            NumIValidation::default().btwn(-42, 42),
            NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(-42), le: Some(42) }
        );
    }
}
