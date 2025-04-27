use std::collections::BTreeMap;

use crate::operation::Operation;

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    U64,
    I64,
    F64,
    USize,
    ISize,
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
    USizeEnum(Vec<usize>),
    ISizeEnum(Vec<isize>),
    StrEnum(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Validation(Vec<ValidationErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl SchemaErr {
    pub fn validation<const N: usize>(value: [ValidationErr; N]) -> SchemaErr {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr); N]) -> SchemaErr {
        SchemaErr::Obj(BTreeMap::from(value))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr};

    #[test]
    fn schema_err_validation() {
        let required = ValidationErr::Required;
        let u64 = ValidationErr::U64;
        let i64 = ValidationErr::I64;
        let f64 = ValidationErr::F64;
        let usize = ValidationErr::USize;
        let isize = ValidationErr::ISize;
        let bool = ValidationErr::Bool;
        let str = ValidationErr::Str;
        let email = ValidationErr::Email;
        let date = ValidationErr::Date;
        let time = ValidationErr::Time;
        let date_time = ValidationErr::DateTime;
        let operation = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Swords"))));
        let bytes_len = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars_len = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes_len = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase_len = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase_len = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers_len = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols_len = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));
        let usize_enum = ValidationErr::USizeEnum(vec![10, 20, 30, 40, 50]);
        let isize_enum = ValidationErr::ISizeEnum(vec![0, -1, -2, -3, -4, -5]);
        let str_enum = ValidationErr::StrEnum(vec!["APPLE".into(), "BANANA".into(), "GRAPE".into(), "ORANGE".into(), "PEACH".into()]);

        assert_eq!(SchemaErr::validation([required.clone()]), SchemaErr::Validation(vec![required.clone()]));
        assert_eq!(SchemaErr::validation([u64.clone()]), SchemaErr::Validation(vec![u64.clone()]));
        assert_eq!(SchemaErr::validation([i64.clone()]), SchemaErr::Validation(vec![i64.clone()]));
        assert_eq!(SchemaErr::validation([f64.clone()]), SchemaErr::Validation(vec![f64.clone()]));
        assert_eq!(SchemaErr::validation([usize.clone()]), SchemaErr::Validation(vec![usize.clone()]));
        assert_eq!(SchemaErr::validation([isize.clone()]), SchemaErr::Validation(vec![isize.clone()]));
        assert_eq!(SchemaErr::validation([bool.clone()]), SchemaErr::Validation(vec![bool.clone()]));
        assert_eq!(SchemaErr::validation([str.clone()]), SchemaErr::Validation(vec![str.clone()]));
        assert_eq!(SchemaErr::validation([email.clone()]), SchemaErr::Validation(vec![email.clone()]));
        assert_eq!(SchemaErr::validation([date.clone()]), SchemaErr::Validation(vec![date.clone()]));
        assert_eq!(SchemaErr::validation([time.clone()]), SchemaErr::Validation(vec![time.clone()]));
        assert_eq!(SchemaErr::validation([date_time.clone()]), SchemaErr::Validation(vec![date_time.clone()]));
        assert_eq!(SchemaErr::validation([operation.clone()]), SchemaErr::Validation(vec![operation.clone()]));
        assert_eq!(SchemaErr::validation([bytes_len.clone()]), SchemaErr::Validation(vec![bytes_len.clone()]));
        assert_eq!(SchemaErr::validation([chars_len.clone()]), SchemaErr::Validation(vec![chars_len.clone()]));
        assert_eq!(SchemaErr::validation([graphemes_len.clone()]), SchemaErr::Validation(vec![graphemes_len.clone()]));
        assert_eq!(SchemaErr::validation([lowercase_len.clone()]), SchemaErr::Validation(vec![lowercase_len.clone()]));
        assert_eq!(SchemaErr::validation([uppercase_len.clone()]), SchemaErr::Validation(vec![uppercase_len.clone()]));
        assert_eq!(SchemaErr::validation([numbers_len.clone()]), SchemaErr::Validation(vec![numbers_len.clone()]));
        assert_eq!(SchemaErr::validation([symbols_len.clone()]), SchemaErr::Validation(vec![symbols_len.clone()]));
        assert_eq!(SchemaErr::validation([usize_enum.clone()]), SchemaErr::Validation(vec![usize_enum.clone()]));
        assert_eq!(SchemaErr::validation([isize_enum.clone()]), SchemaErr::Validation(vec![isize_enum.clone()]));
        assert_eq!(SchemaErr::validation([str_enum.clone()]), SchemaErr::Validation(vec![str_enum.clone()]));
    }

    #[test]
    fn schema_err_obj() {
        assert_eq!(
            SchemaErr::obj([("is".into(), SchemaErr::validation([ValidationErr::Required]))]),
            SchemaErr::Obj(BTreeMap::from([("is".into(), SchemaErr::Validation(vec![ValidationErr::Required]))]))
        );
    }
}
