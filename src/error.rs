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
    Arr(Vec<ValidationErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl SchemaErr {
    pub fn arr<const N: usize>(value: [ValidationErr; N]) -> SchemaErr {
        SchemaErr::Arr(value.to_vec())
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

        assert_eq!(SchemaErr::arr([required.clone()]), SchemaErr::Arr(vec![required.clone()]));
        assert_eq!(SchemaErr::arr([u64.clone()]), SchemaErr::Arr(vec![u64.clone()]));
        assert_eq!(SchemaErr::arr([i64.clone()]), SchemaErr::Arr(vec![i64.clone()]));
        assert_eq!(SchemaErr::arr([f64.clone()]), SchemaErr::Arr(vec![f64.clone()]));
        assert_eq!(SchemaErr::arr([usize.clone()]), SchemaErr::Arr(vec![usize.clone()]));
        assert_eq!(SchemaErr::arr([isize.clone()]), SchemaErr::Arr(vec![isize.clone()]));
        assert_eq!(SchemaErr::arr([bool.clone()]), SchemaErr::Arr(vec![bool.clone()]));
        assert_eq!(SchemaErr::arr([str.clone()]), SchemaErr::Arr(vec![str.clone()]));
        assert_eq!(SchemaErr::arr([email.clone()]), SchemaErr::Arr(vec![email.clone()]));
        assert_eq!(SchemaErr::arr([date.clone()]), SchemaErr::Arr(vec![date.clone()]));
        assert_eq!(SchemaErr::arr([time.clone()]), SchemaErr::Arr(vec![time.clone()]));
        assert_eq!(SchemaErr::arr([date_time.clone()]), SchemaErr::Arr(vec![date_time.clone()]));
        assert_eq!(SchemaErr::arr([operation.clone()]), SchemaErr::Arr(vec![operation.clone()]));
        assert_eq!(SchemaErr::arr([bytes_len.clone()]), SchemaErr::Arr(vec![bytes_len.clone()]));
        assert_eq!(SchemaErr::arr([chars_len.clone()]), SchemaErr::Arr(vec![chars_len.clone()]));
        assert_eq!(SchemaErr::arr([graphemes_len.clone()]), SchemaErr::Arr(vec![graphemes_len.clone()]));
        assert_eq!(SchemaErr::arr([lowercase_len.clone()]), SchemaErr::Arr(vec![lowercase_len.clone()]));
        assert_eq!(SchemaErr::arr([uppercase_len.clone()]), SchemaErr::Arr(vec![uppercase_len.clone()]));
        assert_eq!(SchemaErr::arr([numbers_len.clone()]), SchemaErr::Arr(vec![numbers_len.clone()]));
        assert_eq!(SchemaErr::arr([symbols_len.clone()]), SchemaErr::Arr(vec![symbols_len.clone()]));
        assert_eq!(SchemaErr::arr([usize_enum.clone()]), SchemaErr::Arr(vec![usize_enum.clone()]));
        assert_eq!(SchemaErr::arr([isize_enum.clone()]), SchemaErr::Arr(vec![isize_enum.clone()]));
        assert_eq!(SchemaErr::arr([str_enum.clone()]), SchemaErr::Arr(vec![str_enum.clone()]));
    }

    #[test]
    fn schema_err_obj() {
        assert_eq!(
            SchemaErr::obj([("is".into(), SchemaErr::arr([ValidationErr::Required]))]),
            SchemaErr::Obj(BTreeMap::from([("is".into(), SchemaErr::Arr(vec![ValidationErr::Required]))]))
        );
    }
}
