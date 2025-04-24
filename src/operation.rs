use std::cmp::Ordering;

use crate::{path::resolve_path, value::Value};

#[derive(Debug, PartialEq, Clone)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    Bool(bool),
    Str(String),
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
    use std::collections::BTreeMap;

    use crate::value::Value;

    use super::{Operand, OperandValue, Operation, compare};

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
        assert_eq!(OperandValue::Str("j".into()) == OperandValue::Str("i".into()), false);
        assert!(OperandValue::Str("j".into()) == OperandValue::Str("j".into()));
        assert_eq!(OperandValue::Str("j".into()) == OperandValue::Str("k".into()), false);
        assert!(OperandValue::Str("j".into()) != OperandValue::Str("i".into()));
        assert_eq!(OperandValue::Str("j".into()) != OperandValue::Str("j".into()), false);
        assert!(OperandValue::Str("j".into()) != OperandValue::Str("k".into()));
        assert!(OperandValue::Str("j".into()) > OperandValue::Str("i".into()));
        assert_eq!(OperandValue::Str("j".into()) > OperandValue::Str("j".into()), false);
        assert_eq!(OperandValue::Str("j".into()) > OperandValue::Str("k".into()), false);
        assert!(OperandValue::Str("j".into()) >= OperandValue::Str("i".into()));
        assert!(OperandValue::Str("j".into()) >= OperandValue::Str("j".into()));
        assert_eq!(OperandValue::Str("j".into()) >= OperandValue::Str("k".into()), false);
        assert_eq!(OperandValue::Str("j".into()) < OperandValue::Str("i".into()), false);
        assert_eq!(OperandValue::Str("j".into()) < OperandValue::Str("j".into()), false);
        assert!(OperandValue::Str("j".into()) < OperandValue::Str("k".into()));
        assert_eq!(OperandValue::Str("j".into()) <= OperandValue::Str("i".into()), false);
        assert!(OperandValue::Str("j".into()) <= OperandValue::Str("j".into()));
        assert!(OperandValue::Str("j".into()) <= OperandValue::Str("k".into()));
    }

    #[test]
    fn test_operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Str("a b c".into())), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Str("a b c".into())), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Str("a b c".into())), None);
        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::Str("a b c".into())), None);
    }

    #[test]
    fn test_compare_u64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::U64(24)), Operand::Value(OperandValue::U64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::U64(23), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(24), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(25), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::I64(24)), Operand::Value(OperandValue::I64(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::I64(23), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(24), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(25), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::F64(42.0)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(41.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::F64(24.5)), Operand::Value(OperandValue::F64(42.5)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::F64(23.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(24.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(25.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(42)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::USize(23), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(24), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(25), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::Bool(false)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_eq_value() {
        let v = Operation::Eq(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Str("f".into())), Operand::Value(OperandValue::Str("j".into())));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str("e".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("f".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("g".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::U64(42))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::U64(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::U64(42))), &OperandValue::Str("abc".into()), &root), None);
    }

    #[test]
    fn test_compare_i64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::I64(42))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::I64(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::I64(42))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::I64(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::I64(42))), &OperandValue::Str("abc".into()), &root), None);
    }

    #[test]
    fn test_compare_f64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::F64(42.0))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::F64(42.0))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Str("abc".into()), &root), None);
    }

    #[test]
    fn test_compare_usize_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::USize(42))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::USize(42))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::USize(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::USize(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::USize(42))), &OperandValue::Str("abc".into()), &root), None);
    }

    #[test]
    fn test_compare_bool_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(true))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Bool(true))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Bool(true))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Bool(true))), &OperandValue::Str("abc".into()), &root), None);
    }

    #[test]
    fn test_compare_string_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Str("abc".into()))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Str("abc".into()))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Str("abc".into()))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Str("abc".into()))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Str("abc".into()))), &OperandValue::Bool(false), &root), None);
    }

    #[test]
    fn test_compare_u64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le_field() {
        let v = Operation::Le(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("values.1.value".into()), Operand::FieldPath("values.2.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(21), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(22), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(23), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(31), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(32), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(33), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le_field() {
        let v = Operation::Le(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("values.1.value".into()), Operand::FieldPath("values.2.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(21), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(22), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(23), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(31), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(32), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(33), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_le_field() {
        let v = Operation::Le(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(41.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("values.1.value".into()), Operand::FieldPath("values.2.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::F64(21.5), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(22.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(23.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(31.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(32.5), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(33.5), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("values.2.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_le_field() {
        let v = Operation::Le(Operand::FieldPath("values.2.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("values.0.value".into()), Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(false))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Bool(true))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Bool(false), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_eq_field() {
        let v = Operation::Eq(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne_field() {
        let v = Operation::Ne(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt_field() {
        let v = Operation::Gt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge_field() {
        let v = Operation::Ge(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt_field() {
        let v = Operation::Lt(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le_field() {
        let v = Operation::Le(Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath("values.0.value".into()), Operand::FieldPath("values.3.value".into()));
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("g".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("h".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("i".into()))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::Str("j".into()))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::Str("f".into()), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str("g".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("i".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("j".into()), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str("k".into()), &root), Some(Err(())));
    }
}
