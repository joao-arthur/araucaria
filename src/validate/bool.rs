use crate::{
    error::{Err, ErrWrap},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> Option<ErrWrap> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(eq_v) = validation.eq {
                if bool_value != &eq_v {
                    base.push(Err::Eq(eq_v));
                }
            }
            if let Some(ne_v) = validation.ne {
                if bool_value == &ne_v {
                    base.push(Err::Ne(ne_v));
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
                if let Some(ne_v) = validation.ne {
                    base.push(Err::Ne(ne_v));
                }
            }
        }
        _ => {
            base.push(Err::Bool);
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(eq_v));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(ne_v));
            }
        }
    }
    if !base.is_empty() {
        Some(ErrWrap::Arr(base))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::value::stub::{
        arr_bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_validate_bool_default_none() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::None), None);
    }

    #[test]
    fn test_validate_bool_required_none() {
        let v = BoolValidation::default().required();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
    }

    #[test]
    fn test_validate_bool_eq_none() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::None), None);
    }

    #[test]
    fn test_validate_bool_ne_none() {
        let v = BoolValidation::default().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::None), None);
    }

    #[test]
    fn test_validate_bool_required_eq_none() {
        let v = BoolValidation::default().required().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
    }

    #[test]
    fn test_validate_bool_required_ne_none() {
        let v = BoolValidation::default().required().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
    }

    #[test]
    fn test_validate_bool_default_some() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &num_i_stub()), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &num_f_stub()), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &str_stub()), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &arr_bool_stub()), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &obj_stub()), ErrWrap::arr([Err::Bool]));
    }

    #[test]
    fn test_validate_bool_required_some() {
        let v = BoolValidation::default().required();
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool, Err::Required]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool]));
    }

    #[test]
    fn test_validate_bool_eq_some() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), ErrWrap::arr([Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
    }

    #[test]
    fn test_validate_bool_ne_some() {
        let v = BoolValidation::default().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), ErrWrap::arr([Err::Ne(false)]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Ne(false)]));
    }

    #[test]
    fn test_validate_bool_required_eq_some() {
        let v = BoolValidation::default().required().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), ErrWrap::arr([Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool, Err::Required, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &num_i_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &num_f_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &str_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &arr_bool_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
        assert_eq!(validate_bool(&v, &obj_stub()), ErrWrap::arr([Err::Bool, Err::Eq(false)]));
    }
}
