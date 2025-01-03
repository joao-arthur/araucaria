use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    Bool {
        required: bool
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Bool
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    Bool(bool),
    Num(Option<u64>, Option<i64>, Option<f64>),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
}

fn validate(validation: &Validation, value: &Value) -> Option<Validation> {
    match validation {
        Validation::Bool { required } => {
            match value {
                Value::Bool(value) => None,
                Value::None => if *required {
                    Some(validation.clone())
                } else {
                    None
                },
                _ => Some(validation.clone()),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lib() {
        assert_eq!(validate(&Validation::Bool { required: true }, &Value::Bool(false)), None);
        assert_eq!(validate(&Validation::Bool { required: true }, &Value::Bool(true)), None);

        assert_eq!(validate(&Validation::Bool { required: false }, &Value::None), None);
        assert_eq!(validate(&Validation::Bool { required: true }, &Value::None), Some(Validation::Bool { required: true }));

        assert_eq!(validate(&Validation::Bool { required: true }, &Value::Num(Some(1), Some(1), Some(1.0))), Some(Validation::Bool { required: true }));
        assert_eq!(validate(&Validation::Bool { required: false }, &Value::Num(Some(1), Some(1), Some(1.0))), Some(Validation::Bool { required: false }));
    }
}