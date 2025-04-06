#[derive(Debug, PartialEq, Clone)]
pub struct EmailValidation {
    pub required: bool,
}

impl Default for EmailValidation {
    fn default() -> Self {
        EmailValidation { required: true }
    }
}

impl EmailValidation {
    pub fn optional(self) -> Self {
        EmailValidation { required: false }
    }
}

#[cfg(test)]
mod test {
    use super::EmailValidation;

    #[test]
    fn test_email_validation() {
        assert_eq!(EmailValidation::default(), EmailValidation { required: true });
        assert_eq!(EmailValidation::default().optional(), EmailValidation { required: false });
    }
}
