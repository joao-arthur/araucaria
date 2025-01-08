use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    required: bool,
    eq: Option<bool>,
    ne: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    validation: HashMap<&'static str, Validation>,
    required: bool,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    Bool(BoolValidation),
    Obj(ObjValidation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    Bool,
    Eq(bool),
    Ne(bool),
    Obj,
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

fn validate(validation: &Validation, value: &Value) -> Vec<ValidationErr> {
    match validation {
        Validation::Bool(v) => match value {
            Value::Bool(value) => {
                if let Some(eq_v) = v.eq {
                    if value != &eq_v {
                        return vec![ValidationErr::Eq(eq_v)];
                    }
                }
                vec![]
            }
            Value::None => {
                if v.required {
                    vec![ValidationErr::Required]
                } else {
                    vec![]
                }
            }
            _ => vec![ValidationErr::Bool],
        },
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                return vec![];
            }
            Value::None => {
                if v.required {
                    vec![ValidationErr::Required]
                } else {
                    vec![]
                }
            }
            _ => vec![ValidationErr::Obj],
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bool_value_ok() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(false)), []);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(true)), []);
    }

    #[test]
    fn test_bool_required_ok() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(false)), []);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(true)), []);
    }

    #[test]
    fn test_bool_eq_ok() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(false)), []);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(true)), &Value::Bool(true)), []);
    }

    #[test]
    fn test_lib() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(true)), [ValidationErr::Eq(false)]);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(true)), &Value::Bool(false)), [ValidationErr::Eq(true)]);

        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::None), []);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::None), [ValidationErr::Required]);

        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::Num(Some(1), Some(1), Some(1.0))), [ValidationErr::Bool]);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::Num(Some(1), Some(1), Some(1.0))), [ValidationErr::Bool]);
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from(
                        [
                            ("is", Validation::Bool(BoolValidation::default().required().eq(false)))
                        ]
                    ),
                    required: false
                }),
                &Value::Obj(
                    HashMap::from(
                        [
                            (String::from("is"), Value::Bool(false))
                        ]
                    ),
                
                )
            ),
            []
        );


        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from(
                        [
                            ("is", Validation::Bool(BoolValidation::default().required().eq(false)))
                        ]
                    ),
                    required: false
                }),
                &Value::Bool(false)
            ),
            [ValidationErr::Obj]
        );
    }
}
