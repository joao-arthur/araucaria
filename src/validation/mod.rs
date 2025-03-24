use std::collections::HashMap;

use bool::BoolValidation;
use date::DateValidation;
use datetime::DateTimeValidation;
use email::EmailValidation;
use num_f::NumFValidation;
use num_i::NumIValidation;
use num_u::NumUValidation;
use str::StrValidation;
use time::TimeValidation;

pub mod bool;
pub mod date;
pub mod datetime;
pub mod email;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;
pub mod time;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub validation: HashMap<&'static str, Validation>,
    pub required: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    NumU(NumUValidation),
    NumI(NumIValidation),
    NumF(NumFValidation),
    Bool(BoolValidation),
    Str(StrValidation),
    Email(EmailValidation),
    Date(DateValidation),
    Time(TimeValidation),
    DateTime(DateTimeValidation),
    Obj(ObjValidation),
}
