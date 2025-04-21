use std::collections::BTreeMap;

use bool::BoolValidation;
use date::DateValidation;
use date_time::DateTimeValidation;
use email::EmailValidation;
use enumerated::EnumValidation;
use num_f::NumFValidation;
use num_i::NumIValidation;
use num_u::NumUValidation;
use str::StrValidation;
use time::TimeValidation;

pub mod bool;
pub mod date;
pub mod date_time;
pub mod email;
pub mod enumerated;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;
pub mod time;

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
    U64(NumUValidation),
    I64(NumIValidation),
    F64(NumFValidation),
    Bool(BoolValidation),
    Str(StrValidation),
    Email(EmailValidation),
    Date(DateValidation),
    Time(TimeValidation),
    DateTime(DateTimeValidation),
    Obj(ObjValidation),
    Enum(EnumValidation),
}

impl From<NumUValidation> for Validation {
    fn from(validation: NumUValidation) -> Self {
        Validation::U64(validation)
    }
}

impl From<NumIValidation> for Validation {
    fn from(validation: NumIValidation) -> Self {
        Validation::I64(validation)
    }
}

impl From<NumFValidation> for Validation {
    fn from(validation: NumFValidation) -> Self {
        Validation::F64(validation)
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

    use crate::validation::{
        bool::BoolValidation,
        date::DateValidation,
        date_time::DateTimeValidation,
        email::EmailValidation,
        enumerated::{EnumValidation, EnumValues},
        num_f::NumFValidation,
        num_i::NumIValidation,
        num_u::NumUValidation,
        str::StrValidation,
        time::TimeValidation,
    };

    use super::{ObjValidation, Validation};

    #[test]
    fn test_obj_validation() {
        assert_eq!(ObjValidation::default(), ObjValidation { required: true, validation: BTreeMap::new() });
        assert_eq!(ObjValidation::default().optional(), ObjValidation { required: false, validation: BTreeMap::new() });
        assert_eq!(
            ObjValidation::default().validation(BTreeMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))])),
            ObjValidation {
                required: true,
                validation: BTreeMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))])
            }
        );
    }

    #[test]
    fn test_validation_from() {
        assert_eq!(Validation::from(NumUValidation::default()), Validation::U64(NumUValidation { required: true, operation: None }));
        assert_eq!(Validation::from(NumIValidation::default()), Validation::I64(NumIValidation { required: true, operation: None }));
        assert_eq!(Validation::from(NumFValidation::default()), Validation::F64(NumFValidation { required: true, operation: None }));
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
