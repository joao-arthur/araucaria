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
    use std::sync::LazyLock;

    use super::{EnumValidation, EnumValues};

    const SLICE_U: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const SLICE_I: [isize; 5] = [-2, -1, 0, 1, 2];
    const SLICE_STR: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];
    static SLICE_STRING: LazyLock<[String; 3]> = LazyLock::new(|| ["APPLE".into(), "GRAPE".into(), "PEAR".into()]);

    static VEC_U: LazyLock<Vec<usize>> = LazyLock::new(|| vec![0, 1, 2, 3, 4, 5]);
    static VEC_I: LazyLock<Vec<isize>> = LazyLock::new(|| vec![-2, -1, 0, 1, 2]);
    static VEC_STR: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["APPLE", "GRAPE", "PEAR"]);
    static VEC_STRING: LazyLock<Vec<String>> = LazyLock::new(|| vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()]);

    #[test]
    fn enum_values_from() {
        assert_eq!(EnumValues::from(SLICE_U.clone()), EnumValues::USize(VEC_U.clone()));
        assert_eq!(EnumValues::from(VEC_U.clone()), EnumValues::USize(VEC_U.clone()));
        assert_eq!(EnumValues::from(SLICE_I.clone()), EnumValues::ISize(VEC_I.clone()));
        assert_eq!(EnumValues::from(VEC_I.clone()), EnumValues::ISize(VEC_I.clone()));
        assert_eq!(EnumValues::from(SLICE_STR.clone()), EnumValues::Str(VEC_STRING.clone()));
        assert_eq!(EnumValues::from(VEC_STR.clone()), EnumValues::Str(VEC_STRING.clone()));
        assert_eq!(EnumValues::from(SLICE_STRING.clone()), EnumValues::Str(VEC_STRING.clone()));
        assert_eq!(EnumValues::from(VEC_STRING.clone()), EnumValues::Str(VEC_STRING.clone()));
    }

    #[test]
    fn enum_values_to_string() {
        assert_eq!(EnumValues::USize(VEC_U.clone()).to_string(), r#"[ 0, 1, 2, 3, 4, 5 ]"#.to_string());
        assert_eq!(EnumValues::ISize(VEC_I.clone()).to_string(), r#"[ -2, -1, 0, 1, 2 ]"#.to_string());
        assert_eq!(EnumValues::Str(VEC_STRING.clone()).to_string(), r#"[ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn enum_validation_from() {
        assert_eq!(EnumValidation::from(SLICE_U.clone()), EnumValidation { required: true, values: EnumValues::USize(VEC_U.clone()) });
        assert_eq!(EnumValidation::from(VEC_U.clone()), EnumValidation { required: true, values: EnumValues::USize(VEC_U.clone()) });
        assert_eq!(EnumValidation::from(SLICE_I.clone()), EnumValidation { required: true, values: EnumValues::ISize(VEC_I.clone()) });
        assert_eq!(EnumValidation::from(VEC_I.clone()), EnumValidation { required: true, values: EnumValues::ISize(VEC_I.clone()) });
        assert_eq!(EnumValidation::from(SLICE_STR.clone()), EnumValidation { required: true, values: EnumValues::Str(VEC_STRING.clone()) });
        assert_eq!(EnumValidation::from(VEC_STR.clone()), EnumValidation { required: true, values: EnumValues::Str(VEC_STRING.clone()) });
        assert_eq!(EnumValidation::from(SLICE_STRING.clone()), EnumValidation { required: true, values: EnumValues::Str(VEC_STRING.clone()) });
        assert_eq!(EnumValidation::from(VEC_STRING.clone()), EnumValidation { required: true, values: EnumValues::Str(VEC_STRING.clone()) });
    }

    #[test]
    fn enum_validation_optional() {
        let slice_str: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];
        let vec_str: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];
        assert_eq!(EnumValidation::from(slice_str).optional(), EnumValidation { required: false, values: EnumValues::Str(vec_str) });
    }
}
