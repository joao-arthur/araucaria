use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    Bool,
    Obj,
    Eq(bool),
    Ne(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErrWrapper {
    Arr(Vec<ValidationErr>),
    Obj(HashMap<String, ValidationErrWrapper>),
}
