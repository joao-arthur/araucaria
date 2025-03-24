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
        DateValidation {
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

impl DateValidation {
    pub fn required(self) -> Self {
        DateValidation {
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
        DateValidation {
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
        DateValidation {
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
        DateValidation {
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
        DateValidation {
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
        DateValidation {
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
        DateValidation {
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
    fn test_date_validation() {
        assert_eq!(
            DateValidation::default(),
            DateValidation {
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
            DateValidation::default().required(),
            DateValidation {
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
            DateValidation::default().required().eq(String::from("2026-08-12")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateValidation::default()
                .required()
                .eq(String::from("2026-08-12"))
                .ne(String::from("2027-08-02")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: Some(String::from("2027-08-02")),
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateValidation::default()
                .required()
                .eq(String::from("2026-08-12"))
                .ne(String::from("2027-08-02"))
                .gt(String::from("2028-07-22")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: Some(String::from("2027-08-02")),
                gt: Some(String::from("2028-07-22")),
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateValidation::default()
                .required()
                .eq(String::from("2026-08-12"))
                .ne(String::from("2027-08-02"))
                .gt(String::from("2028-07-22"))
                .lt(String::from("2030-11-25")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: Some(String::from("2027-08-02")),
                gt: Some(String::from("2028-07-22")),
                lt: Some(String::from("2030-11-25")),
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            DateValidation::default()
                .required()
                .eq(String::from("2026-08-12"))
                .ne(String::from("2027-08-02"))
                .gt(String::from("2028-07-22"))
                .lt(String::from("2030-11-25"))
                .ge(String::from("2031-11-14")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: Some(String::from("2027-08-02")),
                gt: Some(String::from("2028-07-22")),
                lt: Some(String::from("2030-11-25")),
                ge: Some(String::from("2031-11-14")),
                le: None,
            }
        );
        assert_eq!(
            DateValidation::default()
                .required()
                .eq(String::from("2026-08-12"))
                .ne(String::from("2027-08-02"))
                .gt(String::from("2028-07-22"))
                .lt(String::from("2030-11-25"))
                .ge(String::from("2031-11-14"))
                .le(String::from("2033-03-30")),
            DateValidation {
                required: true,
                eq: Some(String::from("2026-08-12")),
                ne: Some(String::from("2027-08-02")),
                gt: Some(String::from("2028-07-22")),
                lt: Some(String::from("2030-11-25")),
                ge: Some(String::from("2031-11-14")),
                le: Some(String::from("2033-03-30")),
            }
        );
    }
}
