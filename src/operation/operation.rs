use crate::value::{path::resolve_path, Value};

use super::{OperandValue, operand_value_from_value};

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Value(OperandValue),
    FieldPath(String),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Operand::Value(value) => value.to_string(),
            Operand::FieldPath(path) => "\"".to_string() + path + "\"",
        };
        write!(f, "{}", str)
    }
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
            operand_value_from_value(&field)
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn resolve_operand_value_value() {
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::U64(42)), &ROOT), Some(OperandValue::U64(42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::I64(-42)), &ROOT), Some(OperandValue::I64(-42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::F64(-42.5)), &ROOT), Some(OperandValue::F64(-42.5)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::USize(42)), &ROOT), Some(OperandValue::USize(42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::ISize(-42)), &ROOT), Some(OperandValue::ISize(-42)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::Bool(false)), &ROOT), Some(OperandValue::Bool(false)));
        assert_eq!(resolve_operand_value(&Operand::Value(OperandValue::Str("Lemouria".into())), &ROOT), Some(OperandValue::Str("Lemouria".into())));
    }

    #[test]
    fn resolve_operand_value_field() {
        assert_eq!(resolve_operand_value(&Operand::FieldPath("u64".into()), &ROOT), Some(OperandValue::U64(42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("i64".into()), &ROOT), Some(OperandValue::I64(-42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("f64".into()), &ROOT), Some(OperandValue::F64(-42.5)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("usize".into()), &ROOT), Some(OperandValue::USize(42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("isize".into()), &ROOT), Some(OperandValue::ISize(-42)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("bool".into()), &ROOT), Some(OperandValue::Bool(false)));
        assert_eq!(resolve_operand_value(&Operand::FieldPath("str".into()), &ROOT), Some(OperandValue::Str("Lemouria".into())));
    }

    #[test]
    fn resolve_operand_value_field_not_found() {
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
        assert_eq!(resolve_operand_value(&Operand::FieldPath("field.value.some.foo.bar".into()), &ROOT), None);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Operand::Value(OperandValue::U64(4)).to_string(), "4".to_string());
        assert_eq!(Operand::FieldPath("user.info.details.name".into()).to_string(), r#""user.info.details.name""#.to_string());
    }
}
