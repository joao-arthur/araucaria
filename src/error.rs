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

fn schema_err_has_required(err: SchemaErr) -> bool {
    match err {
        SchemaErr::Arr(arr) => arr.contains(&ValidationErr::Required),
        SchemaErr::Obj(obj) => obj.into_iter().any(|(_, schema_err)| schema_err_has_required(schema_err)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        operation::{Operand, OperandValue, Operation},
        validation::EnumValues,
    };

    use super::{SchemaErr, ValidationErr, schema_err_has_required};

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

    #[test]
    fn schema_err_has_required_arr_false() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let bytes = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));

        assert!(!schema_err_has_required(SchemaErr::arr([U64, operation_u64])));
        assert!(!schema_err_has_required(SchemaErr::arr([I64, operation_i64])));
        assert!(!schema_err_has_required(SchemaErr::arr([F64, operation_f64])));
        assert!(!schema_err_has_required(SchemaErr::arr([USIZE, operation_usize])));
        assert!(!schema_err_has_required(SchemaErr::arr([ISIZE, operation_isize])));
        assert!(!schema_err_has_required(SchemaErr::arr([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])));
    }

    #[test]
    fn schema_err_has_required_arr_true() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let bytes = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));

        assert!(schema_err_has_required(SchemaErr::arr([U64, REQUIRED, operation_u64])));
        assert!(schema_err_has_required(SchemaErr::arr([I64, REQUIRED, operation_i64])));
        assert!(schema_err_has_required(SchemaErr::arr([F64, REQUIRED, operation_f64])));
        assert!(schema_err_has_required(SchemaErr::arr([USIZE, REQUIRED, operation_usize])));
        assert!(schema_err_has_required(SchemaErr::arr([ISIZE, REQUIRED, operation_isize])));
        assert!(schema_err_has_required(SchemaErr::arr([STR, REQUIRED, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])));
    }

    #[test]
    fn schema_err_has_required_obj_false() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let bytes = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));

        let obj = SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), SchemaErr::arr([U64, operation_u64])),
            ("i64".into(), SchemaErr::arr([I64, operation_i64])),
            ("f64".into(), SchemaErr::arr([F64, operation_f64])),
            ("usize".into(), SchemaErr::arr([USIZE, operation_usize])),
            ("isize".into(), SchemaErr::arr([ISIZE, operation_isize])),
            ("str".into(), SchemaErr::arr([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])),
        ]));
        assert!(!schema_err_has_required(obj));
    }

    #[test]
    fn schema_err_has_required_obj_true() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let bytes = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
        let chars = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        let graphemes = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
        let lowercase = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
        let uppercase = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        let numbers = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
        let symbols = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));

        let obj = SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), SchemaErr::arr([U64, operation_u64])),
            ("i64".into(), SchemaErr::arr([I64, operation_i64])),
            ("f64".into(), SchemaErr::arr([F64, operation_f64])),
            ("usize".into(), SchemaErr::arr([USIZE, operation_usize])),
            ("isize".into(), SchemaErr::arr([ISIZE, REQUIRED, operation_isize])),
            ("str".into(), SchemaErr::arr([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])),
        ]));
        assert!(schema_err_has_required(obj));
    }

    #[test]
    fn schema_err_has_required_obj_nested_false() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let err_u64 = SchemaErr::arr([U64, operation_u64]);
        let err_i64 = SchemaErr::arr([I64, operation_i64]);
        let err_f64 = SchemaErr::arr([F64, operation_f64]);
        let err_usize = SchemaErr::arr([USIZE, operation_usize]);
        let err_isize = SchemaErr::arr([ISIZE, operation_isize]);

        let obj = SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), err_u64),
            (
                "nested".into(),
                SchemaErr::Obj(BTreeMap::from([
                    ("i64".into(), err_i64),
                    (
                        "nested".into(),
                        SchemaErr::Obj(BTreeMap::from([
                            ("f64".into(), err_f64),
                            (
                                "nested".into(),
                                SchemaErr::Obj(BTreeMap::from([
                                    ("usize".into(), err_usize),
                                    ("nested".into(), SchemaErr::Obj(BTreeMap::from([("isize".into(), err_isize)]))),
                                ])),
                            ),
                        ])),
                    ),
                ])),
            ),
        ]));
        assert!(!schema_err_has_required(obj));
    }

    #[test]
    fn schema_err_has_required_obj_nested_true() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let err_u64 = SchemaErr::arr([U64, operation_u64]);
        let err_i64 = SchemaErr::arr([I64, operation_i64]);
        let err_f64 = SchemaErr::arr([F64, operation_f64]);
        let err_usize = SchemaErr::arr([USIZE, operation_usize]);
        let err_isize = SchemaErr::arr([ISIZE, operation_isize]);

        let obj = SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), err_u64),
            (
                "nested".into(),
                SchemaErr::Obj(BTreeMap::from([
                    ("i64".into(), err_i64),
                    (
                        "nested".into(),
                        SchemaErr::Obj(BTreeMap::from([
                            ("f64".into(), err_f64),
                            (
                                "nested".into(),
                                SchemaErr::Obj(BTreeMap::from([
                                    ("usize".into(), err_usize),
                                    ("nested".into(), SchemaErr::Obj(BTreeMap::from([("isize".into(), err_isize)]))),
                                ])),
                            ),
                        ])),
                    ),
                ])),
            ),
        ]));
        assert!(schema_err_has_required(obj));
    }
}
