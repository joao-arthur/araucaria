use std::collections::BTreeMap;

pub use bool_schema::BoolSchema;
pub use date_schema::DateSchema;
pub use date_time_schema::DateTimeSchema;
pub use email_schema::EmailSchema;
pub use enum_schema::{EnumSchema, EnumValues};
pub use f64_schema::F64Schema;
pub use i64_schema::I64Schema;
pub use isize_schema::ISizeSchema;
pub use str_schema::StrSchema;
pub use time_schema::TimeSchema;
pub use u64_schema::U64Schema;
pub use usize_schema::USizeSchema;

mod bool_schema;
mod date_schema;
mod date_time_schema;
mod email_schema;
mod enum_schema;
mod f64_schema;
mod i64_schema;
mod isize_schema;
mod str_schema;
mod time_schema;
mod u64_schema;
mod usize_schema;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub required: bool,
    pub validation: BTreeMap<String, Validation>,
}

impl From<BTreeMap<String, Validation>> for ObjValidation {
    fn from(validation: BTreeMap<String, Validation>) -> Self {
        ObjValidation { required: true, validation }
    }
}

impl<const N: usize> From<[(String, Validation); N]> for ObjValidation {
    fn from(value: [(String, Validation); N]) -> Self {
        ObjValidation { required: true, validation: BTreeMap::from(value) }
    }
}

impl ObjValidation {
    pub fn optional(self) -> Self {
        ObjValidation { required: false, validation: self.validation }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    U64(U64Schema),
    I64(I64Schema),
    F64(F64Schema),
    USize(USizeSchema),
    ISize(ISizeSchema),
    Bool(BoolSchema),
    Str(StrSchema),
    Email(EmailSchema),
    Date(DateSchema),
    Time(TimeSchema),
    DateTime(DateTimeSchema),
    Obj(ObjValidation),
    Enum(EnumSchema),
}

impl From<U64Schema> for Validation {
    fn from(validation: U64Schema) -> Self {
        Validation::U64(validation)
    }
}

impl From<I64Schema> for Validation {
    fn from(validation: I64Schema) -> Self {
        Validation::I64(validation)
    }
}

impl From<F64Schema> for Validation {
    fn from(validation: F64Schema) -> Self {
        Validation::F64(validation)
    }
}

impl From<USizeSchema> for Validation {
    fn from(validation: USizeSchema) -> Self {
        Validation::USize(validation)
    }
}

impl From<ISizeSchema> for Validation {
    fn from(validation: ISizeSchema) -> Self {
        Validation::ISize(validation)
    }
}

impl From<BoolSchema> for Validation {
    fn from(validation: BoolSchema) -> Self {
        Validation::Bool(validation)
    }
}

impl From<StrSchema> for Validation {
    fn from(validation: StrSchema) -> Self {
        Validation::Str(validation)
    }
}

impl From<EmailSchema> for Validation {
    fn from(validation: EmailSchema) -> Self {
        Validation::Email(validation)
    }
}

impl From<DateSchema> for Validation {
    fn from(validation: DateSchema) -> Self {
        Validation::Date(validation)
    }
}

impl From<TimeSchema> for Validation {
    fn from(validation: TimeSchema) -> Self {
        Validation::Time(validation)
    }
}

impl From<DateTimeSchema> for Validation {
    fn from(validation: DateTimeSchema) -> Self {
        Validation::DateTime(validation)
    }
}

impl From<ObjValidation> for Validation {
    fn from(validation: ObjValidation) -> Self {
        Validation::Obj(validation)
    }
}

impl From<EnumSchema> for Validation {
    fn from(validation: EnumSchema) -> Self {
        Validation::Enum(validation)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        BoolSchema, DateTimeSchema, DateSchema, EmailSchema, EnumSchema, EnumValues, F64Schema, I64Schema,
        ISizeSchema, ObjValidation, StrSchema, TimeSchema, U64Schema, USizeSchema, Validation,
    };

    #[test]
    fn obj_validation() {
        assert_eq!(
            ObjValidation::from(BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))])),
            ObjValidation { required: true, validation: BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjValidation::from(BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))])).optional(),
            ObjValidation { required: false, validation: BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjValidation::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]),
            ObjValidation { required: true, validation: BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjValidation::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]).optional(),
            ObjValidation { required: false, validation: BTreeMap::from([("is".into(), Validation::Bool(BoolSchema::default().eq(false)))]) }
        );
    }

    #[test]
    fn validation_from() {
        let enum_usize: Vec<usize> = vec![1, 2, 3];
        assert_eq!(Validation::from(U64Schema::default()), Validation::U64(U64Schema { required: true, operation: None }));
        assert_eq!(Validation::from(I64Schema::default()), Validation::I64(I64Schema { required: true, operation: None }));
        assert_eq!(Validation::from(F64Schema::default()), Validation::F64(F64Schema { required: true, operation: None }));
        assert_eq!(Validation::from(USizeSchema::default()), Validation::USize(USizeSchema { required: true, operation: None }));
        assert_eq!(Validation::from(ISizeSchema::default()), Validation::ISize(ISizeSchema { required: true, operation: None }));
        assert_eq!(Validation::from(BoolSchema::default()), Validation::Bool(BoolSchema { required: true, operation: None }));
        assert_eq!(
            Validation::from(StrSchema::default()),
            Validation::Str(StrSchema {
                required: true,
                operation: None,
                bytes_len: None,
                chars_len: None,
                graphemes_len: None,
                lowercase_len: None,
                uppercase_len: None,
                numbers_len: None,
                symbols_len: None,
            })
        );
        assert_eq!(Validation::from(EmailSchema::default()), Validation::Email(EmailSchema { required: true }));
        assert_eq!(Validation::from(DateSchema::default()), Validation::Date(DateSchema { required: true, operation: None }));
        assert_eq!(Validation::from(TimeSchema::default()), Validation::Time(TimeSchema { required: true, operation: None }));
        assert_eq!(Validation::from(DateTimeSchema::default()), Validation::DateTime(DateTimeSchema { required: true, operation: None }));
        assert_eq!(
            Validation::from(ObjValidation::from(BTreeMap::new())),
            Validation::Obj(ObjValidation { required: true, validation: BTreeMap::new() })
        );
        assert_eq!(
            Validation::from(EnumSchema::from(enum_usize.clone())),
            Validation::Enum(EnumSchema { required: true, values: EnumValues::USize(enum_usize) })
        );
    }
}
