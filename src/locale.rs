use std::collections::BTreeMap;

use crate::{
    error::{SchemaErr, ValidationErr},
    operation::Operation,
};

pub struct Locale {
    required: String,
    u64: String,
    i64: String,
    f64: String,
    usize: String,
    isize: String,
    bool: String,
    str: String,
    email: String,
    date: String,
    time: String,
    date_time: String,
    eq: String,
    ne: String,
    gt: String,
    lt: String,
    ge: String,
    le: String,
    btwn: String,
    bytes_len_eq: String,
    bytes_len_ne: String,
    bytes_len_gt: String,
    bytes_len_ge: String,
    bytes_len_lt: String,
    bytes_len_le: String,
    bytes_len_btwn: String,
    chars_len_eq: String,
    chars_len_ne: String,
    chars_len_gt: String,
    chars_len_ge: String,
    chars_len_lt: String,
    chars_len_le: String,
    chars_len_btwn: String,
    graphemes_len_eq: String,
    graphemes_len_ne: String,
    graphemes_len_gt: String,
    graphemes_len_ge: String,
    graphemes_len_lt: String,
    graphemes_len_le: String,
    graphemes_len_btwn: String,
    lowercase_len_eq: String,
    lowercase_len_ne: String,
    lowercase_len_gt: String,
    lowercase_len_ge: String,
    lowercase_len_lt: String,
    lowercase_len_le: String,
    lowercase_len_btwn: String,
    uppercase_len_eq: String,
    uppercase_len_ne: String,
    uppercase_len_gt: String,
    uppercase_len_ge: String,
    uppercase_len_lt: String,
    uppercase_len_le: String,
    uppercase_len_btwn: String,
    number_len_eq: String,
    number_len_ne: String,
    number_len_gt: String,
    number_len_ge: String,
    number_len_lt: String,
    number_len_le: String,
    number_len_btwn: String,
    symbols_eq: String,
    symbols_ne: String,
    symbols_gt: String,
    symbols_ge: String,
    symbols_lt: String,
    symbols_le: String,
    symbols_btwn: String,
    enumerated: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaLocalizedErr {
    Arr(Vec<String>),
    Obj(BTreeMap<String, SchemaLocalizedErr>),
}

pub fn localize_validation_err(error: &ValidationErr, locale: &Locale) -> String {
    match error {
        ValidationErr::Required => locale.required.clone(),
        ValidationErr::U64 => locale.u64.clone(),
        ValidationErr::I64 => locale.i64.clone(),
        ValidationErr::F64 => locale.f64.clone(),
        ValidationErr::USize => locale.usize.clone(),
        ValidationErr::ISize => locale.isize.clone(),
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::Email => locale.email.clone(),
        ValidationErr::Date => locale.date.clone(),
        ValidationErr::Time => locale.time.clone(),
        ValidationErr::DateTime => locale.date_time.clone(),
        ValidationErr::Operation(operation) => match operation {
            Operation::Eq(v) => locale.eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::BytesLen(operation) => match operation {
            Operation::Eq(v) => locale.bytes_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.bytes_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.bytes_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.bytes_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.bytes_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.bytes_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.bytes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::CharsLen(operation) => match operation {
            Operation::Eq(v) => locale.chars_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.chars_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.chars_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.chars_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.chars_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.chars_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.chars_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::GraphemesLen(operation) => match operation {
            Operation::Eq(v) => locale.graphemes_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.graphemes_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.graphemes_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.graphemes_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.graphemes_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.graphemes_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.graphemes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::LowercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.lowercase_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.lowercase_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.lowercase_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.lowercase_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.lowercase_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.lowercase_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.lowercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::UppercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.uppercase_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.uppercase_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.uppercase_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.uppercase_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.uppercase_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.uppercase_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.uppercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::NumbersLen(operation) => match operation {
            Operation::Eq(v) => locale.number_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.number_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.number_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.number_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.number_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.number_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.number_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::SymbolsLen(operation) => match operation {
            Operation::Eq(v) => locale.symbols_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.symbols_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.symbols_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.symbols_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.symbols_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.symbols_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.symbols_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::Enumerated(values) => locale.enumerated.replace("%value%", &values.to_string()),
    }
}

pub fn localize_schema_err(err: &SchemaErr, locale: &Locale) -> SchemaLocalizedErr {
    match err {
        SchemaErr::Arr(arr) => SchemaLocalizedErr::Arr(arr.iter().map(|item| localize_validation_err(item, locale)).collect()),
        SchemaErr::Obj(obj) => {
            let mut result: BTreeMap<String, SchemaLocalizedErr> = BTreeMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), localize_schema_err(item, locale));
            }
            SchemaLocalizedErr::Obj(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::EnumValues,
    };

    use super::{Locale, SchemaLocalizedErr, localize_schema_err, localize_validation_err};

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

    const U64_VALUE_A: Operand = Operand::Value(OperandValue::U64(34));
    const U64_VALUE_B: Operand = Operand::Value(OperandValue::U64(43));
    const I64_VALUE_A: Operand = Operand::Value(OperandValue::I64(-4));
    const I64_VALUE_B: Operand = Operand::Value(OperandValue::I64(4));
    const F64_VALUE_A: Operand = Operand::Value(OperandValue::F64(-4.6));
    const F64_VALUE_B: Operand = Operand::Value(OperandValue::F64(-2.4));
    const USIZE_VALUE_A: Operand = Operand::Value(OperandValue::USize(27));
    const USIZE_VALUE_B: Operand = Operand::Value(OperandValue::USize(39));
    const ISIZE_VALUE_A: Operand = Operand::Value(OperandValue::ISize(-93));
    const ISIZE_VALUE_B: Operand = Operand::Value(OperandValue::ISize(-72));
    const BOOL_VALUE_A: Operand = Operand::Value(OperandValue::Bool(false));
    const BOOL_VALUE_B: Operand = Operand::Value(OperandValue::Bool(true));

    const OP_U64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(U64_VALUE_A));
    const OP_U64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(U64_VALUE_A));
    const OP_U64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(U64_VALUE_A));
    const OP_U64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(U64_VALUE_A));
    const OP_U64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(U64_VALUE_A));
    const OP_U64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(U64_VALUE_A));
    const OP_U64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(U64_VALUE_A, U64_VALUE_B));
    const OP_I64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(I64_VALUE_A));
    const OP_I64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(I64_VALUE_A));
    const OP_I64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(I64_VALUE_A));
    const OP_I64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(I64_VALUE_A));
    const OP_I64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(I64_VALUE_A));
    const OP_I64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(I64_VALUE_A));
    const OP_I64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(I64_VALUE_A, I64_VALUE_B));
    const OP_F64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(F64_VALUE_A));
    const OP_F64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(F64_VALUE_A));
    const OP_F64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(F64_VALUE_A));
    const OP_F64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(F64_VALUE_A));
    const OP_F64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(F64_VALUE_A));
    const OP_F64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(F64_VALUE_A));
    const OP_F64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(F64_VALUE_A, F64_VALUE_B));
    const OP_USIZE_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(USIZE_VALUE_A));
    const OP_USIZE_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(USIZE_VALUE_A));
    const OP_USIZE_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(USIZE_VALUE_A));
    const OP_USIZE_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(USIZE_VALUE_A));
    const OP_USIZE_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(USIZE_VALUE_A));
    const OP_USIZE_LE: ValidationErr = ValidationErr::Operation(Operation::Le(USIZE_VALUE_A));
    const OP_USIZE_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const OP_ISIZE_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(ISIZE_VALUE_A));
    const OP_ISIZE_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(ISIZE_VALUE_A));
    const OP_ISIZE_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(ISIZE_VALUE_A));
    const OP_ISIZE_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(ISIZE_VALUE_A));
    const OP_ISIZE_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(ISIZE_VALUE_A));
    const OP_ISIZE_LE: ValidationErr = ValidationErr::Operation(Operation::Le(ISIZE_VALUE_A));
    const OP_ISIZE_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(ISIZE_VALUE_A, ISIZE_VALUE_B));
    const OP_BOOL_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(BOOL_VALUE_A));
    const OP_BOOL_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(BOOL_VALUE_A));
    const OP_BOOL_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(BOOL_VALUE_A));
    const OP_BOOL_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(BOOL_VALUE_A));
    const OP_BOOL_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(BOOL_VALUE_A));
    const OP_BOOL_LE: ValidationErr = ValidationErr::Operation(Operation::Le(BOOL_VALUE_A));
    const OP_BOOL_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(BOOL_VALUE_A, BOOL_VALUE_B));
    const BYTES_LEN_EQ: ValidationErr = ValidationErr::BytesLen(Operation::Eq(USIZE_VALUE_A));
    const BYTES_LEN_NE: ValidationErr = ValidationErr::BytesLen(Operation::Ne(USIZE_VALUE_A));
    const BYTES_LEN_GT: ValidationErr = ValidationErr::BytesLen(Operation::Gt(USIZE_VALUE_A));
    const BYTES_LEN_GE: ValidationErr = ValidationErr::BytesLen(Operation::Ge(USIZE_VALUE_A));
    const BYTES_LEN_LT: ValidationErr = ValidationErr::BytesLen(Operation::Lt(USIZE_VALUE_A));
    const BYTES_LEN_LE: ValidationErr = ValidationErr::BytesLen(Operation::Le(USIZE_VALUE_A));
    const BYTES_LEN_BTWN: ValidationErr = ValidationErr::BytesLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const CHARS_LEN_EQ: ValidationErr = ValidationErr::CharsLen(Operation::Eq(USIZE_VALUE_A));
    const CHARS_LEN_NE: ValidationErr = ValidationErr::CharsLen(Operation::Ne(USIZE_VALUE_A));
    const CHARS_LEN_GT: ValidationErr = ValidationErr::CharsLen(Operation::Gt(USIZE_VALUE_A));
    const CHARS_LEN_GE: ValidationErr = ValidationErr::CharsLen(Operation::Ge(USIZE_VALUE_A));
    const CHARS_LEN_LT: ValidationErr = ValidationErr::CharsLen(Operation::Lt(USIZE_VALUE_A));
    const CHARS_LEN_LE: ValidationErr = ValidationErr::CharsLen(Operation::Le(USIZE_VALUE_A));
    const CHARS_LEN_BTWN: ValidationErr = ValidationErr::CharsLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const GRAPHEMES_LEN_EQ: ValidationErr = ValidationErr::GraphemesLen(Operation::Eq(USIZE_VALUE_A));
    const GRAPHEMES_LEN_NE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ne(USIZE_VALUE_A));
    const GRAPHEMES_LEN_GT: ValidationErr = ValidationErr::GraphemesLen(Operation::Gt(USIZE_VALUE_A));
    const GRAPHEMES_LEN_GE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ge(USIZE_VALUE_A));
    const GRAPHEMES_LEN_LT: ValidationErr = ValidationErr::GraphemesLen(Operation::Lt(USIZE_VALUE_A));
    const GRAPHEMES_LEN_LE: ValidationErr = ValidationErr::GraphemesLen(Operation::Le(USIZE_VALUE_A));
    const GRAPHEMES_LEN_BTWN: ValidationErr = ValidationErr::GraphemesLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const LOWER_LEN_EQ: ValidationErr = ValidationErr::LowercaseLen(Operation::Eq(USIZE_VALUE_A));
    const LOWER_LEN_NE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ne(USIZE_VALUE_A));
    const LOWER_LEN_GT: ValidationErr = ValidationErr::LowercaseLen(Operation::Gt(USIZE_VALUE_A));
    const LOWER_LEN_GE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(USIZE_VALUE_A));
    const LOWER_LEN_LT: ValidationErr = ValidationErr::LowercaseLen(Operation::Lt(USIZE_VALUE_A));
    const LOWER_LEN_LE: ValidationErr = ValidationErr::LowercaseLen(Operation::Le(USIZE_VALUE_A));
    const LOWER_LEN_BTWN: ValidationErr = ValidationErr::LowercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const UPPER_LEN_EQ: ValidationErr = ValidationErr::UppercaseLen(Operation::Eq(USIZE_VALUE_A));
    const UPPER_LEN_NE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ne(USIZE_VALUE_A));
    const UPPER_LEN_GT: ValidationErr = ValidationErr::UppercaseLen(Operation::Gt(USIZE_VALUE_A));
    const UPPER_LEN_GE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ge(USIZE_VALUE_A));
    const UPPER_LEN_LT: ValidationErr = ValidationErr::UppercaseLen(Operation::Lt(USIZE_VALUE_A));
    const UPPER_LEN_LE: ValidationErr = ValidationErr::UppercaseLen(Operation::Le(USIZE_VALUE_A));
    const UPPER_LEN_BTWN: ValidationErr = ValidationErr::UppercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const NUMBERS_LEN_EQ: ValidationErr = ValidationErr::NumbersLen(Operation::Eq(USIZE_VALUE_A));
    const NUMBERS_LEN_NE: ValidationErr = ValidationErr::NumbersLen(Operation::Ne(USIZE_VALUE_A));
    const NUMBERS_LEN_GT: ValidationErr = ValidationErr::NumbersLen(Operation::Gt(USIZE_VALUE_A));
    const NUMBERS_LEN_GE: ValidationErr = ValidationErr::NumbersLen(Operation::Ge(USIZE_VALUE_A));
    const NUMBERS_LEN_LT: ValidationErr = ValidationErr::NumbersLen(Operation::Lt(USIZE_VALUE_A));
    const NUMBERS_LEN_LE: ValidationErr = ValidationErr::NumbersLen(Operation::Le(USIZE_VALUE_A));
    const NUMBERS_LEN_BTWN: ValidationErr = ValidationErr::NumbersLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const SYMBOLS_LEN_EQ: ValidationErr = ValidationErr::SymbolsLen(Operation::Eq(USIZE_VALUE_A));
    const SYMBOLS_LEN_NE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ne(USIZE_VALUE_A));
    const SYMBOLS_LEN_GT: ValidationErr = ValidationErr::SymbolsLen(Operation::Gt(USIZE_VALUE_A));
    const SYMBOLS_LEN_GE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ge(USIZE_VALUE_A));
    const SYMBOLS_LEN_LT: ValidationErr = ValidationErr::SymbolsLen(Operation::Lt(USIZE_VALUE_A));
    const SYMBOLS_LEN_LE: ValidationErr = ValidationErr::SymbolsLen(Operation::Le(USIZE_VALUE_A));
    const SYMBOLS_LEN_BTWN: ValidationErr = ValidationErr::SymbolsLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));

    fn mock_locale() -> Locale {
        Locale {
            required: "required".into(),
            u64: "u64".into(),
            i64: "i64".into(),
            f64: "f64".into(),
            usize: "usize".into(),
            isize: "isize".into(),
            bool: "bool".into(),
            str: "str".into(),
            email: "email".into(),
            date: "date".into(),
            time: "time".into(),
            date_time: "date_time".into(),
            eq: "== %value%".into(),
            ne: "!= %value%".into(),
            gt: "> %value%".into(),
            ge: ">= %value%".into(),
            lt: "< %value%".into(),
            le: "<= %value%".into(),
            btwn: "%value_a% <= <= %value_b%".into(),
            bytes_len_eq: "bytes_len == %value%".into(),
            bytes_len_ne: "bytes_len != %value%".into(),
            bytes_len_gt: "bytes_len > %value%".into(),
            bytes_len_ge: "bytes_len >= %value%".into(),
            bytes_len_lt: "bytes_len < %value%".into(),
            bytes_len_le: "bytes_len <= %value%".into(),
            bytes_len_btwn: "%value_a% <= bytes_len <= %value_b%".into(),
            chars_len_eq: "chars_len == %value%".into(),
            chars_len_ne: "chars_len != %value%".into(),
            chars_len_gt: "chars_len > %value%".into(),
            chars_len_ge: "chars_len >= %value%".into(),
            chars_len_lt: "chars_len < %value%".into(),
            chars_len_le: "chars_len <= %value%".into(),
            chars_len_btwn: "%value_a% <= chars_len <= %value_b%".into(),
            graphemes_len_eq: "graphemes_len == %value%".into(),
            graphemes_len_ne: "graphemes_len != %value%".into(),
            graphemes_len_gt: "graphemes_len > %value%".into(),
            graphemes_len_ge: "graphemes_len >= %value%".into(),
            graphemes_len_lt: "graphemes_len < %value%".into(),
            graphemes_len_le: "graphemes_len <= %value%".into(),
            graphemes_len_btwn: "%value_a% <= graphemes_len <= %value_b%".into(),
            lowercase_len_eq: "lowercase_len == %value%".into(),
            lowercase_len_ne: "lowercase_len != %value%".into(),
            lowercase_len_gt: "lowercase_len > %value%".into(),
            lowercase_len_ge: "lowercase_len >= %value%".into(),
            lowercase_len_lt: "lowercase_len < %value%".into(),
            lowercase_len_le: "lowercase_len <= %value%".into(),
            lowercase_len_btwn: "%value_a% <= lowercase_len <= %value_b%".into(),
            uppercase_len_eq: "uppercase_len == %value%".into(),
            uppercase_len_ne: "uppercase_len != %value%".into(),
            uppercase_len_gt: "uppercase_len > %value%".into(),
            uppercase_len_ge: "uppercase_len >= %value%".into(),
            uppercase_len_lt: "uppercase_len < %value%".into(),
            uppercase_len_le: "uppercase_len <= %value%".into(),
            uppercase_len_btwn: "%value_a% <= uppercase_len <= %value_b%".into(),
            number_len_eq: "number_len == %value%".into(),
            number_len_ne: "number_len != %value%".into(),
            number_len_gt: "number_len > %value%".into(),
            number_len_ge: "number_len >= %value%".into(),
            number_len_lt: "number_len < %value%".into(),
            number_len_le: "number_len <= %value%".into(),
            number_len_btwn: "%value_a% <= number_len <= %value_b%".into(),
            symbols_eq: "symbols == %value%".into(),
            symbols_ne: "symbols != %value%".into(),
            symbols_gt: "symbols > %value%".into(),
            symbols_ge: "symbols >= %value%".into(),
            symbols_lt: "symbols < %value%".into(),
            symbols_le: "symbols <= %value%".into(),
            symbols_btwn: "%value_a% <= symbols <= %value_b%".into(),
            enumerated: "enum %value%".into(),
        }
    }

    #[test]
    fn test_localize_validation_err() {
        let l = mock_locale();

        let str_value_a = Operand::Value(OperandValue::from("aurorae"));
        let str_value_b = Operand::Value(OperandValue::from("crespúculum"));

        let operation_str_eq = ValidationErr::Operation(Operation::Eq(str_value_a.clone()));
        let operation_str_ne = ValidationErr::Operation(Operation::Ne(str_value_a.clone()));
        let operation_str_gt = ValidationErr::Operation(Operation::Gt(str_value_a.clone()));
        let operation_str_ge = ValidationErr::Operation(Operation::Ge(str_value_a.clone()));
        let operation_str_lt = ValidationErr::Operation(Operation::Lt(str_value_a.clone()));
        let operation_str_le = ValidationErr::Operation(Operation::Le(str_value_a.clone()));
        let operation_str_btwn = ValidationErr::Operation(Operation::Btwn(str_value_a, str_value_b));

        let enum_usize = ValidationErr::Enumerated(EnumValues::USize(vec![0, 1, 2, 3, 4, 5]));
        let enum_isize = ValidationErr::Enumerated(EnumValues::ISize(vec![-2, -1, 0, 1, 2]));
        let enum_str = ValidationErr::Enumerated(EnumValues::Str(vec!["APPLE".into(), "GRAPE".into(), "PEAR".into()]));

        assert_eq!(localize_validation_err(&REQUIRED, &l), "required".to_string());
        assert_eq!(localize_validation_err(&U64, &l), "u64".to_string());
        assert_eq!(localize_validation_err(&I64, &l), "i64".to_string());
        assert_eq!(localize_validation_err(&F64, &l), "f64".to_string());
        assert_eq!(localize_validation_err(&USIZE, &l), "usize".to_string());
        assert_eq!(localize_validation_err(&ISIZE, &l), "isize".to_string());
        assert_eq!(localize_validation_err(&BOOL, &l), "bool".to_string());
        assert_eq!(localize_validation_err(&STR, &l), "str".to_string());
        assert_eq!(localize_validation_err(&EMAIL, &l), "email".to_string());
        assert_eq!(localize_validation_err(&DATE, &l), "date".to_string());
        assert_eq!(localize_validation_err(&TIME, &l), "time".to_string());
        assert_eq!(localize_validation_err(&DATE_TIME, &l), "date_time".to_string());

        assert_eq!(localize_validation_err(&OP_U64_EQ, &l), "== 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_NE, &l), "!= 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_GT, &l), "> 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_GE, &l), ">= 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_LT, &l), "< 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_LE, &l), "<= 34".to_string());
        assert_eq!(localize_validation_err(&OP_U64_BTWN, &l), "34 <= <= 43".to_string());

        assert_eq!(localize_validation_err(&OP_I64_EQ, &l), "== -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_NE, &l), "!= -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_GT, &l), "> -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_GE, &l), ">= -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_LT, &l), "< -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_LE, &l), "<= -4".to_string());
        assert_eq!(localize_validation_err(&OP_I64_BTWN, &l), "-4 <= <= 4".to_string());

        assert_eq!(localize_validation_err(&OP_F64_EQ, &l), "== -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_NE, &l), "!= -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_GT, &l), "> -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_GE, &l), ">= -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_LT, &l), "< -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_LE, &l), "<= -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_F64_BTWN, &l), "-4.6 <= <= -2.4".to_string());

        assert_eq!(localize_validation_err(&OP_USIZE_EQ, &l), "== 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_NE, &l), "!= 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_GT, &l), "> 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_GE, &l), ">= 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_LT, &l), "< 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_LE, &l), "<= 27".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE_BTWN, &l), "27 <= <= 39".to_string());

        assert_eq!(localize_validation_err(&OP_ISIZE_EQ, &l), "== -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_NE, &l), "!= -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_GT, &l), "> -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_GE, &l), ">= -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_LT, &l), "< -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_LE, &l), "<= -93".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE_BTWN, &l), "-93 <= <= -72".to_string());

        assert_eq!(localize_validation_err(&OP_BOOL_EQ, &l), "== false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_NE, &l), "!= false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_GT, &l), "> false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_GE, &l), ">= false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_LT, &l), "< false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_LE, &l), "<= false".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL_BTWN, &l), "false <= <= true".to_string());

        assert_eq!(localize_validation_err(&operation_str_eq, &l), r#"== "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_ne, &l), r#"!= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_gt, &l), r#"> "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_ge, &l), r#">= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_lt, &l), r#"< "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_le, &l), r#"<= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&operation_str_btwn, &l), r#""aurorae" <= <= "crespúculum""#.to_string());

        assert_eq!(localize_validation_err(&BYTES_LEN_EQ, &l), "bytes_len == 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_NE, &l), "bytes_len != 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GT, &l), "bytes_len > 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GE, &l), "bytes_len >= 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LT, &l), "bytes_len < 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LE, &l), "bytes_len <= 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_BTWN, &l), "27 <= bytes_len <= 39".to_string());

        assert_eq!(localize_validation_err(&CHARS_LEN_EQ, &l), "chars_len == 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_NE, &l), "chars_len != 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GT, &l), "chars_len > 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GE, &l), "chars_len >= 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LT, &l), "chars_len < 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LE, &l), "chars_len <= 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_BTWN, &l), "27 <= chars_len <= 39".to_string());

        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_EQ, &l), "graphemes_len == 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_NE, &l), "graphemes_len != 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GT, &l), "graphemes_len > 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GE, &l), "graphemes_len >= 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LT, &l), "graphemes_len < 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LE, &l), "graphemes_len <= 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_BTWN, &l), "27 <= graphemes_len <= 39".to_string());

        assert_eq!(localize_validation_err(&LOWER_LEN_EQ, &l), "lowercase_len == 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_NE, &l), "lowercase_len != 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GT, &l), "lowercase_len > 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GE, &l), "lowercase_len >= 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LT, &l), "lowercase_len < 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LE, &l), "lowercase_len <= 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_BTWN, &l), "27 <= lowercase_len <= 39".to_string());

        assert_eq!(localize_validation_err(&UPPER_LEN_EQ, &l), "uppercase_len == 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_NE, &l), "uppercase_len != 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GT, &l), "uppercase_len > 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GE, &l), "uppercase_len >= 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LT, &l), "uppercase_len < 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LE, &l), "uppercase_len <= 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_BTWN, &l), "27 <= uppercase_len <= 39".to_string());

        assert_eq!(localize_validation_err(&NUMBERS_LEN_EQ, &l), "number_len == 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_NE, &l), "number_len != 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GT, &l), "number_len > 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GE, &l), "number_len >= 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LT, &l), "number_len < 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LE, &l), "number_len <= 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_BTWN, &l), "27 <= number_len <= 39".to_string());

        assert_eq!(localize_validation_err(&SYMBOLS_LEN_EQ, &l), "symbols == 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_NE, &l), "symbols != 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GT, &l), "symbols > 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GE, &l), "symbols >= 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LT, &l), "symbols < 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LE, &l), "symbols <= 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_BTWN, &l), "27 <= symbols <= 39".to_string());

        assert_eq!(localize_validation_err(&enum_usize, &l), "enum [ 0, 1, 2, 3, 4, 5 ]".to_string());
        assert_eq!(localize_validation_err(&enum_isize, &l), "enum [ -2, -1, 0, 1, 2 ]".to_string());
        assert_eq!(localize_validation_err(&enum_str, &l), r#"enum [ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn test_schema_err_arr_to_locale() {
        let locale = mock_locale();
        let err = SchemaErr::arr([REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))]);
        let localized_err = SchemaLocalizedErr::Arr(vec!["required".into(), "bool".into(), "== true".into()]);
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    #[test]
    fn test_schema_err_obj_to_locale() {
        let locale = mock_locale();
        let err = SchemaErr::Obj(BTreeMap::from([
            (
                "name".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Paul McCartney"))))]),
            ),
            (
                "birthdate".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("1942-06-18"))))]),
            ),
            ("alive".into(), SchemaErr::arr([REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))])),
            (
                "bands".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("The Beatles"))))]),
            ),
        ]));
        let localized_err = SchemaLocalizedErr::Obj(BTreeMap::from([
            ("name".into(), SchemaLocalizedErr::Arr(vec!["required".into(), "str".into(), r#"== "Paul McCartney""#.into()])),
            ("birthdate".into(), SchemaLocalizedErr::Arr(vec!["required".into(), "str".into(), r#"== "1942-06-18""#.into()])),
            ("alive".into(), SchemaLocalizedErr::Arr(vec!["required".into(), "bool".into(), "== true".into()])),
            ("bands".into(), SchemaLocalizedErr::Arr(vec!["required".into(), "str".into(), r#"== "The Beatles""#.into()])),
        ]));
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }
}
