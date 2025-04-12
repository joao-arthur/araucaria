use std::collections::HashMap;

use crate::operation::{Operation, OperationEq};

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
    U64Operation(Operation<u64>),
    I64Operation(Operation<i64>),
    F64Operation(Operation<f64>),
    StringOperation(Operation<String>),
    BoolOperation(OperationEq<bool>),
    BytesLen(Operation<usize>),
    CharsLen(Operation<usize>),
    GraphemesLen(Operation<usize>),
    LowercaseLen(Operation<usize>),
    UppercaseLen(Operation<usize>),
    NumbersLen(Operation<usize>),
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
        assert_eq!(SchemaErr::validation([ValidationErr::NumU]), SchemaErr::Validation(vec![ValidationErr::NumU]));
        assert_eq!(SchemaErr::validation([ValidationErr::NumI]), SchemaErr::Validation(vec![ValidationErr::NumI]));
        assert_eq!(SchemaErr::validation([ValidationErr::NumF]), SchemaErr::Validation(vec![ValidationErr::NumF]));
        assert_eq!(SchemaErr::validation([ValidationErr::Bool]), SchemaErr::Validation(vec![ValidationErr::Bool]));
        assert_eq!(SchemaErr::validation([ValidationErr::Str]), SchemaErr::Validation(vec![ValidationErr::Str]));
        assert_eq!(SchemaErr::validation([ValidationErr::Email]), SchemaErr::Validation(vec![ValidationErr::Email]));
        assert_eq!(SchemaErr::validation([ValidationErr::Date]), SchemaErr::Validation(vec![ValidationErr::Date]));
        assert_eq!(SchemaErr::validation([ValidationErr::Time]), SchemaErr::Validation(vec![ValidationErr::Time]));
        assert_eq!(SchemaErr::validation([ValidationErr::DateTime]), SchemaErr::Validation(vec![ValidationErr::DateTime]));
        assert_eq!(
            SchemaErr::validation([ValidationErr::U64Operation(Operation::Gt(10))]),
            SchemaErr::Validation(vec![ValidationErr::U64Operation(Operation::Gt(10))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::I64Operation(Operation::Ge(11))]),
            SchemaErr::Validation(vec![ValidationErr::I64Operation(Operation::Ge(11))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::F64Operation(Operation::Lt(12.5))]),
            SchemaErr::Validation(vec![ValidationErr::F64Operation(Operation::Lt(12.5))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::StringOperation(Operation::Le(String::from("Swords")))]),
            SchemaErr::Validation(vec![ValidationErr::StringOperation(Operation::Le(String::from("Swords")))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::BoolOperation(OperationEq::Eq(false))]),
            SchemaErr::Validation(vec![ValidationErr::BoolOperation(OperationEq::Eq(false))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(1))]),
            SchemaErr::Validation(vec![ValidationErr::BytesLen(Operation::Eq(1))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::CharsLen(Operation::Ne(2))]),
            SchemaErr::Validation(vec![ValidationErr::CharsLen(Operation::Ne(2))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::GraphemesLen(Operation::Gt(3))]),
            SchemaErr::Validation(vec![ValidationErr::GraphemesLen(Operation::Gt(3))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::LowercaseLen(Operation::Ge(4))]),
            SchemaErr::Validation(vec![ValidationErr::LowercaseLen(Operation::Ge(4))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::UppercaseLen(Operation::Lt(5))]),
            SchemaErr::Validation(vec![ValidationErr::UppercaseLen(Operation::Lt(5))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::NumbersLen(Operation::Le(6))]),
            SchemaErr::Validation(vec![ValidationErr::NumbersLen(Operation::Le(6))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::SymbolsLen(Operation::Btwn(7, 8))]),
            SchemaErr::Validation(vec![ValidationErr::SymbolsLen(Operation::Btwn(7, 8))])
        );
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
        assert_eq!(
            SchemaErr::obj([(String::from("is"), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(HashMap::from([(String::from("is"), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
    }
}
