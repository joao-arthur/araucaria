use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
}

impl From<u64> for OperandValue {
    fn from(value: u64) -> Self {
        OperandValue::U64(value)
    }
}

impl From<i64> for OperandValue {
    fn from(value: i64) -> Self {
        OperandValue::I64(value)
    }
}

impl From<f64> for OperandValue {
    fn from(value: f64) -> Self {
        OperandValue::F64(value)
    }
}

impl From<usize> for OperandValue {
    fn from(value: usize) -> Self {
        OperandValue::USize(value)
    }
}

impl From<isize> for OperandValue {
    fn from(value: isize) -> Self {
        OperandValue::ISize(value)
    }
}

impl From<bool> for OperandValue {
    fn from(value: bool) -> Self {
        OperandValue::Bool(value)
    }
}

impl From<&str> for OperandValue {
    fn from(value: &str) -> Self {
        OperandValue::Str(value.into())
    }
}

impl PartialOrd for OperandValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let OperandValue::U64(a) = self {
            if let OperandValue::U64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::I64(a) = self {
            if let OperandValue::I64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::F64(a) = self {
            if let OperandValue::F64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::USize(a) = self {
            if let OperandValue::USize(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::ISize(a) = self {
            if let OperandValue::ISize(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Bool(a) = self {
            if let OperandValue::Bool(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Str(a) = self {
            if let OperandValue::Str(b) = other {
                return a.partial_cmp(b);
            }
        }
        None
    }
}

pub fn value_to_operand_value(value: &Value) -> Option<OperandValue> {
    match value {
        Value::U64(val) => Some(OperandValue::U64(*val)),
        Value::I64(val) => Some(OperandValue::I64(*val)),
        Value::F64(val) => Some(OperandValue::F64(*val)),
        Value::USize(val) => Some(OperandValue::USize(*val)),
        Value::ISize(val) => Some(OperandValue::ISize(*val)),
        Value::Bool(val) => Some(OperandValue::Bool(*val)),
        Value::Str(val) => Some(OperandValue::Str(val.clone())),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::OperandValue;

    #[test]
    fn test_operand_value_from() {
        let u64_num: u64 = 8;
        let i64_num: i64 = -3;
        let f64_num: f64 = -9.8;
        let usize_num: usize = 183;
        let isize_num: isize = -892;
        assert_eq!(OperandValue::from(8_u64), OperandValue::U64(8));
        assert_eq!(OperandValue::from(-3_i64), OperandValue::I64(-3));
        assert_eq!(OperandValue::from(-9.8), OperandValue::F64(-9.8));
        assert_eq!(OperandValue::from(183_usize), OperandValue::USize(183));
        assert_eq!(OperandValue::from(-892_isize), OperandValue::ISize(-892));
        assert_eq!(OperandValue::from(false), OperandValue::Bool(false));
        assert_eq!(OperandValue::from("in vino veritas"), OperandValue::Str("in vino veritas".into()));
    }

    #[test]
    fn test_operand_value_u64() {
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(41), false);
        assert!(OperandValue::U64(42) == OperandValue::U64(42));
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(43), false);
        assert!(OperandValue::U64(42) != OperandValue::U64(41));
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(42), false);
        assert!(OperandValue::U64(42) != OperandValue::U64(43));
        assert!(OperandValue::U64(42) > OperandValue::U64(41));
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(43), false);
        assert!(OperandValue::U64(42) >= OperandValue::U64(41));
        assert!(OperandValue::U64(42) >= OperandValue::U64(42));
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(42), false);
        assert!(OperandValue::U64(42) < OperandValue::U64(43));
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(41), false);
        assert!(OperandValue::U64(42) <= OperandValue::U64(42));
        assert!(OperandValue::U64(42) <= OperandValue::U64(43));
    }

    #[test]
    fn test_operand_value_i64() {
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-43), false);
        assert!(OperandValue::I64(-42) == OperandValue::I64(-42));
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-41), false);
        assert!(OperandValue::I64(-42) != OperandValue::I64(-43));
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-42), false);
        assert!(OperandValue::I64(-42) != OperandValue::I64(-41));
        assert!(OperandValue::I64(-42) > OperandValue::I64(-43));
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-41), false);
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-43));
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-42));
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-42), false);
        assert!(OperandValue::I64(-42) < OperandValue::I64(-41));
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-43), false);
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-42));
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-41));
    }

    #[test]
    fn test_operand_value_f64() {
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-43.0), false);
        assert!(OperandValue::F64(-42.0) == OperandValue::F64(-42.0));
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(41.0), false);
        assert!(OperandValue::F64(-42.0) != OperandValue::F64(-43.0));
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-42.0), false);
        assert!(OperandValue::F64(-42.0) != OperandValue::F64(41.0));
        assert!(OperandValue::F64(-42.0) > OperandValue::F64(-43.0));
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(41.0), false);
        assert!(OperandValue::F64(-42.0) >= OperandValue::F64(-43.0));
        assert!(OperandValue::F64(-42.0) >= OperandValue::F64(-42.0));
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-42.0), false);
        assert!(OperandValue::F64(-42.0) < OperandValue::F64(41.0));
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-43.0), false);
        assert!(OperandValue::F64(-42.0) <= OperandValue::F64(-42.0));
        assert!(OperandValue::F64(-42.0) <= OperandValue::F64(41.0));
    }

    #[test]
    fn test_operand_value_usize() {
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(41), false);
        assert!(OperandValue::USize(42) == OperandValue::USize(42));
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(43), false);
        assert!(OperandValue::USize(42) != OperandValue::USize(41));
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(42), false);
        assert!(OperandValue::USize(42) != OperandValue::USize(43));
        assert!(OperandValue::USize(42) > OperandValue::USize(41));
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(43), false);
        assert!(OperandValue::USize(42) >= OperandValue::USize(41));
        assert!(OperandValue::USize(42) >= OperandValue::USize(42));
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(42), false);
        assert!(OperandValue::USize(42) < OperandValue::USize(43));
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(41), false);
        assert!(OperandValue::USize(42) <= OperandValue::USize(42));
        assert!(OperandValue::USize(42) <= OperandValue::USize(43));
    }

    #[test]
    fn test_operand_value_isize() {
        assert_eq!(OperandValue::ISize(-42) == OperandValue::ISize(-43), false);
        assert!(OperandValue::ISize(-42) == OperandValue::ISize(-42));
        assert_eq!(OperandValue::ISize(-42) == OperandValue::ISize(-41), false);
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-43));
        assert_eq!(OperandValue::ISize(-42) != OperandValue::ISize(-42), false);
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-41));
        assert!(OperandValue::ISize(-42) > OperandValue::ISize(-43));
        assert_eq!(OperandValue::ISize(-42) > OperandValue::ISize(-42), false);
        assert_eq!(OperandValue::ISize(-42) > OperandValue::ISize(-41), false);
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-43));
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-42));
        assert_eq!(OperandValue::ISize(-42) >= OperandValue::ISize(-41), false);
        assert_eq!(OperandValue::ISize(-42) < OperandValue::ISize(-43), false);
        assert_eq!(OperandValue::ISize(-42) < OperandValue::ISize(-42), false);
        assert!(OperandValue::ISize(-42) < OperandValue::ISize(-41));
        assert_eq!(OperandValue::ISize(-42) <= OperandValue::ISize(-43), false);
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-42));
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-41));
    }

    #[test]
    fn test_operand_value_bool() {
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(false), false);
        assert!(OperandValue::Bool(true) == OperandValue::Bool(true));
        assert!(OperandValue::Bool(true) != OperandValue::Bool(false));
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(true), false);
        assert!(OperandValue::Bool(true) > OperandValue::Bool(false));
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(true), false);
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(false));
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(true));
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(false), false);
        assert!(OperandValue::Bool(true) <= OperandValue::Bool(true));
    }

    #[test]
    fn test_operand_value_string() {
        assert_eq!(OperandValue::from("j") == OperandValue::from("i"), false);
        assert!(OperandValue::from("j") == OperandValue::from("j"));
        assert_eq!(OperandValue::from("j") == OperandValue::from("k"), false);
        assert!(OperandValue::from("j") != OperandValue::from("i"));
        assert_eq!(OperandValue::from("j") != OperandValue::from("j"), false);
        assert!(OperandValue::from("j") != OperandValue::from("k"));
        assert!(OperandValue::from("j") > OperandValue::from("i"));
        assert_eq!(OperandValue::from("j") > OperandValue::from("j"), false);
        assert_eq!(OperandValue::from("j") > OperandValue::from("k"), false);
        assert!(OperandValue::from("j") >= OperandValue::from("i"));
        assert!(OperandValue::from("j") >= OperandValue::from("j"));
        assert_eq!(OperandValue::from("j") >= OperandValue::from("k"), false);
        assert_eq!(OperandValue::from("j") < OperandValue::from("i"), false);
        assert_eq!(OperandValue::from("j") < OperandValue::from("j"), false);
        assert!(OperandValue::from("j") < OperandValue::from("k"));
        assert_eq!(OperandValue::from("j") <= OperandValue::from("i"), false);
        assert!(OperandValue::from("j") <= OperandValue::from("j"));
        assert!(OperandValue::from("j") <= OperandValue::from("k"));
    }

    #[test]
    fn test_operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::from("a b c")), None);
        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::from("a b c")), None);
    }
}
