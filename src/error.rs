use std::collections::HashMap;

use crate::{operation::Operation, value::Value};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    NumU,
    NumI,
    NumF,
    Bool,
    Str,
    Email,
    Date,
    Time,
    DateTime,
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Ge(Value),
    Le(Value),
    BytesLen(Operation<usize>),
    CharsLen(Operation<usize>),
    GraphemesLen(Operation<usize>),
    LowercaseLen(Operation<usize>),
    UppercaseLen(Operation<usize>),
    NumberLen(Operation<usize>),
    SymbolsLen(Operation<usize>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Validation(Vec<ValidationErr>),
    Obj(HashMap<String, SchemaErr>),
}

impl SchemaErr {
    pub fn validation<const N: usize>(value: [ValidationErr; N]) -> SchemaErr {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr); N]) -> SchemaErr {
        SchemaErr::Obj(HashMap::from(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arr() {
        assert_eq!(SchemaErr::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
    }
}
