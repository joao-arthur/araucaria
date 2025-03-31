use std::collections::HashMap;

use bool::BoolValidation;
use date::DateValidation;
use datetime::DateTimeValidation;
use email::EmailValidation;
use num_f::NumFValidation;
use num_i::NumIValidation;
use num_u::NumUValidation;
use str::StrValidation;
use time::TimeValidation;

pub mod bool;
pub mod date;
pub mod datetime;
pub mod email;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;
pub mod time;

#[derive(Debug, PartialEq, Clone)]
pub struct ObjValidation {
    pub required: bool,
    pub validation: HashMap<String, Validation>,
}

impl Default for ObjValidation {
    fn default() -> Self {
        ObjValidation { required: true, validation: HashMap::new() }
    }
}

impl ObjValidation {
    pub fn optional(self) -> Self {
        ObjValidation { required: false, validation: self.validation }
    }

    pub fn validation(self, validation: HashMap<String, Validation>) -> Self {
        ObjValidation { required: self.required, validation }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Validation {
    NumU(NumUValidation),
    NumI(NumIValidation),
    NumF(NumFValidation),
    Bool(BoolValidation),
    Str(StrValidation),
    Email(EmailValidation),
    Date(DateValidation),
    Time(TimeValidation),
    DateTime(DateTimeValidation),
    Obj(ObjValidation),
}

impl From<NumUValidation> for Validation {
    fn from(validation: NumUValidation) -> Self {
        Validation::NumU(validation)
    }
}

impl From<NumIValidation> for Validation {
    fn from(validation: NumIValidation) -> Self {
        Validation::NumI(validation)
    }
}

impl From<NumFValidation> for Validation {
    fn from(validation: NumFValidation) -> Self {
        Validation::NumF(validation)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obj_validation() {
        assert_eq!(ObjValidation::default(), ObjValidation { required: true, validation: HashMap::new() });
        assert_eq!(ObjValidation::default().optional(), ObjValidation { required: false, validation: HashMap::new() });
        assert_eq!(
            ObjValidation::default().validation(HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))])),
            ObjValidation {
                required: true,
                validation: HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))])
            }
        );
    }

    #[test]
    fn test_validation_from() {
        assert_eq!(
            Validation::from(NumUValidation::default()),
            Validation::NumU(NumUValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(
            Validation::from(NumIValidation::default()),
            Validation::NumI(NumIValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(
            Validation::from(NumFValidation::default()),
            Validation::NumF(NumFValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(Validation::from(BoolValidation::default()), Validation::Bool(BoolValidation { required: true, eq: None, ne: None }));
        assert_eq!(
            Validation::from(StrValidation::default()),
            Validation::Str(StrValidation {
                required: true,
                eq: None,
                ne: None,
                min_bytes_len: None,
                max_bytes_len: None,
                min_graphemes_len: None,
                max_graphemes_len: None,
                min_lowercase_len: None,
                max_lowercase_len: None,
                min_uppercase_len: None,
                max_uppercase_len: None,
                min_number_len: None,
                max_number_len: None,
                min_symbols_len: None,
                max_symbols_len: None,
            })
        );
        assert_eq!(Validation::from(EmailValidation::default()), Validation::Email(EmailValidation { required: true }));
        assert_eq!(
            Validation::from(DateValidation::default()),
            Validation::Date(DateValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(
            Validation::from(TimeValidation::default()),
            Validation::Time(TimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(
            Validation::from(DateTimeValidation::default()),
            Validation::DateTime(DateTimeValidation { required: true, eq: None, ne: None, gt: None, lt: None, ge: None, le: None })
        );
        assert_eq!(Validation::from(ObjValidation::default()), Validation::Obj(ObjValidation { required: true, validation: HashMap::new() }));
    }
}
