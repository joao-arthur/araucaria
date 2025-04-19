#[derive(Debug, PartialEq, Clone)]
pub enum EnumValues {
    USize(Vec<usize>),
    ISize(Vec<isize>),
    Str(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumValidation {
    pub required: bool,
    pub values: EnumValues,
}

impl EnumValidation {
    pub fn optional(self) -> Self {
        EnumValidation { required: false, ..self }
    }
}

impl From<Vec<usize>> for EnumValidation {
    fn from(value: Vec<usize>) -> Self {
        EnumValidation { required: true, values: EnumValues::USize(value) }
    }
}

impl From<Vec<isize>> for EnumValidation {
    fn from(value: Vec<isize>) -> Self {
        EnumValidation { required: true, values: EnumValues::ISize(value) }
    }
}

impl From<Vec<String>> for EnumValidation {
    fn from(value: Vec<String>) -> Self {
        EnumValidation { required: true, values: EnumValues::Str(value) }
    }
}

#[cfg(test)]
mod test {
    use super::{EnumValidation, EnumValues};

    #[test]
    fn test_enum_validation() {
        let value_u: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let value_i: Vec<isize> = vec![-2, -1, 0, 1, 2];
        let value_str: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        assert_eq!(EnumValidation::from(value_u.clone()), EnumValidation { required: true, values: EnumValues::USize(value_u.clone()) });
        assert_eq!(EnumValidation::from(value_i.clone()), EnumValidation { required: true, values: EnumValues::ISize(value_i.clone()) });
        assert_eq!(EnumValidation::from(value_str.clone()), EnumValidation { required: true, values: EnumValues::Str(value_str.clone()) });
    }

    #[test]
    fn test_enum_validation_optional() {
        let value_u: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let value_i: Vec<isize> = vec![-2, -1, 0, 1, 2];
        let value_str: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        assert_eq!(EnumValidation::from(value_u.clone()).optional(), EnumValidation { required: false, values: EnumValues::USize(value_u.clone()) });
        assert_eq!(EnumValidation::from(value_i.clone()).optional(), EnumValidation { required: false, values: EnumValues::ISize(value_i.clone()) });
        assert_eq!(
            EnumValidation::from(value_str.clone()).optional(),
            EnumValidation { required: false, values: EnumValues::Str(value_str.clone()) }
        );
    }
}
