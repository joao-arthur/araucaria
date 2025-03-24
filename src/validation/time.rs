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
        TimeValidation {
            required: false,
            eq: None,
            ne: None,
            gt: None,
            lt: None,
            ge: None,
            le: None,
        }
    }
}

impl TimeValidation {
    pub fn required(self) -> Self {
        TimeValidation {
            required: true,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn eq(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: Some(value),
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ne(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: self.eq,
            ne: Some(value),
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn gt(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: Some(value),
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn lt(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: Some(value),
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ge(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: Some(value),
            le: self.le,
        }
    }

    pub fn le(self, value: String) -> Self {
        TimeValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: Some(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time_validation() {
        assert_eq!(
            TimeValidation::default(),
            TimeValidation {
                required: false,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default().required(),
            TimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default().required().eq(String::from("08:10")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default()
                .required()
                .eq(String::from("08:10"))
                .ne(String::from("10:27")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: Some(String::from("10:27")),
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default()
                .required()
                .eq(String::from("08:10"))
                .ne(String::from("10:27"))
                .gt(String::from("19:41")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: Some(String::from("10:27")),
                gt: Some(String::from("19:41")),
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default()
                .required()
                .eq(String::from("08:10"))
                .ne(String::from("10:27"))
                .gt(String::from("19:41"))
                .lt(String::from("03:01")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: Some(String::from("10:27")),
                gt: Some(String::from("19:41")),
                lt: Some(String::from("03:01")),
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default()
                .required()
                .eq(String::from("08:10"))
                .ne(String::from("10:27"))
                .gt(String::from("19:41"))
                .lt(String::from("03:01"))
                .ge(String::from("00:00")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: Some(String::from("10:27")),
                gt: Some(String::from("19:41")),
                lt: Some(String::from("03:01")),
                ge: Some(String::from("00:00")),
                le: None,
            }
        );
        assert_eq!(
            TimeValidation::default()
                .required()
                .eq(String::from("08:10"))
                .ne(String::from("10:27"))
                .gt(String::from("19:41"))
                .lt(String::from("03:01"))
                .ge(String::from("00:00"))
                .le(String::from("01:01")),
            TimeValidation {
                required: true,
                eq: Some(String::from("08:10")),
                ne: Some(String::from("10:27")),
                gt: Some(String::from("19:41")),
                lt: Some(String::from("03:01")),
                ge: Some(String::from("00:00")),
                le: Some(String::from("01:01")),
            }
        );
    }
}
