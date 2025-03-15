use crate::{
    error::{Err, ErrWrapper},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> Option<ErrWrapper> {
    let mut base = vec![];
    match value {
        Value::Bool(value) => {
            let mut base = vec![];
            if let Some(eq_v) = validation.eq {
                if value != &eq_v {
                    base.push(Err::Eq(eq_v));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(Err::Bool);
                base.push(Err::Required);
                if let Some(eq_v) = validation.eq {
                    base.push(Err::Eq(eq_v));
                }
            }
        }
        _ => {
            let mut base = vec![Err::Bool];
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(eq_v));
            }
        }
    }
    if !base.is_empty() {
        Some(ErrWrapper::Arr(base))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_validate_bool_default_ok() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::None), None);
    }

    #[test]
    fn test_validate_bool_required_ok() {
        let v = BoolValidation::default().required();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
    }

    #[test]
    fn test_validate_bool_eq_ok() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::None), None);
    }

    #[test]
    fn test_validate_bool_required_eq_ok() {
        let v = BoolValidation::default().required().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
    }

    #[test]
    fn test_validate_bool_default_err() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate_bool(&v, &Value::NumI(1)), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate_bool(&v, &Value::NumF(1.0)), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate_bool(&v, &Value::Str(String::from("hello"))), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate_bool(&v, &Value::Arr(vec![Value::Bool(false)])), Some(ErrWrapper::Arr(vec![Err::Bool])));
        assert_eq!(validate_bool(&v, &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))), Some(ErrWrapper::Arr(vec![Err::Bool])));
    }

    #[test]
    fn test_validate_bool_required_err() {
        let v = BoolValidation::default().required();
        assert_eq!(validate_bool(&v, &Value::None), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Required])));
        assert_eq!(validate_bool(&v, &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool])));
    }

    #[test]
    fn test_validate_bool_eq_err() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Some(ErrWrapper::Arr(vec![Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
    }

    #[test]
    fn test_validate_bool_required_eq_err() {
        let v = BoolValidation::default().required().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Some(ErrWrapper::Arr(vec![Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::None), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Required, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::NumU(1)), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::NumI(1)), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::NumF(1.0)), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::Str(String::from("hello"))), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::Arr(vec![Value::Bool(false)])), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
        assert_eq!(validate_bool(&v, &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))), Some(ErrWrapper::Arr(vec![Err::Bool, Err::Eq(false)])));
    }
}
