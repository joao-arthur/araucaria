use std::cmp::Ordering;

use crate::value::Value;

use super::{OperandValue, Operation, resolve_operand_value};

use compare_eq::compare_eq;
use compare_ne::compare_ne;
use compare_gt::compare_gt;
use compare_ge::compare_ge;
use compare_lt::compare_lt;
use compare_le::compare_le;

mod compare_eq;
mod compare_ne;
mod compare_gt;
mod compare_ge;
mod compare_lt;
mod compare_le;

pub fn compare(operation: &Operation, value_a: &OperandValue, root: &Value) -> Option<Result<(), ()>> {
    match operation {
        Operation::Eq(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_eq(value_a, &value_b)
        }
        Operation::Ne(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_ne(value_a, &value_b)
        }
        Operation::Gt(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_gt(value_a, &value_b)
        }
        Operation::Ge(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_ge(value_a, &value_b)
        }
        Operation::Lt(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_lt(value_a, &value_b)
        }
        Operation::Le(operand) => {
            let value_b = resolve_operand_value(operand, root)?;
            compare_le(value_a, &value_b)
        }
        Operation::Btwn(operand_a, operand_b) => {
            let value_operand_a = resolve_operand_value(operand_a, root)?;
            let value_operand_b = resolve_operand_value(operand_b, root)?;
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

    use super::super::{Operand, OperandValue, Operation};

    use super::compare;

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
