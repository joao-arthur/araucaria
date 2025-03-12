use crate::{
    error::{ValidationErr, ValidationErrWrapper},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> ValidationErrWrapper {
    match value {
        Value::Bool(value) => {
            let mut base = vec![];
            if let Some(eq_v) = validation.eq {
                if value != &eq_v {
                    base.push(ValidationErr::Eq(eq_v));
                }
            }
            ValidationErrWrapper::Arr(base)
        }
        Value::None => {
            let mut base = vec![];
            if validation.required {
                base.push(ValidationErr::Bool);
                base.push(ValidationErr::Required);
                if let Some(eq_v) = validation.eq {
                    base.push(ValidationErr::Eq(eq_v));
                }
            }
            ValidationErrWrapper::Arr(base)
        }
        _ => {
            let mut base = vec![ValidationErr::Bool];
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(eq_v));
            }
            ValidationErrWrapper::Arr(base)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_bool_default() {
        assert_eq!(
            validate_bool(&BoolValidation::default(), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default(), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default(), &Value::None),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default(), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_validate_bool_required() {
        assert_eq!(
            validate_bool(&BoolValidation::default().required(), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required(), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required(), &Value::None),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Required])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required(), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool])
        );
    }

    #[test]
    fn test_validate_bool_eq() {
        assert_eq!(
            validate_bool(&BoolValidation::default().eq(false), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().eq(false), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().eq(false), &Value::None),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().eq(false), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }

    #[test]
    fn test_validate_bool_required_eq() {
        assert_eq!(
            validate_bool(&BoolValidation::default().required().eq(false), &Value::Bool(false)),
            ValidationErrWrapper::Arr(vec![])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required().eq(false), &Value::Bool(true)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Eq(false)])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required().eq(false), &Value::None),
            ValidationErrWrapper::Arr(vec![
                ValidationErr::Bool,
                ValidationErr::Required,
                ValidationErr::Eq(false)
            ])
        );
        assert_eq!(
            validate_bool(&BoolValidation::default().required().eq(false), &Value::NumU(1)),
            ValidationErrWrapper::Arr(vec![ValidationErr::Bool, ValidationErr::Eq(false)])
        );
    }
}
