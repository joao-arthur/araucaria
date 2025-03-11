use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BoolValidation {
    pub required: bool,
    pub eq: Option<bool>,
    pub ne: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub validation: HashMap<&'static str, Validation>,
    pub required: bool,
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
    Obj,
    Eq(bool),
    Ne(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationResErr {
    Arr(Vec<ValidationErr>),
    Obj(HashMap<String, ValidationResErr>),
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

pub fn validate(validation: &Validation, value: &Value) -> ValidationResErr {
    match validation {
        Validation::Bool(v) => match value {
            Value::Bool(value) => {
                let mut base = vec![];
                if let Some(eq_v) = v.eq {
                    if value != &eq_v {
                        base.push(ValidationErr::Eq(eq_v));
                    }
                }
                ValidationResErr::Arr(base)
            }
            Value::None => {
                let mut base = vec![];
                if v.required {
                    base.push(ValidationErr::Bool);
                    base.push(ValidationErr::Required);
                    if let Some(eq_v) = v.eq {
                        base.push(ValidationErr::Eq(eq_v));
                    }
                }
                ValidationResErr::Arr(base)
            }
            _ => {
                let mut base = vec![ValidationErr::Bool];
                if let Some(eq_v) = v.eq {
                    base.push(ValidationErr::Eq(eq_v));
                }
                ValidationResErr::Arr(base)
            }
        },
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                return ValidationResErr::Obj(
                    v.validation
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (String::from(k.clone()), validate(&v, value.get(k.clone()).unwrap_or(&Value::None))))
                        .collect()
                )
            },
            Value::None => {
                if v.required {
                    ValidationResErr::Obj(v.validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .collect())
                } else {
                    ValidationResErr::Obj(HashMap::new())
                }
            }
            _ =>   ValidationResErr::Obj(v.validation
                .clone()
                .into_iter()
                .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                .collect()),
        },
    }
}

#[cfg(test)]
mod test {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn test_bool_default() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(false)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(true)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::None),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::Num(Some(1), Some(1), Some(1.0))),
            ValidationResErr::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_bool_required() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(false)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(true)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::None),
            ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Required])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::Num(Some(1), Some(1), Some(1.0))),
            ValidationResErr::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_bool_eq() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(false)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(true)),
            ValidationResErr::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::None),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Num(Some(1), Some(1), Some(1.0))),
            ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }

    #[test]
    fn test_bool_required_eq() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(false)),
            ValidationResErr::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(true)),
            ValidationResErr::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::None),
            ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Required, ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Num(Some(1), Some(1), Some(1.0))),
            ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation {
                    validation: HashMap::from([
                        ("is", Validation::Bool(BoolValidation::default().required().eq(false))),
                    ]),
                    required: false
                }),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]),)
            ),
            ValidationResErr::Obj(HashMap::from([(String::from("is"), ValidationResErr::Arr(vec![]))]))
        );

        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation {
                    validation: HashMap::from([
                        ("is", Validation::Bool(BoolValidation::default().required().eq(false))),
                    ]),
                    required: false
                }),
                &Value::None
            ),
            ValidationResErr::Obj(HashMap::new()),
        );
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation {
                    validation: HashMap::from([
                        ("is", Validation::Bool(BoolValidation::default().required().eq(false))),
                    ]),
                    required: true
                }),
                &Value::None
            ),
            ValidationResErr::Obj(HashMap::from([(String::from("is"), ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Required, ValidationErr::Eq(false)]))])),
        );





        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation {
                    validation: HashMap::from([
                        ("is", Validation::Bool(BoolValidation::default().required().eq(false))),
                    ]),
                    required: false
                }),
                &Value::Bool(false)
            ),
            ValidationResErr::Obj(HashMap::from([
                (String::from("is"), ValidationResErr::Arr(vec![ValidationErr::Bool, ValidationErr::Required, ValidationErr::Eq(false)]))
            ]))
        );
    }
}
