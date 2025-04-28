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
    fn from(values: Vec<usize>) -> Self {
        EnumValidation { required: true, values: EnumValues::USize(values) }
    }
}

impl From<Vec<isize>> for EnumValidation {
    fn from(values: Vec<isize>) -> Self {
        EnumValidation { required: true, values: EnumValues::ISize(values) }
    }
}

impl From<Vec<String>> for EnumValidation {
    fn from(values: Vec<String>) -> Self {
        EnumValidation { required: true, values: EnumValues::Str(values) }
    }
}

impl From<Vec<&str>> for EnumValidation {
    fn from(values: Vec<&str>) -> Self {
        EnumValidation { required: true, values: EnumValues::Str(values.iter().map(|value| value.to_string()).collect()) }
    }
}

impl<const N: usize> From<[usize; N]> for EnumValidation {
    fn from(values: [usize; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::USize(values.to_vec()) }
    }
}

impl<const N: usize> From<[isize; N]> for EnumValidation {
    fn from(values: [isize; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::ISize(values.to_vec()) }
    }
}

impl<const N: usize> From<[String; N]> for EnumValidation {
    fn from(values: [String; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::Str(values.to_vec()) }
    }
}

impl<const N: usize> From<[&str; N]> for EnumValidation {
    fn from(values: [&str; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::Str(values.iter().map(|value| value.to_string()).collect()) }
    }
}

#[cfg(test)]
mod tests {
    use super::{EnumValidation, EnumValues};

    #[test]
    fn enum_from() {
        let slice_u: [usize; 6] = [0, 1, 2, 3, 4, 5];
        let slice_i: [isize; 5] = [-2, -1, 0, 1, 2];
        let slice_string: [String; 3] = ["APPLE".to_string(), "GRAPE".to_string(), "PEAR".to_string()];
        let slice_str: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];

        let vec_u: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let vec_i: Vec<isize> = vec![-2, -1, 0, 1, 2];
        let vec_string: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        let vec_str: Vec<&str> = vec!["APPLE", "GRAPE", "PEAR"];

        assert_eq!(EnumValidation::from(slice_u), EnumValidation { required: true, values: EnumValues::USize(vec_u.clone()) });
        assert_eq!(EnumValidation::from(slice_i), EnumValidation { required: true, values: EnumValues::ISize(vec_i.clone()) });
        assert_eq!(EnumValidation::from(slice_string), EnumValidation { required: true, values: EnumValues::Str(vec_string.clone()) });
        assert_eq!(EnumValidation::from(slice_str), EnumValidation { required: true, values: EnumValues::Str(vec_string.clone()) });

        assert_eq!(EnumValidation::from(vec_u.clone()), EnumValidation { required: true, values: EnumValues::USize(vec_u.clone()) });
        assert_eq!(EnumValidation::from(vec_i.clone()), EnumValidation { required: true, values: EnumValues::ISize(vec_i.clone()) });
        assert_eq!(EnumValidation::from(vec_string.clone()), EnumValidation { required: true, values: EnumValues::Str(vec_string.clone()) });
        assert_eq!(EnumValidation::from(vec_str), EnumValidation { required: true, values: EnumValues::Str(vec_string.clone()) });
    }

    #[test]
    fn enum_validation_optional() {
        let slice_str: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];
        let vec_str: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        assert_eq!(EnumValidation::from(slice_str).optional(), EnumValidation { required: false, values: EnumValues::Str(vec_str) });
    }
}
