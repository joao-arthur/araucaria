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
    pub validation: HashMap<&'static str, Validation>,
}

impl Default for ObjValidation {
    fn default() -> Self {
        ObjValidation { required: true, validation: HashMap::new(),   }
    }
}

impl ObjValidation {
    pub fn optional(self) -> Self {
        ObjValidation { required: false, validation: self.validation,  }
    }

    pub fn validation(self, validation: HashMap<&'static str, Validation>) -> Self {
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
