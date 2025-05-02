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
    Validation(Vec<ValidationErr>),
    Arr(Vec<SchemaErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl SchemaErr {
    pub fn validation<const N: usize>(value: [ValidationErr; N]) -> SchemaErr {
        SchemaErr::Validation(value.to_vec())
    }

    pub fn arr<const N: usize>(value: [SchemaErr; N]) -> SchemaErr {
        SchemaErr::Arr(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr); N]) -> SchemaErr {
        SchemaErr::Obj(BTreeMap::from(value))
    }
}

fn schema_err_has_required(err: SchemaErr) -> bool {
    match err {
        SchemaErr::Validation(v) => v.contains(&ValidationErr::Required),
        SchemaErr::Arr(obj) => obj.into_iter().any(|schema_err| schema_err_has_required(schema_err)),
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
    fn validation_err() {
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

        assert_eq!(SchemaErr::validation([REQUIRED]), SchemaErr::Validation(vec![REQUIRED]));
        assert_eq!(SchemaErr::validation([U64]), SchemaErr::Validation(vec![U64]));
        assert_eq!(SchemaErr::validation([I64]), SchemaErr::Validation(vec![I64]));
        assert_eq!(SchemaErr::validation([F64]), SchemaErr::Validation(vec![F64]));
        assert_eq!(SchemaErr::validation([USIZE]), SchemaErr::Validation(vec![USIZE]));
        assert_eq!(SchemaErr::validation([ISIZE]), SchemaErr::Validation(vec![ISIZE]));
        assert_eq!(SchemaErr::validation([BOOL]), SchemaErr::Validation(vec![BOOL]));
        assert_eq!(SchemaErr::validation([STR]), SchemaErr::Validation(vec![STR]));
        assert_eq!(SchemaErr::validation([EMAIL]), SchemaErr::Validation(vec![EMAIL]));
        assert_eq!(SchemaErr::validation([DATE]), SchemaErr::Validation(vec![DATE]));
        assert_eq!(SchemaErr::validation([TIME]), SchemaErr::Validation(vec![TIME]));
        assert_eq!(SchemaErr::validation([DATE_TIME]), SchemaErr::Validation(vec![DATE_TIME]));
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

    #[test]
    fn schema_err_has_required_validation_false() {
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

        assert!(!schema_err_has_required(SchemaErr::validation([U64, operation_u64])));
        assert!(!schema_err_has_required(SchemaErr::validation([I64, operation_i64])));
        assert!(!schema_err_has_required(SchemaErr::validation([F64, operation_f64])));
        assert!(!schema_err_has_required(SchemaErr::validation([USIZE, operation_usize])));
        assert!(!schema_err_has_required(SchemaErr::validation([ISIZE, operation_isize])));
        assert!(!schema_err_has_required(SchemaErr::validation([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])));
    }

    #[test]
    fn schema_err_has_required_validation_true() {
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

        assert!(schema_err_has_required(SchemaErr::validation([U64, REQUIRED, operation_u64])));
        assert!(schema_err_has_required(SchemaErr::validation([I64, REQUIRED, operation_i64])));
        assert!(schema_err_has_required(SchemaErr::validation([F64, REQUIRED, operation_f64])));
        assert!(schema_err_has_required(SchemaErr::validation([USIZE, REQUIRED, operation_usize])));
        assert!(schema_err_has_required(SchemaErr::validation([ISIZE, REQUIRED, operation_isize])));
        assert!(schema_err_has_required(SchemaErr::validation([STR, REQUIRED, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])));
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

        let obj = SchemaErr::Arr(vec![
            SchemaErr::validation([U64, operation_u64]),
            SchemaErr::validation([I64, operation_i64]),
            SchemaErr::validation([F64, operation_f64]),
            SchemaErr::validation([USIZE, operation_usize]),
            SchemaErr::validation([ISIZE, operation_isize]),
            SchemaErr::validation([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols]),
        ]);
        assert!(!schema_err_has_required(obj));
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

        let obj = SchemaErr::Arr(vec![
            SchemaErr::validation([U64, operation_u64]),
            SchemaErr::validation([I64, operation_i64]),
            SchemaErr::validation([F64, operation_f64]),
            SchemaErr::validation([USIZE, operation_usize]),
            SchemaErr::validation([ISIZE, REQUIRED, operation_isize]),
            SchemaErr::validation([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols]),
        ]);
        assert!(schema_err_has_required(obj));
    }

    #[test]
    fn schema_err_has_required_arr_nested_false() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let obj = SchemaErr::Arr(vec![
            SchemaErr::validation([U64, operation_u64]),
            SchemaErr::Arr(vec![
                SchemaErr::validation([I64, operation_i64]),
                SchemaErr::Arr(vec![
                    SchemaErr::validation([F64, operation_f64]),
                    SchemaErr::Arr(vec![
                        SchemaErr::validation([USIZE, operation_usize]),
                        SchemaErr::Arr(vec![SchemaErr::validation([ISIZE, operation_isize])]),
                    ]),
                ]),
            ]),
        ]);
        assert!(!schema_err_has_required(obj));
    }

    #[test]
    fn schema_err_has_required_arr_nested_true() {
        let operation_u64 = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(93))));
        let operation_i64 = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::I64(-27))));
        let operation_f64 = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::F64(-5.75))));
        let operation_usize = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
        let operation_isize = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));

        let obj = SchemaErr::Arr(vec![
            SchemaErr::validation([U64, operation_u64]),
            SchemaErr::Arr(vec![
                SchemaErr::validation([I64, operation_i64]),
                SchemaErr::Arr(vec![
                    SchemaErr::validation([F64, operation_f64]),
                    SchemaErr::Arr(vec![
                        SchemaErr::validation([USIZE, REQUIRED, operation_usize]),
                        SchemaErr::Arr(vec![SchemaErr::validation([ISIZE, operation_isize])]),
                    ]),
                ]),
            ]),
        ]);
        assert!(schema_err_has_required(obj));
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
            ("u64".into(), SchemaErr::validation([U64, operation_u64])),
            ("i64".into(), SchemaErr::validation([I64, operation_i64])),
            ("f64".into(), SchemaErr::validation([F64, operation_f64])),
            ("usize".into(), SchemaErr::validation([USIZE, operation_usize])),
            ("isize".into(), SchemaErr::validation([ISIZE, operation_isize])),
            ("str".into(), SchemaErr::validation([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])),
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
            ("u64".into(), SchemaErr::validation([U64, operation_u64])),
            ("i64".into(), SchemaErr::validation([I64, operation_i64])),
            ("f64".into(), SchemaErr::validation([F64, operation_f64])),
            ("usize".into(), SchemaErr::validation([USIZE, operation_usize])),
            ("isize".into(), SchemaErr::validation([ISIZE, REQUIRED, operation_isize])),
            ("str".into(), SchemaErr::validation([STR, bytes, chars, graphemes, lowercase, uppercase, numbers, symbols])),
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

        let err_u64 = SchemaErr::validation([U64, operation_u64]);
        let err_i64 = SchemaErr::validation([I64, operation_i64]);
        let err_f64 = SchemaErr::validation([F64, operation_f64]);
        let err_usize = SchemaErr::validation([USIZE, operation_usize]);
        let err_isize = SchemaErr::validation([ISIZE, operation_isize]);

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

        let err_u64 = SchemaErr::validation([U64, operation_u64]);
        let err_i64 = SchemaErr::validation([I64, operation_i64]);
        let err_f64 = SchemaErr::validation([F64, operation_f64]);
        let err_usize = SchemaErr::validation([USIZE, REQUIRED, operation_usize]);
        let err_isize = SchemaErr::validation([ISIZE, operation_isize]);

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
