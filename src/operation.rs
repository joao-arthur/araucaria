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

pub fn compare(operation: &Operation, value_a: &OperandValue, root: &Value) -> Option<Result<(), ()>> {
    match operation {
        Operation::Eq(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_eq(&value_a, &value_b)
        }
        Operation::Ne(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_ne(&value_a, &value_b)
        }
        Operation::Gt(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_gt(&value_a, &value_b)
        }
        Operation::Ge(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_ge(&value_a, &value_b)
        }
        Operation::Lt(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_lt(&value_a, &value_b)
        }
        Operation::Le(operand) => {
            let value_b = match operand {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            compare_le(&value_a, &value_b)
        }
        Operation::Btwn(operand_a, operand_b) => {
            let value_operand_a = match operand_a {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            let value_operand_b = match operand_b {
                Operand::Value(value) => Some(value.clone()),
                Operand::FieldPath(field_path) => Some(value_to_operand_value(&resolve_path(root, field_path)?)?),
            }?;
            if let Some(Ok(())) = compare_ge(&value_a, &value_operand_a) {
                if let Some(Ok(())) = compare_le(&value_a, &value_operand_b) {
                    Some(Ok(()))
                } else {
                    Some(Err(()))
                }
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

    use super::{compare, Operand, OperandValue, Operation};

    #[test]
    fn test_operand_value_u64() {
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(43), true);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(43), true);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(43), true);
    }

    #[test]
    fn test_operand_value_i64() {
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-41), true);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-41), true);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-41), true);
    }

    #[test]
    fn test_operand_value_f64() {
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(41.0), true);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(41.0), true);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(41.0), true);
    }

    #[test]
    fn test_operand_value_usize() {
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(43), true);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(43), true);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(43), true);
    }

    #[test]
    fn test_operand_value_bool() {
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(true), true);
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) >= OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) >= OperandValue::Bool(true), true);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(true), true);
    }

    #[test]
    fn test_operand_value_string() {
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("k")), true);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("k")), true);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("k")), true);
    }

    #[test]
    fn test_operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
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
        let v = Operation::Eq(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne_value() {
        let v = Operation::Ne(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt_value() {
        let v = Operation::Gt(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge_value() {
        let v = Operation::Ge(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt_value() {
        let v = Operation::Lt(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le_value() {
        let v = Operation::Le(Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn_value() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Str(String::from("f"))), Operand::Value(OperandValue::Str(String::from("j"))));
        let root = Value::None;
        assert_eq!(compare(&v, &OperandValue::Str(String::from("e")), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("f")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("g")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j")), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k")), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::U64(42))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::U64(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::U64(42))), &OperandValue::Str(String::from("abc")), &root), None);
    }

    #[test]
    fn test_compare_i64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::I64(42))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::I64(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::I64(42))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::I64(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::I64(42))), &OperandValue::Str(String::from("abc")), &root), None);
    }

    #[test]
    fn test_compare_f64_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::F64(42.0))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::F64(42.0))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Str(String::from("abc")), &root), None);
    }

    #[test]
    fn test_compare_usize_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::USize(42))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::USize(42))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::USize(42))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::USize(42))), &OperandValue::Bool(false), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::USize(42))), &OperandValue::Str(String::from("abc")), &root), None);
    }

    #[test]
    fn test_compare_bool_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(true))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Bool(true))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Bool(true))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Bool(true))), &OperandValue::Str(String::from("abc")), &root), None);
    }

    #[test]
    fn test_compare_string_other_types() {
        let root = Value::None;
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::U64(41), &root), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::I64(41), &root), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::F64(41.5), &root), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::USize(41), &root), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::Bool(false), &root), None);
    }

    #[test]
    fn test_compare_u64_eq_field() {
        let v = Operation::Eq(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le_field() {
        let v = Operation::Le(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::U64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath(String::from("values.1.value")), Operand::FieldPath(String::from("values.2.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::U64(42))])),
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
        let v = Operation::Eq(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne_field() {
        let v = Operation::Ne(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt_field() {
        let v = Operation::Gt(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge_field() {
        let v = Operation::Ge(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt_field() {
        let v = Operation::Lt(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le_field() {
        let v = Operation::Le(Operand::FieldPath(String::from("values.3.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(41), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43), &root), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn_field() {
        let v = Operation::Btwn(Operand::FieldPath(String::from("values.1.value")), Operand::FieldPath(String::from("values.2.value")));
        let root = Value::Obj(BTreeMap::from([(
            String::from("values"),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(12))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(22))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(32))])),
                Value::Obj(BTreeMap::from([(String::from("value"), Value::I64(42))])),
            ]),
        )]));
        assert_eq!(compare(&v, &OperandValue::I64(21), &root), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(22), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(23), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(31), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(32), &root), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(33), &root), Some(Err(())));
    }
}
