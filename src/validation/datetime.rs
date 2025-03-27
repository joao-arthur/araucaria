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
        DateTimeValidation {
            required: true,
            eq: None,
            ne: None,
            gt: None,
            lt: None,
            ge: None,
            le: None,
        }
    }
}

impl DateTimeValidation {
    pub fn optional(self) -> Self {
        DateTimeValidation {
            required: false,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn eq(self, value: String) -> Self {
        DateTimeValidation {
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
        DateTimeValidation {
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
        DateTimeValidation {
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
        DateTimeValidation {
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
        DateTimeValidation {
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
        DateTimeValidation {
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
    fn test_date_time_validation() {
        assert_eq!(
            DateTimeValidation::default(),
            DateTimeValidation {
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
            DateTimeValidation::default().optional(),
            DateTimeValidation {
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
            DateTimeValidation::default().eq(String::from("2026-08-12T08:10Z")),
            DateTimeValidation {
                required: true,
                eq: Some(String::from("2026-08-12T08:10Z")),
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateTimeValidation::default().ne(String::from("2027-08-02T10:27Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: Some(String::from("2027-08-02T10:27Z")),
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateTimeValidation::default().gt(String::from("2028-07-22T19:41Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: Some(String::from("2028-07-22T19:41Z")),
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateTimeValidation::default().lt(String::from("2030-11-25T03:01Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: Some(String::from("2030-11-25T03:01Z")),
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateTimeValidation::default().ge(String::from("2031-11-14T00:00Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: Some(String::from("2031-11-14T00:00Z")),
                le: None,
            }
        );
        assert_eq!(
            DateTimeValidation::default().le(String::from("2033-03-30T01:01Z")),
            DateTimeValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: Some(String::from("2033-03-30T01:01Z")),
            }
        );
    }
}
