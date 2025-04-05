#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub gt: Option<String>,
    pub lt: Option<String>,
    pub ge: Option<String>,
    pub le: Option<String>,
}

impl Default for DateTimeValidation {
    fn default() -> Self {
        DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl DateTimeValidation {
    pub fn optional(self) -> Self {
        DateTimeValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        DateTimeValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        DateTimeValidation { ne: Some(value), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        DateTimeValidation { gt: Some(value), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        DateTimeValidation { lt: Some(value), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        DateTimeValidation { ge: Some(value), ..self }
    }

    pub fn le(self, value: String) -> Self {
        DateTimeValidation { le: Some(value), ..self }
    }

    pub fn between(self, a: String, b: String) -> Self {
        DateTimeValidation { ge: Some(a), le: Some(b), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_time_validation() {
        assert_eq!(DateTimeValidation::default(), DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            DateTimeValidation::default().optional(),
            DateTimeValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateTimeValidation::default().eq(String::from("2026-08-12T08:10Z")),
            DateTimeValidation { required: true, eq: Some(String::from("2026-08-12T08:10Z")), ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateTimeValidation::default().ne(String::from("2027-08-02T10:27Z")),
            DateTimeValidation { required: true, eq: None, ne: Some(String::from("2027-08-02T10:27Z")), gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateTimeValidation::default().gt(String::from("2028-07-22T19:41Z")),
            DateTimeValidation { required: true, eq: None, ne: None, gt: Some(String::from("2028-07-22T19:41Z")), lt: None, ge: None, le: None }
        );
        assert_eq!(
            DateTimeValidation::default().lt(String::from("2030-11-25T03:01Z")),
            DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: Some(String::from("2030-11-25T03:01Z")), ge: None, le: None }
        );
        assert_eq!(
            DateTimeValidation::default().ge(String::from("2031-11-14T00:00Z")),
            DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(String::from("2031-11-14T00:00Z")), le: None }
        );
        assert_eq!(
            DateTimeValidation::default().le(String::from("2033-03-30T01:01Z")),
            DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(String::from("2033-03-30T01:01Z")) }
        );
        assert_eq!(
            DateTimeValidation::default().between(String::from("2031-11-14T00:00Z"), String::from("2033-03-30T01:01Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: Some(String::from("2031-11-14T00:00Z")),
                le: Some(String::from("2033-03-30T01:01Z"))
            }
        );
    }
}
