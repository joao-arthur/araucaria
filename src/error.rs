use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Err {
    Required,
    Bool,
    Obj,
    Eq(bool),
    Ne(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrWrapper {
    Arr(Vec<Err>),
    Obj(HashMap<String, ErrWrapper>),
}
