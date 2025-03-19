use crate::{
    error::{Err, ErrWrap}, validation::num_u::NumUValidation, value::Value
};

pub fn validate_num_u(validation: &NumUValidation, value: &Value) -> Option<ErrWrap> {
    let mut base = vec![];
    match value {
        Value::NumU(num_u_value) => {
            if let Some(eq_v) = validation.eq {
                if num_u_value != &eq_v {
                    base.push(Err::Eq(Value::NumU(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if num_u_value == &ne_v {
                    base.push(Err::Ne(Value::NumU(ne_v)));
                }
            }
        }
        Value::None => {
            base.push(Err::NumU);
            if validation.required {
                base.push(Err::Required);
            }
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::NumU(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::NumU(ne_v)));
            }
        }
        _ => {
            base.push(Err::NumU);
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::NumU(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::NumU(ne_v)));
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
        arr_bool_stub, bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub
    };

    use super::*;

    #[test]
    fn test_validate_num_u_default_none() {
        let v = NumUValidation::default();
        assert_eq!(validate_num_u(&v, &Value::NumU(0)), None);
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), None);
    }

    #[test]
    fn test_validate_num_u_required_none() {
        let v = NumUValidation::default().required();
        assert_eq!(validate_num_u(&v, &Value::NumU(0)), None);
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), None);
    }

    #[test]
    fn test_validate_num_u_eq_none() {
        let v = NumUValidation::default().eq(42);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), None);
    }

    #[test]
    fn test_validate_num_u_ne_none() {
        let v = NumUValidation::default().ne(22);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), None);
    }

    #[test]
    fn test_validate_num_u_gt_none() {
        let v = NumUValidation::default().gt(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(2)), None);
    }

    #[test]
    fn test_validate_num_u_lt_none() {
        let v = NumUValidation::default().lt(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(4)), None);
    }

    #[test]
    fn test_validate_num_u_ge_none() {
        let v = NumUValidation::default().ge(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), None);
    }

    #[test]
    fn test_validate_num_u_le_none() {
        let v = NumUValidation::default().le(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(5)), None);
    }

    #[test]
    fn test_validate_num_u_eq_required_none() {
        let v = NumUValidation::default().required().eq(42);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), None);
    }

    #[test]
    fn test_validate_num_u_ne_required_none() {
        let v = NumUValidation::default().required().ne(22);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), None);
    }

    #[test]
    fn test_validate_num_u_gt_required_none() {
        let v = NumUValidation::default().required().gt(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(2)), None);
    }

    #[test]
    fn test_validate_num_u_lt_required_none() {
        let v = NumUValidation::default().required().lt(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(4)), None);
    }

    #[test]
    fn test_validate_num_u_ge_required_none() {
        let v = NumUValidation::default().required().ge(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), None);
    }

    #[test]
    fn test_validate_num_u_le_required_none() {
        let v = NumUValidation::default().required().le(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(5)), None);
    }

    #[test]
    fn test_validate_num_u_default_some() {
        let v = NumUValidation::default();
        assert_eq!(validate_num_u(&v, &bool_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &num_i_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &num_f_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &str_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &arr_bool_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &obj_stub()), ErrWrap::arr([Err::NumU]));
        assert_eq!(validate_num_u(&v, &Value::None), ErrWrap::arr([Err::NumU]));
    }

    #[test]
    fn test_validate_num_u_required_some() {
        let v = NumUValidation::default().required();
        assert_eq!(validate_num_u(&v, &Value::None), ErrWrap::arr([Err::NumU, Err::Required]));
        assert_eq!(validate_num_u(&v, &bool_stub()), ErrWrap::arr([Err::NumU]));
    }

    #[test]
    fn test_validate_num_u_eq_some() {
        let v = NumUValidation::default().eq(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(0)), ErrWrap::arr([Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &Value::None), ErrWrap::arr([Err::NumU,  Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &bool_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
    }

    #[test]
    fn test_validate_num_u_ne_some() {
        let v = NumUValidation::default().ne(8);
        assert_eq!(validate_num_u(&v, &Value::NumU(8)), ErrWrap::arr([Err::Ne(Value::NumU(8))]));
        assert_eq!(validate_num_u(&v, &Value::None), ErrWrap::arr([Err::NumU, Err::Ne(Value::NumU(8))]));
        assert_eq!(validate_num_u(&v, &bool_stub()), ErrWrap::arr([Err::NumU, Err::Ne(Value::NumU(8))]));
    }

    #[test]
    fn test_validate_num_u_required_eq_some() {
        let v = NumUValidation::default().required().eq(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(5)), ErrWrap::arr([Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &Value::None), ErrWrap::arr([Err::NumU, Err::Required, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &bool_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &num_i_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &num_f_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &str_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &arr_bool_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
        assert_eq!(validate_num_u(&v, &obj_stub()), ErrWrap::arr([Err::NumU, Err::Eq(Value::NumU(1))]));
    }
}
