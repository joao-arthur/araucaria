use std::collections::BTreeMap;

use crate::{operation::Operation, schema::EnumValues};

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationErr {
    Required,
    U64,
    I64,
    F64,
    USize,
    ISize,
    Bool,
    Str,
    Email,
    Date,
    Time,
    DateTime,
    Operation(Operation),
    BytesLen(Operation),
    CharsLen(Operation),
    GraphemesLen(Operation),
    LowercaseLen(Operation),
    UppercaseLen(Operation),
    NumbersLen(Operation),
    SymbolsLen(Operation),
    Enumerated(EnumValues),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Schema(Vec<ValidationErr>),
    Arr(Vec<SchemaErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl SchemaErr {
    pub fn validation<const N: usize>(value: [ValidationErr; N]) -> SchemaErr {
        SchemaErr::Schema(value.to_vec())
    }

    pub fn arr<const N: usize>(value: [SchemaErr; N]) -> SchemaErr {
        SchemaErr::Arr(value.to_vec())
    }

    pub fn obj<const N: usize>(value: [(String, SchemaErr); N]) -> SchemaErr {
        SchemaErr::Obj(BTreeMap::from(value))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::operation::{Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr};

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;

    const OPERATION_U64: ValidationErr = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(37))));
    const OPERATION_I64: ValidationErr = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-17))));
    const OPERATION_F64: ValidationErr = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-38.5))));

    #[test]
    fn schema_err_validation() {
        assert_eq!(SchemaErr::validation([REQUIRED, U64, OPERATION_U64]), SchemaErr::Schema(vec![REQUIRED, U64, OPERATION_U64]));
    }

    #[test]
    fn schema_err_arr() {
        assert_eq!(
            SchemaErr::arr([SchemaErr::validation([REQUIRED, I64, OPERATION_I64])]),
            SchemaErr::Arr(vec![SchemaErr::Schema(vec![REQUIRED, I64, OPERATION_I64])])
        );
    }

    #[test]
    fn schema_err_obj() {
        assert_eq!(
            SchemaErr::obj([("f64".into(), SchemaErr::validation([REQUIRED, F64, OPERATION_F64]))]),
            SchemaErr::Obj(BTreeMap::from([("f64".into(), SchemaErr::Schema(vec![REQUIRED, F64, OPERATION_F64]))]))
        );
    }

    #[test]
    fn schema_err_nested() {
        assert_eq!(
            SchemaErr::obj([(
                "user".into(),
                SchemaErr::arr([
                    SchemaErr::validation([REQUIRED]),
                    SchemaErr::obj([("i64".into(), SchemaErr::validation([REQUIRED, I64, OPERATION_I64]))]),
                ]),
            )]),
            SchemaErr::Obj(BTreeMap::from([(
                "user".into(),
                SchemaErr::Arr(vec![
                    SchemaErr::Schema(vec![REQUIRED]),
                    SchemaErr::Obj(BTreeMap::from([("i64".into(), SchemaErr::Schema(vec![REQUIRED, I64, OPERATION_I64]))])),
                ]),
            )]))
        );
    }
}
