#[derive(Debug, PartialEq, Clone)]
pub struct TimeValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub gt: Option<String>,
    pub lt: Option<String>,
    pub ge: Option<String>,
    pub le: Option<String>,
}

impl Default for TimeValidation {
    fn default() -> Self {
        TimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
    }
}

impl TimeValidation {
    pub fn optional(self) -> Self {
        TimeValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        TimeValidation { eq: Some(value), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        TimeValidation { ne: Some(value), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        TimeValidation { gt: Some(value), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        TimeValidation { lt: Some(value), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        TimeValidation { ge: Some(value), ..self }
    }

    pub fn le(self, value: String) -> Self {
        TimeValidation { le: Some(value), ..self }
    }

    pub fn btwn(self, a: String, b: String) -> Self {
        TimeValidation { ge: Some(a), le: Some(b), ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time_validation() {
        assert_eq!(TimeValidation::default(), TimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None });
        assert_eq!(
            TimeValidation::default().optional(),
            TimeValidation { required: false, eq: None, ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            TimeValidation::default().eq(String::from("08:10")),
            TimeValidation { required: true, eq: Some(String::from("08:10")), ne: None, gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            TimeValidation::default().ne(String::from("10:27")),
            TimeValidation { required: true, eq: None, ne: Some(String::from("10:27")), gt: None, lt: None, ge: None, le: None }
        );
        assert_eq!(
            TimeValidation::default().gt(String::from("19:41")),
            TimeValidation { required: true, eq: None, ne: None, gt: Some(String::from("19:41")), lt: None, ge: None, le: None }
        );
        assert_eq!(
            TimeValidation::default().lt(String::from("03:01")),
            TimeValidation { required: true, eq: None, ne: None, gt: None, lt: Some(String::from("03:01")), ge: None, le: None }
        );
        assert_eq!(
            TimeValidation::default().ge(String::from("00:00")),
            TimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: Some(String::from("00:00")), le: None }
        );
        assert_eq!(
            TimeValidation::default().le(String::from("01:01")),
            TimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: Some(String::from("01:01")) }
        );
        assert_eq!(
            TimeValidation::default().btwn(String::from("00:00"), String::from("23:59")),
            TimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: Some(String::from("00:00")),
                le: Some(String::from("23:59"))
            }
        );
    }
}
