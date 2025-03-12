use std::collections::HashMap;

use error::{ValidationErr, ValidationErrWrapper};
use validate::bool::validate_bool;
use validation::Validation;
use value::Value;

pub mod error;
pub mod validate;
pub mod validation;
pub mod value;

pub fn validate(validation: &Validation, value: &Value) -> ValidationErrWrapper {
    match validation {
        Validation::Bool(v) => validate_bool(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                return ValidationErrWrapper::Obj(
                    v.validation
                        .clone()
                        .into_iter()
                        .map(|(k, v)| {
                            (
                                String::from(k.clone()),
                                validate(&v, value.get(k.clone()).unwrap_or(&Value::None)),
                            )
                        })
                        .collect(),
                )
            }
            Value::None => {
                if v.required {
                    ValidationErrWrapper::Obj(
                        v.validation
                            .clone()
                            .into_iter()
                            .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                            .collect(),
                    )
                } else {
                    ValidationErrWrapper::Obj(HashMap::new())
                }
            }
            _ => ValidationErrWrapper::Obj(
                v.validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .collect(),
            ),
        },
    }
}

#[cfg(test)]
mod test {
    use crate::validation::{bool::BoolValidation, ObjValidation};

    use super::*;

    #[test]
    fn test_bool_default() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::None),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_bool_required() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::None),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Required])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_bool_eq() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::None),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }

    #[test]
    fn test_bool_required_eq() {
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::Bool(false)
            ),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::Bool(true)
            ),
            ValidationErrWrapper::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::None
            ),
            ValidationErrWrapper::Arr(vec![
                ValidationErr::Bool,
                ValidationErr::Required,
                ValidationErr::Eq(false)
            ])
        );
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::NumU(1)
            ),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    ),]),
                    required: false
                }),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]),)
            ),
            ValidationErrWrapper::Obj(HashMap::from([(
                String::from("is"),
                ValidationErrWrapper::Arr(vec![])
            )]))
        );

        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    ),]),
                    required: false
                }),
                &Value::None
            ),
            ValidationErrWrapper::Obj(HashMap::new()),
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    ),]),
                    required: true
                }),
                &Value::None
            ),
            ValidationErrWrapper::Obj(HashMap::from([(
                String::from("is"),
                ValidationErrWrapper::Arr(vec![
                    ValidationErr::Bool,
                    ValidationErr::Required,
                    ValidationErr::Eq(false)
                ])
            )])),
        );

        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    ),]),
                    required: false
                }),
                &Value::Bool(false)
            ),
            ValidationErrWrapper::Obj(HashMap::from([(
                String::from("is"),
                ValidationErrWrapper::Arr(vec![
                    ValidationErr::Bool,
                    ValidationErr::Required,
                    ValidationErr::Eq(false)
                ])
            )]))
        );
    }
}
