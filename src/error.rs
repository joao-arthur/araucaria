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

pub fn schema_err_has_required(err: SchemaErr) -> bool {
    match err {
        SchemaErr::Validation(v) => v.contains(&ValidationErr::Required),
        SchemaErr::Arr(obj) => obj.into_iter().any(|schema_err| schema_err_has_required(schema_err)),
        SchemaErr::Obj(obj) => obj.into_iter().any(|(_, schema_err)| schema_err_has_required(schema_err)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr, schema_err_has_required};

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;
    const USIZE: ValidationErr = ValidationErr::USize;
    const ISIZE: ValidationErr = ValidationErr::ISize;
    const STR: ValidationErr = ValidationErr::Str;

    const OPERATION_U64: ValidationErr = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(37))));
    const OPERATION_I64: ValidationErr = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-17))));
    const OPERATION_F64: ValidationErr = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-38.5))));
    const OPERATION_USIZE: ValidationErr = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(89))));
    const OPERATION_ISIZE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::ISize(-79))));
    const BYTES: ValidationErr = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(1))));
    const CHARS: ValidationErr = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
    const GRAPHEMES: ValidationErr = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(3))));
    const LOWERCASE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(4))));
    const UPPERCASE: ValidationErr = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(5))));
    const NUMBERS: ValidationErr = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(6))));
    const SYMBOLS: ValidationErr =
        ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(7)), Operand::Value(OperandValue::USize(8))));

    #[test]
    fn schema_err_validation() {
        assert_eq!(SchemaErr::validation([REQUIRED, U64, OPERATION_U64]), SchemaErr::Validation(vec![REQUIRED, U64, OPERATION_U64]));
    }

    #[test]
    fn schema_err_arr() {
        assert_eq!(
            SchemaErr::arr([SchemaErr::validation([REQUIRED, I64, OPERATION_I64])]),
            SchemaErr::Arr(vec![SchemaErr::Validation(vec![REQUIRED, I64, OPERATION_I64])])
        );
    }

    #[test]
    fn schema_err_obj() {
        assert_eq!(
            SchemaErr::obj([("f64".into(), SchemaErr::validation([REQUIRED, F64, OPERATION_F64]))]),
            SchemaErr::Obj(BTreeMap::from([("f64".into(), SchemaErr::Validation(vec![REQUIRED, F64, OPERATION_F64]))]))
        );
    }

    #[test]
    fn schema_err_nested() {
        assert_eq!(
            SchemaErr::obj([(
                "user".into(),
                SchemaErr::arr([
                    SchemaErr::validation([REQUIRED]),
                    SchemaErr::obj([("i64".into(), SchemaErr::validation([REQUIRED, I64, OPERATION_I64]))]),
                ]),
            )]),
            SchemaErr::Obj(BTreeMap::from([(
                "user".into(),
                SchemaErr::Arr(vec![
                    SchemaErr::Validation(vec![REQUIRED]),
                    SchemaErr::Obj(BTreeMap::from([("i64".into(), SchemaErr::Validation(vec![REQUIRED, I64, OPERATION_I64]))])),
                ]),
            )]))
        );
    }

    #[test]
    fn schema_err_has_required_validation_false() {
        assert!(!schema_err_has_required(SchemaErr::validation([U64, OPERATION_U64])));
        assert!(!schema_err_has_required(SchemaErr::validation([I64, OPERATION_I64])));
        assert!(!schema_err_has_required(SchemaErr::validation([F64, OPERATION_F64])));
        assert!(!schema_err_has_required(SchemaErr::validation([USIZE, OPERATION_USIZE])));
        assert!(!schema_err_has_required(SchemaErr::validation([ISIZE, OPERATION_ISIZE])));
        assert!(!schema_err_has_required(SchemaErr::validation([STR, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS])));
    }

    #[test]
    fn schema_err_has_required_validation_true() {
        assert!(schema_err_has_required(SchemaErr::validation([U64, REQUIRED, OPERATION_U64])));
        assert!(schema_err_has_required(SchemaErr::validation([I64, REQUIRED, OPERATION_I64])));
        assert!(schema_err_has_required(SchemaErr::validation([F64, REQUIRED, OPERATION_F64])));
        assert!(schema_err_has_required(SchemaErr::validation([USIZE, REQUIRED, OPERATION_USIZE])));
        assert!(schema_err_has_required(SchemaErr::validation([ISIZE, REQUIRED, OPERATION_ISIZE])));
        assert!(schema_err_has_required(SchemaErr::validation([STR, REQUIRED, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS])));
    }

    #[test]
    fn schema_err_has_required_arr_false() {
        assert!(!schema_err_has_required(SchemaErr::arr([
            SchemaErr::validation([U64, OPERATION_U64]),
            SchemaErr::validation([I64, OPERATION_I64]),
            SchemaErr::validation([F64, OPERATION_F64]),
            SchemaErr::validation([USIZE, OPERATION_USIZE]),
            SchemaErr::validation([ISIZE, OPERATION_ISIZE]),
            SchemaErr::validation([STR, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS]),
        ])));
    }

    #[test]
    fn schema_err_has_required_arr_true() {
        assert!(schema_err_has_required(SchemaErr::arr([
            SchemaErr::validation([U64, OPERATION_U64]),
            SchemaErr::validation([I64, OPERATION_I64]),
            SchemaErr::validation([F64, OPERATION_F64]),
            SchemaErr::validation([USIZE, OPERATION_USIZE]),
            SchemaErr::validation([ISIZE, REQUIRED, OPERATION_ISIZE]),
            SchemaErr::validation([STR, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS]),
        ])));
    }

    #[test]
    fn schema_err_has_required_arr_nested_false() {
        assert!(!schema_err_has_required(SchemaErr::arr([
            SchemaErr::validation([U64, OPERATION_U64]),
            SchemaErr::arr([
                SchemaErr::validation([I64, OPERATION_I64]),
                SchemaErr::arr([
                    SchemaErr::validation([F64, OPERATION_F64]),
                    SchemaErr::arr([
                        SchemaErr::validation([USIZE, OPERATION_USIZE]),
                        SchemaErr::arr([SchemaErr::validation([ISIZE, OPERATION_ISIZE])]),
                    ]),
                ]),
            ]),
        ])));
    }

    #[test]
    fn schema_err_has_required_arr_nested_true() {
        assert!(schema_err_has_required(SchemaErr::arr([
            SchemaErr::validation([U64, OPERATION_U64]),
            SchemaErr::arr([
                SchemaErr::validation([I64, OPERATION_I64]),
                SchemaErr::arr([
                    SchemaErr::validation([F64, OPERATION_F64]),
                    SchemaErr::arr([
                        SchemaErr::validation([USIZE, REQUIRED, OPERATION_USIZE]),
                        SchemaErr::arr([SchemaErr::validation([ISIZE, OPERATION_ISIZE])]),
                    ]),
                ]),
            ]),
        ])));
    }

    #[test]
    fn schema_err_has_required_obj_false() {
        assert!(!schema_err_has_required(SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
            ("usize".into(), SchemaErr::validation([USIZE, OPERATION_USIZE])),
            ("isize".into(), SchemaErr::validation([ISIZE, OPERATION_ISIZE])),
            ("str".into(), SchemaErr::validation([STR, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS])),
        ]))));
    }

    #[test]
    fn schema_err_has_required_obj_true() {
        assert!(schema_err_has_required(SchemaErr::Obj(BTreeMap::from([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
            ("usize".into(), SchemaErr::validation([USIZE, OPERATION_USIZE])),
            ("isize".into(), SchemaErr::validation([ISIZE, REQUIRED, OPERATION_ISIZE])),
            ("str".into(), SchemaErr::validation([STR, BYTES, CHARS, GRAPHEMES, LOWERCASE, UPPERCASE, NUMBERS, SYMBOLS])),
        ]))));
    }

    #[test]
    fn schema_err_has_required_obj_nested_false() {
        assert!(!schema_err_has_required(SchemaErr::obj([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            (
                "nested".into(),
                SchemaErr::obj([
                    ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
                    (
                        "nested".into(),
                        SchemaErr::obj([
                            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
                            (
                                "nested".into(),
                                SchemaErr::obj([
                                    ("usize".into(), SchemaErr::validation([USIZE, OPERATION_USIZE])),
                                    ("nested".into(), SchemaErr::obj([("isize".into(), SchemaErr::validation([ISIZE, OPERATION_ISIZE]))])),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ])));
    }

    #[test]
    fn schema_err_has_required_obj_nested_true() {
        assert!(schema_err_has_required(SchemaErr::obj([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            (
                "nested".into(),
                SchemaErr::obj([
                    ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
                    (
                        "nested".into(),
                        SchemaErr::obj([
                            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
                            (
                                "nested".into(),
                                SchemaErr::obj([
                                    ("usize".into(), SchemaErr::validation([USIZE, REQUIRED, OPERATION_USIZE])),
                                    ("nested".into(), SchemaErr::obj([("isize".into(), SchemaErr::validation([ISIZE, OPERATION_ISIZE]))])),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ])));
    }

    #[test]
    fn schema_err_has_required_nested_false() {
        assert!(!schema_err_has_required(SchemaErr::obj([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            (
                "nested".into(),
                SchemaErr::arr([SchemaErr::obj([
                    ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
                    (
                        "nested".into(),
                        SchemaErr::arr([SchemaErr::obj([
                            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
                            (
                                "nested".into(),
                                SchemaErr::arr([SchemaErr::obj([
                                    ("usize".into(), SchemaErr::validation([USIZE, OPERATION_USIZE])),
                                    ("nested".into(), SchemaErr::obj([("isize".into(), SchemaErr::validation([ISIZE, OPERATION_ISIZE]))])),
                                ])]),
                            ),
                        ])]),
                    ),
                ])])
            ),
        ])));
    }

    #[test]
    fn schema_err_has_required_nested_true() {
        assert!(schema_err_has_required(SchemaErr::obj([
            ("u64".into(), SchemaErr::validation([U64, OPERATION_U64])),
            (
                "nested".into(),
                SchemaErr::arr([SchemaErr::obj([
                    ("i64".into(), SchemaErr::validation([I64, OPERATION_I64])),
                    (
                        "nested".into(),
                        SchemaErr::arr([SchemaErr::obj([
                            ("f64".into(), SchemaErr::validation([F64, OPERATION_F64])),
                            (
                                "nested".into(),
                                SchemaErr::arr([
                                    SchemaErr::validation([ValidationErr::Required]),
                                    SchemaErr::obj([
                                        ("usize".into(), SchemaErr::validation([USIZE, OPERATION_USIZE])),
                                        ("nested".into(), SchemaErr::obj([("isize".into(), SchemaErr::validation([ISIZE, OPERATION_ISIZE]))])),
                                    ])
                                ]),
                            ),
                        ])]),
                    ),
                ])])
            ),
        ])));
    }
}
