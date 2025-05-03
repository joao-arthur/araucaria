use std::collections::BTreeMap;

use crate::{
    error::{SchemaErr, ValidationErr},
    operation::{Operand, Operation},
};

pub struct Locale {
    pub required: String,
    pub u64: String,
    pub i64: String,
    pub f64: String,
    pub usize: String,
    pub isize: String,
    pub bool: String,
    pub str: String,
    pub email: String,
    pub date: String,
    pub time: String,
    pub date_time: String,
    pub eq: String,
    pub ne: String,
    pub gt: String,
    pub lt: String,
    pub ge: String,
    pub le: String,
    pub btwn: String,
    pub eq_field: String,
    pub ne_field: String,
    pub gt_field: String,
    pub lt_field: String,
    pub ge_field: String,
    pub le_field: String,
    pub bytes_len_eq: String,
    pub bytes_len_ne: String,
    pub bytes_len_gt: String,
    pub bytes_len_ge: String,
    pub bytes_len_lt: String,
    pub bytes_len_le: String,
    pub bytes_len_btwn: String,
    pub chars_len_eq: String,
    pub chars_len_ne: String,
    pub chars_len_gt: String,
    pub chars_len_ge: String,
    pub chars_len_lt: String,
    pub chars_len_le: String,
    pub chars_len_btwn: String,
    pub graphemes_len_eq: String,
    pub graphemes_len_ne: String,
    pub graphemes_len_gt: String,
    pub graphemes_len_ge: String,
    pub graphemes_len_lt: String,
    pub graphemes_len_le: String,
    pub graphemes_len_btwn: String,
    pub lowercase_len_eq: String,
    pub lowercase_len_ne: String,
    pub lowercase_len_gt: String,
    pub lowercase_len_ge: String,
    pub lowercase_len_lt: String,
    pub lowercase_len_le: String,
    pub lowercase_len_btwn: String,
    pub uppercase_len_eq: String,
    pub uppercase_len_ne: String,
    pub uppercase_len_gt: String,
    pub uppercase_len_ge: String,
    pub uppercase_len_lt: String,
    pub uppercase_len_le: String,
    pub uppercase_len_btwn: String,
    pub number_len_eq: String,
    pub number_len_ne: String,
    pub number_len_gt: String,
    pub number_len_ge: String,
    pub number_len_lt: String,
    pub number_len_le: String,
    pub number_len_btwn: String,
    pub symbols_eq: String,
    pub symbols_ne: String,
    pub symbols_gt: String,
    pub symbols_ge: String,
    pub symbols_lt: String,
    pub symbols_le: String,
    pub symbols_btwn: String,
    pub enumerated: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaLocalizedErr {
    Validation(Vec<String>),
    Arr(Vec<SchemaLocalizedErr>),
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
            Operation::Eq(operand) => match operand {
                Operand::Value(value) => locale.eq.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.eq_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
            Operation::Ne(operand) => match operand {
                Operand::Value(value) => locale.ne.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.ne_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
            Operation::Gt(operand) => match operand {
                Operand::Value(value) => locale.gt.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.gt_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
            Operation::Ge(operand) => match operand {
                Operand::Value(value) => locale.ge.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.ge_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
            Operation::Lt(operand) => match operand {
                Operand::Value(value) => locale.lt.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.lt_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
            Operation::Le(operand) => match operand {
                Operand::Value(value) => locale.le.replace("%value%", &value.to_string()),
                Operand::FieldPath(field) => locale.le_field.replace("%value%", &("\"".to_string() + field + "\"")),
            },
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
        SchemaErr::Validation(v) => SchemaLocalizedErr::Validation(v.iter().map(|item| localize_validation_err(item, locale)).collect()),
        SchemaErr::Arr(arr) => {
            let mut result: Vec<SchemaLocalizedErr> = Vec::new();
            for item in arr {
                result.push(localize_schema_err(item, locale));
            }
            SchemaLocalizedErr::Arr(result)
        }
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
    use std::{collections::BTreeMap, sync::LazyLock};

    use crate::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::EnumValues,
    };

    use super::{Locale, SchemaLocalizedErr, localize_schema_err, localize_validation_err};

    const STR_VALUES: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];
    const USIZE_VALUES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const ISIZE_VALUES: [isize; 5] = [-2, -1, 0, 1, 2];

    const U64_VALUE: Operand = Operand::Value(OperandValue::U64(34));
    const U64_VALUE_B: Operand = Operand::Value(OperandValue::U64(43));
    const I64_VALUE: Operand = Operand::Value(OperandValue::I64(-4));
    const I64_VALUE_B: Operand = Operand::Value(OperandValue::I64(4));
    const F64_VALUE: Operand = Operand::Value(OperandValue::F64(-4.6));
    const F64_VALUE_B: Operand = Operand::Value(OperandValue::F64(-2.4));
    const USIZE_VALUE: Operand = Operand::Value(OperandValue::USize(27));
    const USIZE_VALUE_B: Operand = Operand::Value(OperandValue::USize(39));
    const ISIZE_VALUE: Operand = Operand::Value(OperandValue::ISize(-93));
    const ISIZE_VALUE_B: Operand = Operand::Value(OperandValue::ISize(-72));
    const BOOL_VALUE: Operand = Operand::Value(OperandValue::Bool(false));
    const BOOL_VALUE_B: Operand = Operand::Value(OperandValue::Bool(true));
    const STR_VALUE: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("aurorae")));
    const STR_VALUE_B: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("crespúculum")));
    const FIELD_PATH: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath("user.account.info.details.user_name".into()));

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

    const OP_U64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(U64_VALUE));
    const OP_U64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(U64_VALUE));
    const OP_U64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(U64_VALUE));
    const OP_U64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(U64_VALUE));
    const OP_U64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(U64_VALUE));
    const OP_U64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(U64_VALUE));
    const OP_U64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(U64_VALUE, U64_VALUE_B));

    const OP_I64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(I64_VALUE));
    const OP_I64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(I64_VALUE));
    const OP_I64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(I64_VALUE));
    const OP_I64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(I64_VALUE));
    const OP_I64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(I64_VALUE));
    const OP_I64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(I64_VALUE));
    const OP_I64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(I64_VALUE, I64_VALUE_B));

    const OP_F64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(F64_VALUE));
    const OP_F64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(F64_VALUE));
    const OP_F64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(F64_VALUE));
    const OP_F64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(F64_VALUE));
    const OP_F64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(F64_VALUE));
    const OP_F64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(F64_VALUE));
    const OP_F64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(F64_VALUE, F64_VALUE_B));

    const OP_USIZE_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(USIZE_VALUE));
    const OP_USIZE_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(USIZE_VALUE));
    const OP_USIZE_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(USIZE_VALUE));
    const OP_USIZE_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(USIZE_VALUE));
    const OP_USIZE_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(USIZE_VALUE));
    const OP_USIZE_LE: ValidationErr = ValidationErr::Operation(Operation::Le(USIZE_VALUE));
    const OP_USIZE_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const OP_ISIZE_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(ISIZE_VALUE));
    const OP_ISIZE_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(ISIZE_VALUE));
    const OP_ISIZE_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(ISIZE_VALUE));
    const OP_ISIZE_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(ISIZE_VALUE));
    const OP_ISIZE_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(ISIZE_VALUE));
    const OP_ISIZE_LE: ValidationErr = ValidationErr::Operation(Operation::Le(ISIZE_VALUE));
    const OP_ISIZE_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(ISIZE_VALUE, ISIZE_VALUE_B));

    const OP_BOOL_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(BOOL_VALUE));
    const OP_BOOL_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(BOOL_VALUE));
    const OP_BOOL_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(BOOL_VALUE));
    const OP_BOOL_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(BOOL_VALUE));
    const OP_BOOL_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(BOOL_VALUE));
    const OP_BOOL_LE: ValidationErr = ValidationErr::Operation(Operation::Le(BOOL_VALUE));
    const OP_BOOL_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(BOOL_VALUE, BOOL_VALUE_B));

    const OP_STR_EQ: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(STR_VALUE.clone())));
    const OP_STR_NE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Ne(STR_VALUE.clone())));
    const OP_STR_GT: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Gt(STR_VALUE.clone())));
    const OP_STR_GE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Ge(STR_VALUE.clone())));
    const OP_STR_LT: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Lt(STR_VALUE.clone())));
    const OP_STR_LE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Le(STR_VALUE.clone())));
    const OP_STR_BTWN: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Btwn(STR_VALUE.clone(), STR_VALUE_B.clone())));

    const OP_FIELD_EQ: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(FIELD_PATH.clone())));
    const OP_FIELD_NE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Ne(FIELD_PATH.clone())));
    const OP_FIELD_GT: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Gt(FIELD_PATH.clone())));
    const OP_FIELD_GE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Ge(FIELD_PATH.clone())));
    const OP_FIELD_LT: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Lt(FIELD_PATH.clone())));
    const OP_FIELD_LE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Le(FIELD_PATH.clone())));

    const BYTES_LEN_EQ: ValidationErr = ValidationErr::BytesLen(Operation::Eq(USIZE_VALUE));
    const BYTES_LEN_NE: ValidationErr = ValidationErr::BytesLen(Operation::Ne(USIZE_VALUE));
    const BYTES_LEN_GT: ValidationErr = ValidationErr::BytesLen(Operation::Gt(USIZE_VALUE));
    const BYTES_LEN_GE: ValidationErr = ValidationErr::BytesLen(Operation::Ge(USIZE_VALUE));
    const BYTES_LEN_LT: ValidationErr = ValidationErr::BytesLen(Operation::Lt(USIZE_VALUE));
    const BYTES_LEN_LE: ValidationErr = ValidationErr::BytesLen(Operation::Le(USIZE_VALUE));
    const BYTES_LEN_BTWN: ValidationErr = ValidationErr::BytesLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const CHARS_LEN_EQ: ValidationErr = ValidationErr::CharsLen(Operation::Eq(USIZE_VALUE));
    const CHARS_LEN_NE: ValidationErr = ValidationErr::CharsLen(Operation::Ne(USIZE_VALUE));
    const CHARS_LEN_GT: ValidationErr = ValidationErr::CharsLen(Operation::Gt(USIZE_VALUE));
    const CHARS_LEN_GE: ValidationErr = ValidationErr::CharsLen(Operation::Ge(USIZE_VALUE));
    const CHARS_LEN_LT: ValidationErr = ValidationErr::CharsLen(Operation::Lt(USIZE_VALUE));
    const CHARS_LEN_LE: ValidationErr = ValidationErr::CharsLen(Operation::Le(USIZE_VALUE));
    const CHARS_LEN_BTWN: ValidationErr = ValidationErr::CharsLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const GRAPHEMES_LEN_EQ: ValidationErr = ValidationErr::GraphemesLen(Operation::Eq(USIZE_VALUE));
    const GRAPHEMES_LEN_NE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ne(USIZE_VALUE));
    const GRAPHEMES_LEN_GT: ValidationErr = ValidationErr::GraphemesLen(Operation::Gt(USIZE_VALUE));
    const GRAPHEMES_LEN_GE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ge(USIZE_VALUE));
    const GRAPHEMES_LEN_LT: ValidationErr = ValidationErr::GraphemesLen(Operation::Lt(USIZE_VALUE));
    const GRAPHEMES_LEN_LE: ValidationErr = ValidationErr::GraphemesLen(Operation::Le(USIZE_VALUE));
    const GRAPHEMES_LEN_BTWN: ValidationErr = ValidationErr::GraphemesLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const LOWER_LEN_EQ: ValidationErr = ValidationErr::LowercaseLen(Operation::Eq(USIZE_VALUE));
    const LOWER_LEN_NE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ne(USIZE_VALUE));
    const LOWER_LEN_GT: ValidationErr = ValidationErr::LowercaseLen(Operation::Gt(USIZE_VALUE));
    const LOWER_LEN_GE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(USIZE_VALUE));
    const LOWER_LEN_LT: ValidationErr = ValidationErr::LowercaseLen(Operation::Lt(USIZE_VALUE));
    const LOWER_LEN_LE: ValidationErr = ValidationErr::LowercaseLen(Operation::Le(USIZE_VALUE));
    const LOWER_LEN_BTWN: ValidationErr = ValidationErr::LowercaseLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const UPPER_LEN_EQ: ValidationErr = ValidationErr::UppercaseLen(Operation::Eq(USIZE_VALUE));
    const UPPER_LEN_NE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ne(USIZE_VALUE));
    const UPPER_LEN_GT: ValidationErr = ValidationErr::UppercaseLen(Operation::Gt(USIZE_VALUE));
    const UPPER_LEN_GE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ge(USIZE_VALUE));
    const UPPER_LEN_LT: ValidationErr = ValidationErr::UppercaseLen(Operation::Lt(USIZE_VALUE));
    const UPPER_LEN_LE: ValidationErr = ValidationErr::UppercaseLen(Operation::Le(USIZE_VALUE));
    const UPPER_LEN_BTWN: ValidationErr = ValidationErr::UppercaseLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const NUMBERS_LEN_EQ: ValidationErr = ValidationErr::NumbersLen(Operation::Eq(USIZE_VALUE));
    const NUMBERS_LEN_NE: ValidationErr = ValidationErr::NumbersLen(Operation::Ne(USIZE_VALUE));
    const NUMBERS_LEN_GT: ValidationErr = ValidationErr::NumbersLen(Operation::Gt(USIZE_VALUE));
    const NUMBERS_LEN_GE: ValidationErr = ValidationErr::NumbersLen(Operation::Ge(USIZE_VALUE));
    const NUMBERS_LEN_LT: ValidationErr = ValidationErr::NumbersLen(Operation::Lt(USIZE_VALUE));
    const NUMBERS_LEN_LE: ValidationErr = ValidationErr::NumbersLen(Operation::Le(USIZE_VALUE));
    const NUMBERS_LEN_BTWN: ValidationErr = ValidationErr::NumbersLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    const SYMBOLS_LEN_EQ: ValidationErr = ValidationErr::SymbolsLen(Operation::Eq(USIZE_VALUE));
    const SYMBOLS_LEN_NE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ne(USIZE_VALUE));
    const SYMBOLS_LEN_GT: ValidationErr = ValidationErr::SymbolsLen(Operation::Gt(USIZE_VALUE));
    const SYMBOLS_LEN_GE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ge(USIZE_VALUE));
    const SYMBOLS_LEN_LT: ValidationErr = ValidationErr::SymbolsLen(Operation::Lt(USIZE_VALUE));
    const SYMBOLS_LEN_LE: ValidationErr = ValidationErr::SymbolsLen(Operation::Le(USIZE_VALUE));
    const SYMBOLS_LEN_BTWN: ValidationErr = ValidationErr::SymbolsLen(Operation::Btwn(USIZE_VALUE, USIZE_VALUE_B));

    static ENUM_USIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(USIZE_VALUES)));
    static ENUM_ISIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(ISIZE_VALUES)));
    static ENUM_STR: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(STR_VALUES)));

    static NAME: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("Paul McCartney")));
    static BIRTHDATE: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("1942-06-18")));
    static ALIVE: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::Bool(true)));
    static BANDS: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("The Beatles")));

    static OP_NAME: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(NAME.clone())));
    static OP_BIRTHDATE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(BIRTHDATE.clone())));
    static OP_ALIVE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(ALIVE.clone())));
    static OP_BANDS: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Eq(BANDS.clone())));

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
            eq_field: "== field %value%".into(),
            ne_field: "!= field %value%".into(),
            gt_field: "> field %value%".into(),
            ge_field: ">= field %value%".into(),
            lt_field: "< field %value%".into(),
            le_field: "<= field %value%".into(),
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

        assert_eq!(localize_validation_err(&OP_STR_EQ, &l), r#"== "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_NE, &l), r#"!= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_GT, &l), r#"> "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_GE, &l), r#">= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_LT, &l), r#"< "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_LE, &l), r#"<= "aurorae""#.to_string());
        assert_eq!(localize_validation_err(&OP_STR_BTWN, &l), r#""aurorae" <= <= "crespúculum""#.to_string());

        assert_eq!(localize_validation_err(&OP_FIELD_EQ, &l), r#"== field "user.account.info.details.user_name""#.to_string());
        assert_eq!(localize_validation_err(&OP_FIELD_NE, &l), r#"!= field "user.account.info.details.user_name""#.to_string());
        assert_eq!(localize_validation_err(&OP_FIELD_GT, &l), r#"> field "user.account.info.details.user_name""#.to_string());
        assert_eq!(localize_validation_err(&OP_FIELD_GE, &l), r#">= field "user.account.info.details.user_name""#.to_string());
        assert_eq!(localize_validation_err(&OP_FIELD_LT, &l), r#"< field "user.account.info.details.user_name""#.to_string());
        assert_eq!(localize_validation_err(&OP_FIELD_LE, &l), r#"<= field "user.account.info.details.user_name""#.to_string());

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

        assert_eq!(localize_validation_err(&ENUM_USIZE, &l), "enum [ 0, 1, 2, 3, 4, 5 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_ISIZE, &l), "enum [ -2, -1, 0, 1, 2 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_STR, &l), r#"enum [ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn localize_schema_err_validation() {
        let locale = mock_locale();
        let err = SchemaErr::validation([REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))]);
        let localized_err = SchemaLocalizedErr::Validation(vec!["required".into(), "bool".into(), "== true".into()]);
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    #[test]
    fn localize_schema_err_arr() {
        let locale = mock_locale();
        let err = SchemaErr::arr([
            SchemaErr::validation([REQUIRED, STR, OP_NAME.clone()]),
            SchemaErr::validation([REQUIRED, STR, OP_BIRTHDATE.clone()]),
            SchemaErr::validation([REQUIRED, BOOL, OP_ALIVE.clone()]),
            SchemaErr::validation([REQUIRED, STR, OP_BANDS.clone()]),
        ]);
        let localized_err = SchemaLocalizedErr::Arr(vec![
            SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "Paul McCartney""#.into()]),
            SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "1942-06-18""#.into()]),
            SchemaLocalizedErr::Validation(vec!["required".into(), "bool".into(), "== true".into()]),
            SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "The Beatles""#.into()]),
        ]);
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    #[test]
    fn localize_schema_err_obj() {
        let locale = mock_locale();
        let err = SchemaErr::obj([
            ("name".into(), SchemaErr::validation([REQUIRED, STR, OP_NAME.clone()])),
            ("birthdate".into(), SchemaErr::validation([REQUIRED, STR, OP_BIRTHDATE.clone()])),
            ("alive".into(), SchemaErr::validation([REQUIRED, BOOL, OP_ALIVE.clone()])),
            ("bands".into(), SchemaErr::validation([REQUIRED, STR, OP_BANDS.clone()])),
        ]);
        let localized_err = SchemaLocalizedErr::Obj(BTreeMap::from([
            ("name".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "Paul McCartney""#.into()])),
            ("birthdate".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "1942-06-18""#.into()])),
            ("alive".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "bool".into(), "== true".into()])),
            ("bands".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "The Beatles""#.into()])),
        ]));
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    #[test]
    fn localize_schema_err_nested() {
        let locale = mock_locale();
        let err = SchemaErr::obj([(
            "user".into(),
            SchemaErr::arr([
                SchemaErr::validation([REQUIRED]),
                SchemaErr::obj([
                    ("name".into(), SchemaErr::validation([REQUIRED, STR, OP_NAME.clone()])),
                    ("birthdate".into(), SchemaErr::validation([REQUIRED, STR, OP_BIRTHDATE.clone()])),
                    ("alive".into(), SchemaErr::validation([REQUIRED, BOOL, OP_ALIVE.clone()])),
                    ("bands".into(), SchemaErr::validation([REQUIRED, STR, OP_BANDS.clone()])),
                ]),
            ]),
        )]);
        let localized_err = SchemaLocalizedErr::Obj(BTreeMap::from([(
            "user".into(),
            SchemaLocalizedErr::Arr(vec![
                SchemaLocalizedErr::Validation(vec!["required".into()]),
                SchemaLocalizedErr::Obj(BTreeMap::from([
                    ("name".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "Paul McCartney""#.into()])),
                    ("birthdate".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "1942-06-18""#.into()])),
                    ("alive".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "bool".into(), "== true".into()])),
                    ("bands".into(), SchemaLocalizedErr::Validation(vec!["required".into(), "str".into(), r#"== "The Beatles""#.into()])),
                ])),
            ]),
        )]));
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }
}
