#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub eq: Option<bool>,
    pub ne: Option<bool>,
}

impl Default for BoolValidation {
    fn default() -> Self {
        BoolValidation { required: false, eq: None, ne: None }
    }
}

impl BoolValidation {
    pub fn required(&self) -> Self {
        BoolValidation { required: true, eq: self.eq, ne: self.ne }
    }

    pub fn eq(&self, eq: bool) -> Self {
        BoolValidation { required: self.required, eq: Some(eq), ne: self.ne }
    }
}
