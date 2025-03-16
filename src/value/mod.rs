use std::collections::HashMap;

#[cfg(test)]
pub mod stub;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    Bool(bool),
    NumU(u64),
    NumI(i64),
    NumF(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
}
