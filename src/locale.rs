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
        _ => "".into(),
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

