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

pub fn resolve_operand(root: &Value, operation: &Operand) -> Option<OperandValue> {
    match operation {
        Operand::Value(value) => Some(value.clone()),
        Operand::FieldPath(field_path) => {
            let field = resolve_path(root, field_path)?;
            value_to_operand_value(&field)
        }
    }
}
