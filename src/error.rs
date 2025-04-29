use std::collections::BTreeMap;

use crate::{operation::Operation, validation::EnumValues};

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
    Enumerated(EnumValues),
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

    use crate::{
        operation::{Operand, OperandValue, Operation},
        validation::EnumValues,
    };

    use super::{SchemaErr, ValidationErr};

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;
    const USIZE: ValidationErr = ValidationErr::USize;
    const ISIZE: ValidationErr = ValidationErr::ISize;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const STR: ValidationErr = ValidationErr::Str;
    const EMAIL: ValidationErr = ValidationErr::Email;
    const DATE: ValidationErr = ValidationErr::Date;
    const TIME: ValidationErr = ValidationErr::Time;
    const DATE_TIME: ValidationErr = ValidationErr::DateTime;

    #[test]
    fn schema_err_validation() {
        let vec_usize: Vec<usize> = vec![10, 20, 30, 40, 50];
        let vec_isize: Vec<isize> = vec![0, -1, -2, -3, -4, -5];
        let vec_string: Vec<String> = vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()];

        let operation = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Swords"))));
        let bytes_len = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars_len = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes_len = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase_len = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase_len = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers_len = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols_len = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));
        let usize_enum = ValidationErr::Enumerated(EnumValues::USize(vec_usize));
        let isize_enum = ValidationErr::Enumerated(EnumValues::ISize(vec_isize));
        let str_enum = ValidationErr::Enumerated(EnumValues::Str(vec_string));

        assert_eq!(SchemaErr::arr([REQUIRED]), SchemaErr::Arr(vec![REQUIRED]));
        assert_eq!(SchemaErr::arr([U64]), SchemaErr::Arr(vec![U64]));
        assert_eq!(SchemaErr::arr([I64]), SchemaErr::Arr(vec![I64]));
        assert_eq!(SchemaErr::arr([F64]), SchemaErr::Arr(vec![F64]));
        assert_eq!(SchemaErr::arr([USIZE]), SchemaErr::Arr(vec![USIZE]));
        assert_eq!(SchemaErr::arr([ISIZE]), SchemaErr::Arr(vec![ISIZE]));
        assert_eq!(SchemaErr::arr([BOOL]), SchemaErr::Arr(vec![BOOL]));
        assert_eq!(SchemaErr::arr([STR]), SchemaErr::Arr(vec![STR]));
        assert_eq!(SchemaErr::arr([EMAIL]), SchemaErr::Arr(vec![EMAIL]));
        assert_eq!(SchemaErr::arr([DATE]), SchemaErr::Arr(vec![DATE]));
        assert_eq!(SchemaErr::arr([TIME]), SchemaErr::Arr(vec![TIME]));
        assert_eq!(SchemaErr::arr([DATE_TIME]), SchemaErr::Arr(vec![DATE_TIME]));
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
