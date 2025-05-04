#[derive(Debug, PartialEq, Clone)]
pub struct EmailSchema {
    pub required: bool,
}

impl Default for EmailSchema {
    fn default() -> Self {
        EmailSchema { required: true }
    }
}

impl EmailSchema {
    pub fn optional(self) -> Self {
        EmailSchema { required: false }
    }
}

#[cfg(test)]
mod tests {
    use super::EmailSchema;

    #[test]
    fn email_validation() {
        assert_eq!(EmailSchema::default(), EmailSchema { required: true });
        assert_eq!(EmailSchema::default().optional(), EmailSchema { required: false });
    }
}
