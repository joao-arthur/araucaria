use crate::{path::resolve_path, value::Value};

use super::{OperandValue, value_to_operand_value};

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

pub fn resolve_operand_value(operation: &Operand, root: &Value) -> Option<OperandValue> {
    match operation {
        Operand::Value(value) => Some(value.clone()),
        Operand::FieldPath(field_path) => {
            let field = resolve_path(root, field_path)?;
            value_to_operand_value(&field)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, sync::LazyLock};

    use crate::value::Value;

    use super::super::OperandValue;

    use super::{Operand, resolve_operand_value};

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([
            ("u64".into(), Value::U64(42)),
            ("i64".into(), Value::I64(-42)),
            ("f64".into(), Value::F64(-42.5)),
            ("usize".into(), Value::USize(42)),
            ("isize".into(), Value::ISize(-42)),
            ("bool".into(), Value::Bool(false)),
            ("str".into(), Value::from("Lemouria")),
        ]))
    });

    #[test]
    fn test_resolve_operand_value_value() {
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::U64(42)), &ROOT), Some(OperandValue::U64(42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::I64(-42)), &ROOT), Some(OperandValue::I64(-42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::F64(-42.5)), &ROOT), Some(OperandValue::F64(-42.5)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::USize(42)), &ROOT), Some(OperandValue::USize(42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::ISize(-42)), &ROOT), Some(OperandValue::ISize(-42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::Bool(false)), &ROOT), Some(OperandValue::Bool(false)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::Str("Lemouria".into())), &ROOT), Some(OperandValue::Str("Lemouria".into())));
    }

    #[test]
    fn test_resolve_operand_value_field() {
        assert_eq!(resolve_operand_value(&Operand::FieldPath("u64".into()), &ROOT), Some(OperandValue::U64(42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("i64".into()), &ROOT), Some(OperandValue::I64(-42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("f64".into()), &ROOT), Some(OperandValue::F64(-42.5)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("usize".into()), &ROOT), Some(OperandValue::USize(42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("isize".into()), &ROOT), Some(OperandValue::ISize(-42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("bool".into()), &ROOT), Some(OperandValue::Bool(false)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("str".into()), &ROOT), Some(OperandValue::Str("Lemouria".into())));
    }
}
