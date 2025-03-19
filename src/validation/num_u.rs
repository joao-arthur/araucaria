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
        NumUValidation {
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

impl NumUValidation {
    pub fn required(self) -> Self {
        NumUValidation {
            required: true,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn eq(self, value: u64) -> Self {
        NumUValidation {
            required: self.required,
            eq: Some(value),
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ne(self, value: u64) -> Self {
        NumUValidation {
            required: self.required,
            eq: self.eq,
            ne: Some(value),
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn gt(self, value: u64) -> Self {
        NumUValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: Some(value),
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn lt(self, value: u64) -> Self {
        NumUValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: Some(value),
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ge(self, value: u64) -> Self {
        NumUValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: Some(value),
            le: self.le,
        }
    }

    pub fn le(self, value: u64) -> Self {
        NumUValidation {
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
    fn test_num_u_validation() {
        assert_eq!(
            NumUValidation::default(),
            NumUValidation {
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
            NumUValidation::default().required(),
            NumUValidation {
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
            NumUValidation::default().required().eq(1),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumUValidation::default().required().eq(1).ne(2),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: Some(2),
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumUValidation::default().required().eq(1).ne(2).gt(3),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: Some(2),
                gt: Some(3),
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumUValidation::default().required().eq(1).ne(2).gt(3).lt(4),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: Some(2),
                gt: Some(3),
                lt: Some(4),
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumUValidation::default().required().eq(1).ne(2).gt(3).lt(4).ge(5),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: Some(2),
                gt: Some(3),
                lt: Some(4),
                ge: Some(5),
                le: None,
            }
        );
        assert_eq!(
            NumUValidation::default().required().eq(1).ne(2).gt(3).lt(4).ge(5).le(6),
            NumUValidation {
                required: true,
                eq: Some(1),
                ne: Some(2),
                gt: Some(3),
                lt: Some(4),
                ge: Some(5),
                le: Some(6),
            }
        );
    }
}
