use std::collections::HashMap;

use crate::operation::{Operation, OperationEq};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr<T> {
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
    Operation(Operation<T>),
    OperationEq(OperationEq<T>),
    BytesLen(Operation<usize>),
    CharsLen(Operation<usize>),
    GraphemesLen(Operation<usize>),
    LowercaseLen(Operation<usize>),
    UppercaseLen(Operation<usize>),
    NumbersLen(Operation<usize>),
    SymbolsLen(Operation<usize>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr<T> {
    Validation(Vec<ValidationErr<T>>),
    Obj(HashMap<String, SchemaErr<T>>),
}

impl SchemaErr<usize> {
    pub fn validation<const N: usize>(value: [ValidationErr<usize>; N]) -> SchemaErr<usize> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<usize>); N]) -> SchemaErr<usize> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

impl SchemaErr<bool> {
    pub fn validation<const N: usize>(value: [ValidationErr<bool>; N]) -> SchemaErr<bool> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<bool>); N]) -> SchemaErr<bool> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

impl SchemaErr<String> {
    pub fn validation<const N: usize>(value: [ValidationErr<String>; N]) -> SchemaErr<String> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<String>); N]) -> SchemaErr<String> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

impl SchemaErr<u64> {
    pub fn validation<const N: usize>(value: [ValidationErr<u64>; N]) -> SchemaErr<u64> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<u64>); N]) -> SchemaErr<u64> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

impl SchemaErr<i64> {
    pub fn validation<const N: usize>(value: [ValidationErr<i64>; N]) -> SchemaErr<i64> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<i64>); N]) -> SchemaErr<i64> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

impl SchemaErr<f64> {
    pub fn validation<const N: usize>(value: [ValidationErr<f64>; N]) -> SchemaErr<f64> {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr<f64>); N]) -> SchemaErr<f64> {
        SchemaErr::Obj(HashMap::from(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arr() {
        assert_eq!(SchemaErr::<u64>::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
        assert_eq!(SchemaErr::<i64>::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
        assert_eq!(SchemaErr::<f64>::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
        assert_eq!(SchemaErr::<bool>::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
        assert_eq!(SchemaErr::<String>::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            SchemaErr::<u64>::obj([(String::from("is"), SchemaErr::<u64>::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::<i64>::obj([(String::from("is"), SchemaErr::<i64>::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::<f64>::obj([(String::from("is"), SchemaErr::<f64>::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::<bool>::obj([(String::from("is"), SchemaErr::<bool>::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::<String>::obj([(String::from("is"), SchemaErr::<String>::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
    }
}
