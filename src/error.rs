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
    Validation(Vec<ValidationErr>),
    Arr(Vec<SchemaErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl<const N: usize> From<[ValidationErr; N]> for SchemaErr {
    fn from(value: [ValidationErr; N]) -> Self {
        SchemaErr::Validation(value.to_vec())
    }
}

impl<const N: usize> From<[SchemaErr; N]> for SchemaErr {
    fn from(value: [SchemaErr; N]) -> Self {
        SchemaErr::Arr(value.to_vec())
    }
}

impl<const N: usize> From<[(String, SchemaErr); N]> for SchemaErr {
    fn from(value: [(String, SchemaErr); N]) -> Self {
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
        assert_eq!(SchemaErr::from([REQUIRED, U64, OPERATION_U64]), SchemaErr::Validation(vec![REQUIRED, U64, OPERATION_U64]));
    }

    #[test]
    fn schema_err_arr() {
        assert_eq!(
            SchemaErr::from([SchemaErr::from([REQUIRED, I64, OPERATION_I64])]),
            SchemaErr::Arr(vec![SchemaErr::Validation(vec![REQUIRED, I64, OPERATION_I64])])
        );
    }

    #[test]
    fn schema_err_obj() {
        assert_eq!(
            SchemaErr::from([("f64".into(), SchemaErr::from([REQUIRED, F64, OPERATION_F64]))]),
            SchemaErr::Obj(BTreeMap::from([("f64".into(), SchemaErr::Validation(vec![REQUIRED, F64, OPERATION_F64]))]))
        );
    }

    #[test]
    fn schema_err_nested() {
        assert_eq!(
            SchemaErr::from([(
                "user".into(),
                SchemaErr::from([SchemaErr::from([REQUIRED]), SchemaErr::from([("i64".into(), SchemaErr::from([REQUIRED, I64, OPERATION_I64]))]),]),
            )]),
            SchemaErr::Obj(BTreeMap::from([(
                "user".into(),
                SchemaErr::Arr(vec![
                    SchemaErr::Validation(vec![REQUIRED]),
                    SchemaErr::Obj(BTreeMap::from([("i64".into(), SchemaErr::Validation(vec![REQUIRED, I64, OPERATION_I64]))])),
                ]),
            )]))
        );
    }
}
