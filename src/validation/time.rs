use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub struct TimeValidation {
    pub required: bool,
    pub operation: Option<Operation<String>>,
}

impl Default for TimeValidation {
    fn default() -> Self {
        TimeValidation { required: true, operation: None }
    }
}

impl TimeValidation {
    pub fn optional(self) -> Self {
        TimeValidation { required: false, ..self }
    }

    pub fn eq(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Eq(value)), ..self }
    }

    pub fn ne(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Ne(value)), ..self }
    }

    pub fn gt(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Gt(value)), ..self }
    }

    pub fn ge(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Ge(value)), ..self }
    }

    pub fn lt(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Lt(value)), ..self }
    }

    pub fn le(self, value: String) -> Self {
        TimeValidation { operation: Some(Operation::Le(value)), ..self }
    }

    pub fn btwn(self, value_a: String, value_b: String) -> Self {
        TimeValidation { operation: Some(Operation::Btwn(value_a, value_b)), ..self }
    }
}

#[cfg(test)]
mod test {
    use crate::operation::Operation;

    use super::TimeValidation;

    #[test]
    fn test_time_validation() {
        assert_eq!(TimeValidation::default(), TimeValidation { required: true, operation: None });
        assert_eq!(TimeValidation::default().optional(), TimeValidation { required: false, operation: None });
        assert_eq!(TimeValidation::default().eq(String::from("08:10")), TimeValidation { required: true, operation: Some(Operation::Eq(String::from("08:10"))) });
        assert_eq!(TimeValidation::default().ne(String::from("10:27")), TimeValidation { required: true, operation: Some(Operation::Ne(String::from("10:27"))) });
        assert_eq!(TimeValidation::default().gt(String::from("19:41")), TimeValidation { required: true, operation: Some(Operation::Gt(String::from("19:41"))) });
        assert_eq!(TimeValidation::default().ge(String::from("03:01")), TimeValidation { required: true, operation: Some(Operation::Ge(String::from("03:01"))) });
        assert_eq!(TimeValidation::default().lt(String::from("00:00")), TimeValidation { required: true, operation: Some(Operation::Lt(String::from("00:00"))) });
        assert_eq!(TimeValidation::default().le(String::from("01:01")), TimeValidation { required: true, operation: Some(Operation::Le(String::from("01:01"))) });
        assert_eq!(TimeValidation::default().btwn(String::from("00:00"), String::from("23:59")), TimeValidation { required: true, operation: Some(Operation::Btwn(String::from("00:00"), String::from("23:59"))) });
    }
}
