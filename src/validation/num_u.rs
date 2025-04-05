#[derive(Debug, PartialEq, Clone)]
pub struct NumUValidation {
    pub required: bool,
    pub eq: Option<u64>,
    pub ne: Option<u64>,
    pub gt: Option<u64>,
    pub lt: Option<u64>,
    pub ge: Option<u64>,
    pub le: Option<u64>,
}

impl Default for NumUValidation {
    fn default() -> Self {
        NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl NumUValidation {
    pub fn optional(self) -> Self {
        NumUValidation { required: false, ..self }
    }

    pub fn eq(self, value: u64) -> Self {
        NumUValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: u64) -> Self {
        NumUValidation { ne: Some(value), ..self }
    }

    pub fn gt(self, value: u64) -> Self {
        NumUValidation { gt: Some(value), ..self }
    }

    pub fn lt(self, value: u64) -> Self {
        NumUValidation { lt: Some(value), ..self }
    }

    pub fn ge(self, value: u64) -> Self {
        NumUValidation { ge: Some(value), ..self }
    }

    pub fn le(self, value: u64) -> Self {
        NumUValidation { le: Some(value), ..self }
    }

    pub fn btwn(self, a: u64, b: u64) -> Self {
        NumUValidation { ge: Some(a), le: Some(b), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_u_validation() {
        assert_eq!(NumUValidation::default(), NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            NumUValidation::default().optional(),
            NumUValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(NumUValidation::default().eq(1), NumUValidation { required: true, eq: Some(1), ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(NumUValidation::default().ne(2), NumUValidation { required: true, eq: None, ne: Some(2), gt: None, lt: None, ge: None, le: None });
        assert_eq!(NumUValidation::default().gt(3), NumUValidation { required: true, eq: None, ne: None, gt: Some(3), lt: None, ge: None, le: None });
        assert_eq!(NumUValidation::default().lt(4), NumUValidation { required: true, eq: None, ne: None, gt: None, lt: Some(4), ge: None, le: None });
        assert_eq!(NumUValidation::default().ge(5), NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(5), le: None });
        assert_eq!(NumUValidation::default().le(6), NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(6) });
        assert_eq!(
            NumUValidation::default().btwn(1, 9),
            NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(1), le: Some(9) }
        );
    }
}
