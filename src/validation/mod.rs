use std::collections::BTreeMap;

pub use validation_bool::BoolValidation;
pub use validation_date::DateValidation;
pub use validation_date_time::DateTimeValidation;
pub use validation_email::EmailValidation;
pub use validation_enumerated::EnumValidation;
pub use validation_enumerated::EnumValues;
pub use validation_f64::F64Validation;
pub use validation_i64::I64Validation;
pub use validation_isize::ISizeValidation;
pub use validation_str::StrValidation;
pub use validation_time::TimeValidation;
pub use validation_u64::U64Validation;
pub use validation_usize::USizeValidation;

mod validation_bool;
mod validation_date;
mod validation_date_time;
mod validation_email;
mod validation_enumerated;
mod validation_f64;
mod validation_i64;
mod validation_isize;
mod validation_str;
mod validation_time;
mod validation_u64;
mod validation_usize;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub required: bool,
    pub validation: BTreeMap<String, Validation>,
}

impl Default for ObjValidation {
    fn default() -> Self {
        ObjValidation { required: true, validation: BTreeMap::new() }
    }
}

impl ObjValidation {
    pub fn optional(self) -> Self {
        ObjValidation { required: false, validation: self.validation }
    }

    pub fn validation(self, validation: BTreeMap<String, Validation>) -> Self {
        ObjValidation { required: self.required, validation }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    U64(U64Validation),
    I64(I64Validation),
    F64(F64Validation),
    USize(USizeValidation),
    ISize(ISizeValidation),
    Bool(BoolValidation),
    Str(StrValidation),
    Email(EmailValidation),
    Date(DateValidation),
    Time(TimeValidation),
    DateTime(DateTimeValidation),
    Obj(ObjValidation),
    Enum(EnumValidation),
}

impl From<U64Validation> for Validation {
    fn from(validation: U64Validation) -> Self {
        Validation::U64(validation)
    }
}

impl From<I64Validation> for Validation {
    fn from(validation: I64Validation) -> Self {
        Validation::I64(validation)
    }
}

impl From<F64Validation> for Validation {
    fn from(validation: F64Validation) -> Self {
        Validation::F64(validation)
    }
}

impl From<USizeValidation> for Validation {
    fn from(validation: USizeValidation) -> Self {
        Validation::USize(validation)
    }
}

impl From<ISizeValidation> for Validation {
    fn from(validation: ISizeValidation) -> Self {
        Validation::ISize(validation)
    }
}

impl From<BoolValidation> for Validation {
    fn from(validation: BoolValidation) -> Self {
        Validation::Bool(validation)
    }
}

impl From<StrValidation> for Validation {
    fn from(validation: StrValidation) -> Self {
        Validation::Str(validation)
    }
}

impl From<EmailValidation> for Validation {
    fn from(validation: EmailValidation) -> Self {
        Validation::Email(validation)
    }
}

impl From<DateValidation> for Validation {
    fn from(validation: DateValidation) -> Self {
        Validation::Date(validation)
    }
}

impl From<TimeValidation> for Validation {
    fn from(validation: TimeValidation) -> Self {
        Validation::Time(validation)
    }
}

impl From<DateTimeValidation> for Validation {
    fn from(validation: DateTimeValidation) -> Self {
        Validation::DateTime(validation)
    }
}

impl From<ObjValidation> for Validation {
    fn from(validation: ObjValidation) -> Self {
        Validation::Obj(validation)
    }
}

impl From<EnumValidation> for Validation {
    fn from(validation: EnumValidation) -> Self {
        Validation::Enum(validation)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::{
        BoolValidation, DateTimeValidation, DateValidation, EmailValidation, EnumValidation, EnumValues, F64Validation, I64Validation,
        ISizeValidation, ObjValidation, StrValidation, TimeValidation, U64Validation, USizeValidation, Validation,
    };

    #[test]
    fn test_obj_validation() {
        assert_eq!(ObjValidation::default(), ObjValidation { required: true, validation: BTreeMap::new() });
        assert_eq!(ObjValidation::default().optional(), ObjValidation { required: false, validation: BTreeMap::new() });
        assert_eq!(
            ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))])),
            ObjValidation { required: true, validation: BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))]) }
        );
    }

    #[test]
    fn test_validation_from() {
        assert_eq!(Validation::from(U64Validation::default()), Validation::U64(U64Validation { required: true, operation: None }));
        assert_eq!(Validation::from(I64Validation::default()), Validation::I64(I64Validation { required: true, operation: None }));
        assert_eq!(Validation::from(F64Validation::default()), Validation::F64(F64Validation { required: true, operation: None }));
        assert_eq!(Validation::from(USizeValidation::default()), Validation::USize(USizeValidation { required: true, operation: None }));
        assert_eq!(Validation::from(ISizeValidation::default()), Validation::ISize(ISizeValidation { required: true, operation: None }));
        assert_eq!(Validation::from(BoolValidation::default()), Validation::Bool(BoolValidation { required: true, operation: None }));
        assert_eq!(
            Validation::from(StrValidation::default()),
            Validation::Str(StrValidation {
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
        assert_eq!(Validation::from(EmailValidation::default()), Validation::Email(EmailValidation { required: true }));
        assert_eq!(Validation::from(DateValidation::default()), Validation::Date(DateValidation { required: true, operation: None }));
        assert_eq!(Validation::from(TimeValidation::default()), Validation::Time(TimeValidation { required: true, operation: None }));
        assert_eq!(Validation::from(DateTimeValidation::default()), Validation::DateTime(DateTimeValidation { required: true, operation: None }));
        assert_eq!(Validation::from(ObjValidation::default()), Validation::Obj(ObjValidation { required: true, validation: BTreeMap::new() }));
        assert_eq!(
            Validation::from(EnumValidation::from(vec![1_usize, 2_usize, 3_usize])),
            Validation::Enum(EnumValidation { required: true, values: EnumValues::USize(vec![1_usize, 2_usize, 3_usize]) })
        );
    }
}
