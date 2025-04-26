use std::cmp::Ordering;

use crate::{path::resolve_path, value::Value};

#[derive(Debug, PartialEq, Clone)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
}

impl From<u64> for OperandValue {
    fn from(value: u64) -> Self {
        OperandValue::U64(value)
    }
}

impl From<i64> for OperandValue {
    fn from(value: i64) -> Self {
        OperandValue::I64(value)
    }
}

impl From<f64> for OperandValue {
    fn from(value: f64) -> Self {
        OperandValue::F64(value)
    }
}

impl From<usize> for OperandValue {
    fn from(value: usize) -> Self {
        OperandValue::USize(value)
    }
}

impl From<isize> for OperandValue {
    fn from(value: isize) -> Self {
        OperandValue::ISize(value)
    }
}

impl From<bool> for OperandValue {
    fn from(value: bool) -> Self {
        OperandValue::Bool(value)
    }
}

impl From<&str> for OperandValue {
    fn from(value: &str) -> Self {
        OperandValue::Str(value.into())
    }
}

impl PartialOrd for OperandValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let OperandValue::U64(a) = self {
            if let OperandValue::U64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::I64(a) = self {
            if let OperandValue::I64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::F64(a) = self {
            if let OperandValue::F64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::USize(a) = self {
            if let OperandValue::USize(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::ISize(a) = self {
            if let OperandValue::ISize(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Bool(a) = self {
            if let OperandValue::Bool(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Str(a) = self {
            if let OperandValue::Str(b) = other {
                return a.partial_cmp(b);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Value(OperandValue),
    FieldPath(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Eq(Operand),
    Ne(Operand),
    Gt(Operand),
    Ge(Operand),
    Lt(Operand),
    Le(Operand),
    Btwn(Operand, Operand),
}

fn compare_eq(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Greater => Some(Err(())),
        Ordering::Equal => Some(Ok(())),
    }
}

fn compare_ne(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Greater => Some(Ok(())),
        Ordering::Equal => Some(Err(())),
    }
}

fn compare_gt(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Equal => Some(Err(())),
        Ordering::Greater => Some(Ok(())),
    }
}

fn compare_ge(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less => Some(Err(())),
        Ordering::Equal | Ordering::Greater => Some(Ok(())),
    }
}

fn compare_lt(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less => Some(Ok(())),
        Ordering::Equal | Ordering::Greater => Some(Err(())),
    }
}

fn compare_le(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Equal => Some(Ok(())),
        Ordering::Greater => Some(Err(())),
    }
}

fn value_to_operand_value(value: &Value) -> Option<OperandValue> {
    match value {
        Value::U64(val) => Some(OperandValue::U64(*val)),
        Value::I64(val) => Some(OperandValue::I64(*val)),
        Value::F64(val) => Some(OperandValue::F64(*val)),
        Value::USize(val) => Some(OperandValue::USize(*val)),
        Value::ISize(val) => Some(OperandValue::ISize(*val)),
        Value::Bool(val) => Some(OperandValue::Bool(*val)),
        Value::Str(val) => Some(OperandValue::Str(val.clone())),
        _ => None,
    }
}

fn resolve_operand(root: &Value, operation: &Operand) -> Option<OperandValue> {
    match operation {
        Operand::Value(value) => Some(value.clone()),
        Operand::FieldPath(field_path) => {
            let field = resolve_path(root, field_path)?;
            value_to_operand_value(&field)
        }
    }
}

pub fn compare(operation: &Operation, value_a: &OperandValue, root: &Value) -> Option<Result<(), ()>> {
    match operation {
        Operation::Eq(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_eq(value_a, &value_b)
        }
        Operation::Ne(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_ne(value_a, &value_b)
        }
        Operation::Gt(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_gt(value_a, &value_b)
        }
        Operation::Ge(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_ge(value_a, &value_b)
        }
        Operation::Lt(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_lt(value_a, &value_b)
        }
        Operation::Le(operand) => {
            let value_b = resolve_operand(root, operand)?;
            compare_le(value_a, &value_b)
        }
        Operation::Btwn(operand_a, operand_b) => {
            let value_operand_a = resolve_operand(root, operand_a)?;
            let value_operand_b = resolve_operand(root, operand_b)?;
            if let Some(Ok(())) = compare_ge(value_a, &value_operand_a) {
                if let Some(Ok(())) = compare_le(value_a, &value_operand_b) { Some(Ok(())) } else { Some(Err(())) }
            } else {
                Some(Err(()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, sync::LazyLock};

    use crate::value::Value;

    use super::{Operand, OperandValue, Operation, compare};

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([
            (
                "u64".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
                ]),
            ),
            (
                "i64".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::I64(-42))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
                ]),
            ),
            (
                "f64".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::F64(-42.5))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
                ]),
            ),
            (
                "usize".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::USize(32))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::USize(42))])),
                ]),
            ),
            (
                "isize".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::ISize(-42))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::ISize(42))])),
                ]),
            ),
            (
                "bool".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                ]),
            ),
            (
                "str".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("naruto"))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("sasuke"))])),
                ]),
            ),
        ]))
    });

    #[test]
    fn test_operand_value_from() {
        assert_eq!(OperandValue::from(8_u64), OperandValue::U64(8));
        assert_eq!(OperandValue::from(-3_i64), OperandValue::I64(-3));
        assert_eq!(OperandValue::from(-9.8), OperandValue::F64(-9.8));
        assert_eq!(OperandValue::from(183_usize), OperandValue::USize(183));
        assert_eq!(OperandValue::from(-892_isize), OperandValue::ISize(-892));
        assert_eq!(OperandValue::from(false), OperandValue::Bool(false));
        assert_eq!(OperandValue::from("in vino veritas"), OperandValue::Str("in vino veritas".into()));
    }

    #[test]
    fn test_operand_value_u64() {
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(41), false);
        assert!(OperandValue::U64(42) == OperandValue::U64(42));
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(43), false);
        assert!(OperandValue::U64(42) != OperandValue::U64(41));
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(42), false);
        assert!(OperandValue::U64(42) != OperandValue::U64(43));
        assert!(OperandValue::U64(42) > OperandValue::U64(41));
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(43), false);
        assert!(OperandValue::U64(42) >= OperandValue::U64(41));
        assert!(OperandValue::U64(42) >= OperandValue::U64(42));
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(42), false);
        assert!(OperandValue::U64(42) < OperandValue::U64(43));
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(41), false);
        assert!(OperandValue::U64(42) <= OperandValue::U64(42));
        assert!(OperandValue::U64(42) <= OperandValue::U64(43));
    }

    #[test]
    fn test_operand_value_i64() {
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-43), false);
        assert!(OperandValue::I64(-42) == OperandValue::I64(-42));
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-41), false);
        assert!(OperandValue::I64(-42) != OperandValue::I64(-43));
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-42), false);
        assert!(OperandValue::I64(-42) != OperandValue::I64(-41));
        assert!(OperandValue::I64(-42) > OperandValue::I64(-43));
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-41), false);
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-43));
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-42));
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-42), false);
        assert!(OperandValue::I64(-42) < OperandValue::I64(-41));
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-43), false);
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-42));
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-41));
    }

    #[test]
    fn test_operand_value_f64() {
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-43.0), false);
        assert!(OperandValue::F64(-42.0) == OperandValue::F64(-42.0));
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(41.0), false);
        assert!(OperandValue::F64(-42.0) != OperandValue::F64(-43.0));
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-42.0), false);
        assert!(OperandValue::F64(-42.0) != OperandValue::F64(41.0));
        assert!(OperandValue::F64(-42.0) > OperandValue::F64(-43.0));
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(41.0), false);
        assert!(OperandValue::F64(-42.0) >= OperandValue::F64(-43.0));
        assert!(OperandValue::F64(-42.0) >= OperandValue::F64(-42.0));
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-42.0), false);
        assert!(OperandValue::F64(-42.0) < OperandValue::F64(41.0));
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-43.0), false);
        assert!(OperandValue::F64(-42.0) <= OperandValue::F64(-42.0));
        assert!(OperandValue::F64(-42.0) <= OperandValue::F64(41.0));
    }

    #[test]
    fn test_operand_value_usize() {
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(41), false);
        assert!(OperandValue::USize(42) == OperandValue::USize(42));
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(43), false);
        assert!(OperandValue::USize(42) != OperandValue::USize(41));
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(42), false);
        assert!(OperandValue::USize(42) != OperandValue::USize(43));
        assert!(OperandValue::USize(42) > OperandValue::USize(41));
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(43), false);
        assert!(OperandValue::USize(42) >= OperandValue::USize(41));
        assert!(OperandValue::USize(42) >= OperandValue::USize(42));
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(42), false);
        assert!(OperandValue::USize(42) < OperandValue::USize(43));
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(41), false);
        assert!(OperandValue::USize(42) <= OperandValue::USize(42));
        assert!(OperandValue::USize(42) <= OperandValue::USize(43));
    }

    #[test]
    fn test_operand_value_isize() {
        assert_eq!(OperandValue::ISize(-42) == OperandValue::ISize(-43), false);
        assert!(OperandValue::ISize(-42) == OperandValue::ISize(-42));
        assert_eq!(OperandValue::ISize(-42) == OperandValue::ISize(-41), false);
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-43));
        assert_eq!(OperandValue::ISize(-42) != OperandValue::ISize(-42), false);
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-41));
        assert!(OperandValue::ISize(-42) > OperandValue::ISize(-43));
        assert_eq!(OperandValue::ISize(-42) > OperandValue::ISize(-42), false);
        assert_eq!(OperandValue::ISize(-42) > OperandValue::ISize(-41), false);
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-43));
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-42));
        assert_eq!(OperandValue::ISize(-42) >= OperandValue::ISize(-41), false);
        assert_eq!(OperandValue::ISize(-42) < OperandValue::ISize(-43), false);
        assert_eq!(OperandValue::ISize(-42) < OperandValue::ISize(-42), false);
        assert!(OperandValue::ISize(-42) < OperandValue::ISize(-41));
        assert_eq!(OperandValue::ISize(-42) <= OperandValue::ISize(-43), false);
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-42));
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-41));
    }

    #[test]
    fn test_operand_value_bool() {
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(false), false);
        assert!(OperandValue::Bool(true) == OperandValue::Bool(true));
        assert!(OperandValue::Bool(true) != OperandValue::Bool(false));
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(true), false);
        assert!(OperandValue::Bool(true) > OperandValue::Bool(false));
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(true), false);
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(false));
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(true));
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(false), false);
        assert!(OperandValue::Bool(true) <= OperandValue::Bool(true));
    }

    #[test]
    fn test_operand_value_string() {
        assert_eq!(OperandValue::from("j") == OperandValue::from("i"), false);
        assert!(OperandValue::from("j") == OperandValue::from("j"));
        assert_eq!(OperandValue::from("j") == OperandValue::from("k"), false);
        assert!(OperandValue::from("j") != OperandValue::from("i"));
        assert_eq!(OperandValue::from("j") != OperandValue::from("j"), false);
        assert!(OperandValue::from("j") != OperandValue::from("k"));
        assert!(OperandValue::from("j") > OperandValue::from("i"));
        assert_eq!(OperandValue::from("j") > OperandValue::from("j"), false);
        assert_eq!(OperandValue::from("j") > OperandValue::from("k"), false);
        assert!(OperandValue::from("j") >= OperandValue::from("i"));
        assert!(OperandValue::from("j") >= OperandValue::from("j"));
        assert_eq!(OperandValue::from("j") >= OperandValue::from("k"), false);
        assert_eq!(OperandValue::from("j") < OperandValue::from("i"), false);
        assert_eq!(OperandValue::from("j") < OperandValue::from("j"), false);
        assert!(OperandValue::from("j") < OperandValue::from("k"));
        assert_eq!(OperandValue::from("j") <= OperandValue::from("i"), false);
        assert!(OperandValue::from("j") <= OperandValue::from("j"));
        assert!(OperandValue::from("j") <= OperandValue::from("k"));
    }

    #[test]
    fn test_operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::from("a b c")), None);
    }

    #[test]
    fn test_compare_u64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::U64(24)), Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(23), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(25), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::I64(24)), Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(23), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(25), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::F64(24.5)), Operand::Value(OperandValue::F64(42.5)));
        assert_eq!(compare(&v, &OperandValue::F64(23.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(24.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(25.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(23), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(25), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::ISize(24)), Operand::Value(OperandValue::ISize(42)));
        assert_eq!(compare(&v, &OperandValue::ISize(23), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(25), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::Bool(false)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::from("f")), Operand::Value(OperandValue::from("j")));
        assert_eq!(compare(&v, &OperandValue::from("e"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("f"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("g"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("i"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("j"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("k"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::U64(42))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::U64(42))), &OperandValue::Bool(false), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::U64(42))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_i64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::I64(42))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::I64(42))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::I64(42))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::I64(42))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::I64(42))), &OperandValue::Bool(false), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::I64(42))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_f64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::F64(42.0))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::F64(42.0))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Bool(false), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_usize_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::USize(42))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::USize(42))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::USize(42))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::USize(42))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::USize(42))), &OperandValue::Bool(false), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::USize(42))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_isize_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::ISize(42))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::ISize(42))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::ISize(42))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::ISize(42))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::ISize(42))), &OperandValue::Bool(false), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::ISize(42))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_bool_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(true))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Bool(true))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Bool(true))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Bool(true))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Bool(true))), &OperandValue::from("abc"), &ROOT), None);
    }

    #[test]
    fn test_compare_string_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::from("abc"))), &OperandValue::U64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::from("abc"))), &OperandValue::I64(41), &ROOT), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::from("abc"))), &OperandValue::F64(41.5), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::from("abc"))), &OperandValue::USize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::from("abc"))), &OperandValue::ISize(41), &ROOT), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::from("abc"))), &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_u64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le_field() {
        let v = Operation::Le(Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("u64.0.value".into()), Operand::FieldPath("u64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::U64(31), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(32), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(33), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le_field() {
        let v = Operation::Le(Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("i64.0.value".into()), Operand::FieldPath("i64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::I64(-43), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(-42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(-41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_le_field() {
        let v = Operation::Le(Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("f64.0.value".into()), Operand::FieldPath("f64.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::F64(-43.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(-42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(-41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_le_field() {
        let v = Operation::Le(Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("usize.0.value".into()), Operand::FieldPath("usize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::USize(31), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(32), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(33), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_isize_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_le_field() {
        let v = Operation::Le(Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_isize_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("isize.0.value".into()), Operand::FieldPath("isize.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::ISize(-43), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::ISize(-42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(-41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(41), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::ISize(43), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("bool.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("bool.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("bool.0.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("bool.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("bool.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_le_field() {
        let v = Operation::Le(Operand::FieldPath("bool.0.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("bool.0.value".into()), Operand::FieldPath("bool.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le_field() {
        let v = Operation::Le(Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("str.0.value".into()), Operand::FieldPath("str.1.value".into()));
        assert_eq!(compare(&v, &OperandValue::from("narutn"), &ROOT), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::from("naruto"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("narutp"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukd"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasuke"), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::from("sasukf"), &ROOT), Some(Err(())));
    }
}
