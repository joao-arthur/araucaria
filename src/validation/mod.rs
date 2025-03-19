use std::collections::HashMap;

use bool::BoolValidation;
use num_f::NumFValidation;
use num_i::NumIValidation;
use num_u::NumUValidation;
use str::StrValidation;

pub mod bool;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub validation: HashMap<&'static str, Validation>,
    pub required: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    Bool(BoolValidation),
    NumU(NumUValidation),
    NumI(NumIValidation),
    NumF(NumFValidation),
    Str(StrValidation),
    Obj(ObjValidation),
}
