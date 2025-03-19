use crate::{
    error::{Err, ErrWrap},
    validation::{bool::BoolValidation, num_f::NumFValidation},
    value::Value,
};

pub fn validate_num_f(validation: &NumFValidation, value: &Value) -> Option<ErrWrap> {
    None
}
