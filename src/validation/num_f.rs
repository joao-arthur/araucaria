#[derive(Debug, PartialEq, Clone)]
pub struct NumFValidation {
    pub required: bool,
    pub eq: Option<f64>,
    pub ne: Option<f64>,
    pub gt: Option<f64>,
    pub lt: Option<f64>,
    pub ge: Option<f64>,
    pub le: Option<f64>,
}

impl Default for NumFValidation {
    fn default() -> Self {
        NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl NumFValidation {
    pub fn optional(self) -> Self {
        NumFValidation { required: false, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn eq(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: Some(value), ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn ne(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: Some(value), gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn gt(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: self.ne, gt: Some(value), lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn lt(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: Some(value), ge: self.ge, le: self.le }
    }

    pub fn ge(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: Some(value), le: self.le }
    }

    pub fn le(self, value: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: Some(value) }
    }

    pub fn between(self, a: f64, b: f64) -> Self {
        NumFValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: Some(a), le: Some(b) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_f_validation() {
        assert_eq!(NumFValidation::default(), NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            NumFValidation::default().optional(),
            NumFValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumFValidation::default().eq(-1.5),
            NumFValidation { required: true, eq: Some(-1.5), ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumFValidation::default().ne(-2.5),
            NumFValidation { required: true, eq: None, ne: Some(-2.5), gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumFValidation::default().gt(-3.5),
            NumFValidation { required: true, eq: None, ne: None, gt: Some(-3.5), lt: None, ge: None, le: None }
        );
        assert_eq!(
            NumFValidation::default().lt(-4.5),
            NumFValidation { required: true, eq: None, ne: None, gt: None, lt: Some(-4.5), ge: None, le: None }
        );
        assert_eq!(
            NumFValidation::default().ge(-5.5),
            NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(-5.5), le: None }
        );
        assert_eq!(
            NumFValidation::default().le(-6.5),
            NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(-6.5) }
        );
        assert_eq!(
            NumFValidation::default().between(-42.5, 42.5),
            NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(-42.5), le: Some(42.5) }
        );
    }
}
