use std::collections::HashMap;

use crate::error::{Err, ErrWrap};
use crate::validate::bool::validate_bool;
use crate::validation::Validation;
use crate::value::Value;

pub mod bool;

pub fn validate(validation: &Validation, value: &Value) -> Option<ErrWrap> {
    match validation {
        Validation::Bool(v) => validate_bool(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                let result: HashMap<String, ErrWrap> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, value.get(k.clone()).unwrap_or(&Value::None))))
                    .filter(|(k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect();
                if result.is_empty() {
                    None
                } else {
                    Some(ErrWrap::Obj(result))
                }
            }
            Value::None => {
                if v.required {
                    let result: HashMap<String, ErrWrap> = v
                        .validation
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                        .filter(|(k, v)| v.is_some())
                        .map(|(k, v)| (k, v.unwrap()))
                        .collect();
                    if result.is_empty() {
                        None
                    } else {
                        Some(ErrWrap::Obj(result))
                    }
                } else {
                    None
                }
            }
            _ => {
                let result: HashMap<String, ErrWrap> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .filter(|(k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect();
                if result.is_empty() {
                    None
                } else {
                    Some(ErrWrap::Obj(result))
                }
            }
        },
    }
}

#[cfg(test)]
mod test {
    use crate::validation::{bool::BoolValidation, ObjValidation};

    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::Bool(false)), None);
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required().eq(false)), &Value::None), ErrWrap::arr([Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]));
    }

    #[test]
    fn test_bool_some() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::NumU(1)), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().required()), &Value::None), ErrWrap::arr([Err::Bool, Err::Required]));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(true)), ErrWrap::arr([Err::Eq(Value::Bool(false))]));
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
            Some(ErrWrap::Obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]))])))
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)) )]), required: false }),
                &Value::Bool(false)
            ),
            Some(ErrWrap::Obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]))])))
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
            Some(ErrWrap::Obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]))])))
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation { validation: HashMap::from([("is",Validation::Bool(BoolValidation::default().required().eq(false)) )]), required: false }),
                &Value::Bool(false)
            ),
            Some(ErrWrap::Obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]))])))
        );
    }
}
 