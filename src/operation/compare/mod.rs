use crate::value::Value;

use super::{OperandValue, Operation, resolve_operand_value};

use compare_btwn::compare_btwn;
use compare_eq::compare_eq;
use compare_ge::compare_ge;
use compare_gt::compare_gt;
use compare_le::compare_le;
use compare_lt::compare_lt;
use compare_ne::compare_ne;

mod compare_btwn;
mod compare_eq;
mod compare_ge;
mod compare_gt;
mod compare_le;
mod compare_lt;
mod compare_ne;

pub fn compare(operation: &Operation, value: &OperandValue, root: &Value) -> Option<Result<(), ()>> {
    match operation {
        Operation::Eq(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_eq(value, &operand_value)
        }
        Operation::Ne(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_ne(value, &operand_value)
        }
        Operation::Gt(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_gt(value, &operand_value)
        }
        Operation::Ge(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_ge(value, &operand_value)
        }
        Operation::Lt(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_lt(value, &operand_value)
        }
        Operation::Le(operand) => {
            let operand_value = resolve_operand_value(operand, root)?;
            compare_le(value, &operand_value)
        }
        Operation::Btwn(operand_a, operand_b) => {
            let operand_value_a = resolve_operand_value(operand_a, root)?;
            let operand_value_b = resolve_operand_value(operand_b, root)?;
            compare_btwn(value, &operand_value_a, &operand_value_b)
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
            ("u64".into(), Value::U64(42)),
            ("i64".into(), Value::I64(-42)),
            ("f64".into(), Value::F64(-42.5)),
            ("usize".into(), Value::USize(42)),
            ("isize".into(), Value::ISize(-42)),
            ("bool".into(), Value::Bool(false)),
            ("str".into(), Value::from("Lemouria")),
            ("u64_btwn".into(), Value::from([Value::U64(22), Value::U64(24)])),
        ]))
    });

    #[test]
    fn test_compare_eq() {
        let v_value = Operation::Eq(Operand::Value(OperandValue::U64(42)));
        let v_field = Operation::Eq(Operand::FieldPath("u64".into()));
        let v_field_not_found = Operation::Eq(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::U64(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::U64(42), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::U64(42), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_ne() {
        let v_value = Operation::Ne(Operand::Value(OperandValue::I64(-42)));
        let v_field = Operation::Ne(Operand::FieldPath("i64".into()));
        let v_field_not_found = Operation::Ne(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::I64(-42), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::I64(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::I64(24), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::I64(24), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_gt() {
        let v_value = Operation::Gt(Operand::Value(OperandValue::F64(-42.5)));
        let v_field = Operation::Gt(Operand::FieldPath("f64".into()));
        let v_field_not_found = Operation::Gt(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::F64(-43.5), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::F64(-41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::F64(-41.5), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::F64(-41.5), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_ge() {
        let v_value = Operation::Ge(Operand::Value(OperandValue::USize(42)));
        let v_field = Operation::Ge(Operand::FieldPath("usize".into()));
        let v_field_not_found = Operation::Ge(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::USize(41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::USize(43), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::USize(43), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::USize(43), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_lt() {
        let v_value = Operation::Lt(Operand::Value(OperandValue::ISize(-42)));
        let v_field = Operation::Lt(Operand::FieldPath("isize".into()));
        let v_field_not_found = Operation::Lt(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::ISize(-41), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::ISize(-43), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::ISize(-43), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::ISize(-43), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_le() {
        let v_value = Operation::Le(Operand::Value(OperandValue::Str("Lemouria".into())));
        let v_field = Operation::Le(Operand::FieldPath("str".into()));
        let v_field_not_found = Operation::Le(Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::Str("mu".into()), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::Str("Atlantis".into()), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::Str("Atlantis".into()), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found, &OperandValue::Str("Atlantis".into()), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }

    #[test]
    fn test_compare_btwn() {
        let v_value = Operation::Btwn(Operand::Value(OperandValue::U64(22)), Operand::Value(OperandValue::U64(24)));
        let v_field = Operation::Btwn(Operand::FieldPath("u64_btwn.0".into()), Operand::FieldPath("u64_btwn.1".into()));
        let v_field_not_found_a = Operation::Btwn(Operand::FieldPath("fa.fe.fi.fo.fu".into()), Operand::FieldPath("u64_btwn.1".into()));
        let v_field_not_found_b = Operation::Btwn(Operand::FieldPath("u64_btwn.0".into()), Operand::FieldPath("fa.fe.fi.fo.fu".into()));
        assert_eq!(compare(&v_value, &OperandValue::U64(21), &ROOT), Some(Err(())));
        assert_eq!(compare(&v_value, &OperandValue::U64(23), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field, &OperandValue::U64(23), &ROOT), Some(Ok(())));
        assert_eq!(compare(&v_field_not_found_a, &OperandValue::U64(23), &ROOT), None);
        assert_eq!(compare(&v_field_not_found_b, &OperandValue::U64(23), &ROOT), None);
        assert_eq!(compare(&v_value, &OperandValue::Bool(false), &ROOT), None);
    }
}
