#[derive(Debug, PartialEq, Clone)]
pub struct DateValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub gt: Option<String>,
    pub lt: Option<String>,
    pub ge: Option<String>,
    pub le: Option<String>,
}

impl Default for DateValidation {
    fn default() -> Self {
        DateValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl DateValidation {
    pub fn optional(self) -> Self {
        DateValidation { required: false, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn eq(self, value: String) -> Self {
        DateValidation { required: self.required, eq: Some(value), ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn ne(self, value: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: Some(value), gt: self.gt, lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn gt(self, value: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: self.ne, gt: Some(value), lt: self.lt, ge: self.ge, le: self.le }
    }

    pub fn lt(self, value: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: Some(value), ge: self.ge, le: self.le }
    }

    pub fn ge(self, value: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: Some(value), le: self.le }
    }

    pub fn le(self, value: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: self.ge, le: Some(value) }
    }

    pub fn between(self, a: String, b: String) -> Self {
        DateValidation { required: self.required, eq: self.eq, ne: self.ne, gt: self.gt, lt: self.lt, ge: Some(a), le: Some(b) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_validation() {
        assert_eq!(DateValidation::default(), DateValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            DateValidation::default().optional(),
            DateValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateValidation::default().eq(String::from("2026-08-12")),
            DateValidation { required: true, eq: Some(String::from("2026-08-12")), ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateValidation::default().ne(String::from("2027-08-02")),
            DateValidation { required: true, eq: None, ne: Some(String::from("2027-08-02")), gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateValidation::default().gt(String::from("2028-07-22")),
            DateValidation { required: true, eq: None, ne: None, gt: Some(String::from("2028-07-22")), lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateValidation::default().lt(String::from("2030-11-25")),
            DateValidation { required: true, eq: None, ne: None, gt: None, lt: Some(String::from("2030-11-25")), ge: None, le: None }
        );
        assert_eq!(
            DateValidation::default().ge(String::from("2031-11-14")),
            DateValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(String::from("2031-11-14")), le: None }
        );
        assert_eq!(
            DateValidation::default().le(String::from("2033-03-30")),
            DateValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(String::from("2033-03-30")) }
        );
        assert_eq!(
            DateValidation::default().between(String::from("2031-11-14"), String::from("2033-03-30")),
            DateValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: Some(String::from("2031-11-14")),
                le: Some(String::from("2033-03-30"))
            }
        );
    }
}
