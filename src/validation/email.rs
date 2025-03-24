#[derive(Debug, PartialEq, Clone)]
pub struct EmailValidation {
    pub required: bool,
}

impl Default for EmailValidation {
    fn default() -> Self {
        EmailValidation { required: false }
    }
}

impl EmailValidation {
    pub fn required(self) -> Self {
        EmailValidation { required: true }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert_eq!(EmailValidation::default(), EmailValidation { required: false });
        assert_eq!(EmailValidation::default().required(), EmailValidation { required: true });
    }
}
