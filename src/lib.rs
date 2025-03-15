use std::collections::HashMap;

use error::{Err, ErrWrapper};
use validate::bool::validate_bool;
use validation::Validation;
use value::Value;

pub mod error;
pub mod operation;
//pub mod validate;
//pub mod validation;
pub mod value;

#[cfg(test)]
mod stub;
/*
pub fn validate(validation: &Validation, value: &Value) -> Option<ErrWrapper> {
    match validation {
        Validation::Bool(v) => validate_bool(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                return Some(ErrWrapper::Obj(
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
                ))
            }
            Value::None => {
                if v.required {
                    Some(ErrWrapper::Obj(
                        v.validation
                            .clone()
                            .into_iter()
                            .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            _ => Some(ErrWrapper::Obj(
                v.validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .collect()),
            ),
        },
    }
}

#[cfg(test)]
mod test {
    use crate::validation::{bool::BoolValidation, ObjValidation};

    use super::*;

    #[test]
    fn test_bool_none() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(false)), None);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::Bool(false)), None);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(false)), None);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(false)), None);
    }

    #[test]
    fn test_bool_some() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::None), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Required])));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(true)), Some(ErrWrapper::Arr(vec![Err::Eq(false)])));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(true)), Some(ErrWrapper::Arr(vec![Err::Eq(false)])));
    }

    #[test]
    fn test_obj_ok() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is", Validation::Bool(BoolValidation::default().required().eq(false)))]), required: false}),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))
            ),
            None
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)))]), required: false}),
                &Value::None
            ),
            None
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is", Validation::Bool(BoolValidation::default().required().eq(false)))]), required: true}),
                &Value::None
            ),
            Some(ErrWrapper::Obj(HashMap::from([(String::from("is"), ErrWrapper::Arr(vec![Err::Bool, Err::Required, Err::Eq(false)]))])))
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)) )]), required: false }),
                &Value::Bool(false)
            ),
            Some(ErrWrapper::Obj(HashMap::from([(String::from("is"), ErrWrapper::Arr(vec![Err::Bool, Err::Required, Err::Eq(false)]))])))
        );
    }

    #[test]
    fn test_obj_err() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is", Validation::Bool(BoolValidation::default().required().eq(false)))]), required: false}),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))
            ),
            None
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)))]), required: false}),
                &Value::None
            ),
            None
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is", Validation::Bool(BoolValidation::default().required().eq(false)))]), required: true}),
                &Value::None
            ),
            Some(ErrWrapper::Obj(HashMap::from([(String::from("is"), ErrWrapper::Arr(vec![Err::Bool, Err::Required, Err::Eq(false)]))])))
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)) )]), required: false }),
                &Value::Bool(false)
            ),
            Some(ErrWrapper::Obj(HashMap::from([(String::from("is"), ErrWrapper::Arr(vec![Err::Bool, Err::Required, Err::Eq(false)]))])))
        );
    }
}
 */
