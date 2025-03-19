#[derive(Debug, PartialEq, Clone)]
pub struct StrValidation {
    pub required: bool,
    pub eq: Option<String>,
    pub ne: Option<String>,
}

impl Default for StrValidation {
    fn default() -> Self {
        StrValidation { required: false, eq: None, ne: None }
    }
}

impl StrValidation {
    pub fn required(self) -> Self {
        StrValidation { required: true, eq: self.eq, ne: self.ne }
    }

    pub fn eq(self, value: String) -> Self {
        StrValidation { required: self.required, eq: Some(value), ne: self.ne }
    }

    pub fn ne(self, value: String) -> Self {
        StrValidation { required: self.required, eq: self.eq, ne: Some(value) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_validation() {
        assert_eq!(
            StrValidation::default(),
            StrValidation { required: false, eq: None, ne: None }
        );
        assert_eq!(
            StrValidation::default().required(),
            StrValidation { required: true, eq: None, ne: None }
        );
        assert_eq!(
            StrValidation::default().required().eq(String::from("Avalon")),
            StrValidation { required: true, eq: Some(String::from("Avalon")), ne: None }
        );
        assert_eq!(
            StrValidation::default().required().eq(String::from("Avalon")).ne(String::from("Mu")),
            StrValidation { required: true, eq: Some(String::from("Avalon")), ne: Some(String::from("Mu")) }
        );
    }
}
