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
pub struct ObjSchema {
    pub required: bool,
    pub validation: BTreeMap<String, Schema>,
}

impl From<BTreeMap<String, Schema>> for ObjSchema {
    fn from(validation: BTreeMap<String, Schema>) -> Self {
        ObjSchema { required: true, validation }
    }
}

impl<const N: usize> From<[(String, Schema); N]> for ObjSchema {
    fn from(value: [(String, Schema); N]) -> Self {
        ObjSchema { required: true, validation: BTreeMap::from(value) }
    }
}

impl ObjSchema {
    pub fn optional(self) -> Self {
        ObjSchema { required: false, validation: self.validation }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Schema {
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
    Obj(ObjSchema),
    Enum(EnumSchema),
}

impl From<U64Schema> for Schema {
    fn from(validation: U64Schema) -> Self {
        Schema::U64(validation)
    }
}

impl From<I64Schema> for Schema {
    fn from(validation: I64Schema) -> Self {
        Schema::I64(validation)
    }
}

impl From<F64Schema> for Schema {
    fn from(validation: F64Schema) -> Self {
        Schema::F64(validation)
    }
}

impl From<USizeSchema> for Schema {
    fn from(validation: USizeSchema) -> Self {
        Schema::USize(validation)
    }
}

impl From<ISizeSchema> for Schema {
    fn from(validation: ISizeSchema) -> Self {
        Schema::ISize(validation)
    }
}

impl From<BoolSchema> for Schema {
    fn from(validation: BoolSchema) -> Self {
        Schema::Bool(validation)
    }
}

impl From<StrSchema> for Schema {
    fn from(validation: StrSchema) -> Self {
        Schema::Str(validation)
    }
}

impl From<EmailSchema> for Schema {
    fn from(validation: EmailSchema) -> Self {
        Schema::Email(validation)
    }
}

impl From<DateSchema> for Schema {
    fn from(validation: DateSchema) -> Self {
        Schema::Date(validation)
    }
}

impl From<TimeSchema> for Schema {
    fn from(validation: TimeSchema) -> Self {
        Schema::Time(validation)
    }
}

impl From<DateTimeSchema> for Schema {
    fn from(validation: DateTimeSchema) -> Self {
        Schema::DateTime(validation)
    }
}

impl From<ObjSchema> for Schema {
    fn from(validation: ObjSchema) -> Self {
        Schema::Obj(validation)
    }
}

impl From<EnumSchema> for Schema {
    fn from(validation: EnumSchema) -> Self {
        Schema::Enum(validation)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        BoolSchema, DateSchema, DateTimeSchema, EmailSchema, EnumSchema, EnumValues, F64Schema, I64Schema, ISizeSchema, ObjSchema, Schema, StrSchema,
        TimeSchema, U64Schema, USizeSchema,
    };

    #[test]
    fn obj_validation() {
        assert_eq!(
            ObjSchema::from(BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))])),
            ObjSchema { required: true, validation: BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjSchema::from(BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))])).optional(),
            ObjSchema { required: false, validation: BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjSchema::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]),
            ObjSchema { required: true, validation: BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]) }
        );
        assert_eq!(
            ObjSchema::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]).optional(),
            ObjSchema { required: false, validation: BTreeMap::from([("is".into(), Schema::Bool(BoolSchema::default().eq(false)))]) }
        );
    }

    #[test]
    fn validation_from() {
        let enum_usize: Vec<usize> = vec![1, 2, 3];
        assert_eq!(Schema::from(U64Schema::default()), Schema::U64(U64Schema { required: true, operation: None }));
        assert_eq!(Schema::from(I64Schema::default()), Schema::I64(I64Schema { required: true, operation: None }));
        assert_eq!(Schema::from(F64Schema::default()), Schema::F64(F64Schema { required: true, operation: None }));
        assert_eq!(Schema::from(USizeSchema::default()), Schema::USize(USizeSchema { required: true, operation: None }));
        assert_eq!(Schema::from(ISizeSchema::default()), Schema::ISize(ISizeSchema { required: true, operation: None }));
        assert_eq!(Schema::from(BoolSchema::default()), Schema::Bool(BoolSchema { required: true, operation: None }));
        assert_eq!(
            Schema::from(StrSchema::default()),
            Schema::Str(StrSchema {
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
        assert_eq!(Schema::from(EmailSchema::default()), Schema::Email(EmailSchema { required: true }));
        assert_eq!(Schema::from(DateSchema::default()), Schema::Date(DateSchema { required: true, operation: None }));
        assert_eq!(Schema::from(TimeSchema::default()), Schema::Time(TimeSchema { required: true, operation: None }));
        assert_eq!(Schema::from(DateTimeSchema::default()), Schema::DateTime(DateTimeSchema { required: true, operation: None }));
        assert_eq!(Schema::from(ObjSchema::from(BTreeMap::new())), Schema::Obj(ObjSchema { required: true, validation: BTreeMap::new() }));
        assert_eq!(
            Schema::from(EnumSchema::from(enum_usize.clone())),
            Schema::Enum(EnumSchema { required: true, values: EnumValues::USize(enum_usize) })
        );
    }
}
