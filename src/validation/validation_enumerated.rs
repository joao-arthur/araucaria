#[derive(Debug, PartialEq, Clone)]
pub enum EnumValues {
    USize(Vec<usize>),
    ISize(Vec<isize>),
    Str(Vec<String>),
}

impl From<Vec<usize>> for EnumValues {
    fn from(values: Vec<usize>) -> Self {
        EnumValues::USize(values)
    }
}

impl From<Vec<isize>> for EnumValues {
    fn from(values: Vec<isize>) -> Self {
        EnumValues::ISize(values)
    }
}

impl From<Vec<String>> for EnumValues {
    fn from(values: Vec<String>) -> Self {
        EnumValues::Str(values)
    }
}

impl From<Vec<&str>> for EnumValues {
    fn from(values: Vec<&str>) -> Self {
        EnumValues::Str(values.iter().map(|value| value.to_string()).collect())
    }
}

impl<const N: usize> From<[usize; N]> for EnumValues {
    fn from(values: [usize; N]) -> Self {
        EnumValues::USize(values.to_vec())
    }
}

impl<const N: usize> From<[isize; N]> for EnumValues {
    fn from(values: [isize; N]) -> Self {
        EnumValues::ISize(values.to_vec())
    }
}

impl<const N: usize> From<[String; N]> for EnumValues {
    fn from(values: [String; N]) -> Self {
        EnumValues::Str(values.to_vec())
    }
}

impl<const N: usize> From<[&str; N]> for EnumValues {
    fn from(values: [&str; N]) -> Self {
        EnumValues::Str(values.iter().map(|value| value.to_string()).collect())
    }
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
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl From<Vec<isize>> for EnumValidation {
    fn from(values: Vec<isize>) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl From<Vec<String>> for EnumValidation {
    fn from(values: Vec<String>) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl From<Vec<&str>> for EnumValidation {
    fn from(values: Vec<&str>) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl<const N: usize> From<[usize; N]> for EnumValidation {
    fn from(values: [usize; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl<const N: usize> From<[isize; N]> for EnumValidation {
    fn from(values: [isize; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl<const N: usize> From<[String; N]> for EnumValidation {
    fn from(values: [String; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl<const N: usize> From<[&str; N]> for EnumValidation {
    fn from(values: [&str; N]) -> Self {
        EnumValidation { required: true, values: EnumValues::from(values) }
    }
}

impl std::fmt::Display for EnumValues {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            EnumValues::USize(values) => {
                let parts: Vec<String> = values.iter().map(|value| value.to_string()).collect();
                "[ ".to_string() + &parts.join(", ") + " ]"
            }
            EnumValues::ISize(values) => {
                let parts: Vec<String> = values.iter().map(|value| value.to_string()).collect();
                "[ ".to_string() + &parts.join(", ") + " ]"
            }
            EnumValues::Str(values) => {
                let parts: Vec<String> = values.iter().map(|value| "\"".to_string() + value + "\"").collect();
                "[ ".to_string() + &parts.join(", ") + " ]"
            }
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::{EnumValidation, EnumValues};

    #[test]
    fn enum_values_from() {
        let slice_u: [usize; 6] = [0, 1, 2, 3, 4, 5];
        let slice_i: [isize; 5] = [-2, -1, 0, 1, 2];
        let slice_string: [String; 3] = ["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        let slice_str: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];

        let vec_u: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let vec_i: Vec<isize> = vec![-2, -1, 0, 1, 2];
        let vec_string: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        let vec_str: Vec<&str> = vec!["APPLE", "GRAPE", "PEAR"];

        assert_eq!(EnumValues::from(slice_u), EnumValues::USize(vec_u.clone()));
        assert_eq!(EnumValues::from(slice_i), EnumValues::ISize(vec_i.clone()));
        assert_eq!(EnumValues::from(slice_string), EnumValues::Str(vec_string.clone()));
        assert_eq!(EnumValues::from(slice_str), EnumValues::Str(vec_string.clone()));

        assert_eq!(EnumValues::from(vec_u.clone()), EnumValues::USize(vec_u.clone()));
        assert_eq!(EnumValues::from(vec_i.clone()), EnumValues::ISize(vec_i.clone()));
        assert_eq!(EnumValues::from(vec_string.clone()), EnumValues::Str(vec_string.clone()));
        assert_eq!(EnumValues::from(vec_str), EnumValues::Str(vec_string.clone()));
    }

    #[test]
    fn enum_values_to_string() {
        let vec_u: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let vec_i: Vec<isize> = vec![-2, -1, 0, 1, 2];
        let vec_str: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];

        assert_eq!(EnumValues::USize(vec_u).to_string(), r#"[ 0, 1, 2, 3, 4, 5 ]"#.to_string());
        assert_eq!(EnumValues::ISize(vec_i).to_string(), r#"[ -2, -1, 0, 1, 2 ]"#.to_string());
        assert_eq!(EnumValues::Str(vec_str).to_string(), r#"[ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn enum_validation_from() {
        let slice_u: [usize; 6] = [0, 1, 2, 3, 4, 5];
        let slice_i: [isize; 5] = [-2, -1, 0, 1, 2];
        let slice_string: [String; 3] = ["APPLE".into(), "GRAPE".into(), "PEAR".into()];
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
