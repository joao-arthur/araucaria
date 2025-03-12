use std::collections::HashMap;

use bool::BoolValidation;

pub mod bool;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub validation: HashMap<&'static str, Validation>,
    pub required: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    Bool(BoolValidation),
    Obj(ObjValidation),
}
