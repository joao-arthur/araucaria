use std::collections::HashMap;

use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    U64,
    I64,
    F64,
    Bool,
    Str,
    Email,
    Date,
    Time,
    DateTime,
    Operation(Operation),
    BytesLen(Operation),
    CharsLen(Operation),
    GraphemesLen(Operation),
    LowercaseLen(Operation),
    UppercaseLen(Operation),
    NumbersLen(Operation),
    SymbolsLen(Operation),
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
    use std::collections::HashMap;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr};

    #[test]
    fn test_arr() {
        assert_eq!(SchemaErr::validation([ValidationErr::Required]), SchemaErr::Validation(vec![ValidationErr::Required]));
        assert_eq!(SchemaErr::validation([ValidationErr::U64]), SchemaErr::Validation(vec![ValidationErr::U64]));
        assert_eq!(SchemaErr::validation([ValidationErr::I64]), SchemaErr::Validation(vec![ValidationErr::I64]));
        assert_eq!(SchemaErr::validation([ValidationErr::F64]), SchemaErr::Validation(vec![ValidationErr::F64]));
        assert_eq!(SchemaErr::validation([ValidationErr::Bool]), SchemaErr::Validation(vec![ValidationErr::Bool]));
        assert_eq!(SchemaErr::validation([ValidationErr::Str]), SchemaErr::Validation(vec![ValidationErr::Str]));
        assert_eq!(SchemaErr::validation([ValidationErr::Email]), SchemaErr::Validation(vec![ValidationErr::Email]));
        assert_eq!(SchemaErr::validation([ValidationErr::Date]), SchemaErr::Validation(vec![ValidationErr::Date]));
        assert_eq!(SchemaErr::validation([ValidationErr::Time]), SchemaErr::Validation(vec![ValidationErr::Time]));
        assert_eq!(SchemaErr::validation([ValidationErr::DateTime]), SchemaErr::Validation(vec![ValidationErr::DateTime]));
        assert_eq!(
            SchemaErr::validation([ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Swords")))))]),
            SchemaErr::Validation(vec![ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Swords")))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))))]),
            SchemaErr::Validation(vec![ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))))]),
            SchemaErr::Validation(vec![ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))))]),
            SchemaErr::Validation(vec![ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))))]),
            SchemaErr::Validation(vec![ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))))]),
            SchemaErr::Validation(vec![ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))))]),
            SchemaErr::Validation(vec![ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))))])
        );
        assert_eq!(
            SchemaErr::validation([ValidationErr::SymbolsLen(Operation::Btwn(
                Operand::Value(OperandValue::USize(7)),
                Operand::Value(OperandValue::USize(8))
            ))]),
            SchemaErr::Validation(vec![ValidationErr::SymbolsLen(Operation::Btwn(
                Operand::Value(OperandValue::USize(7)),
                Operand::Value(OperandValue::USize(8))
            ))])
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
