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

impl std::fmt::Display for OperandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            OperandValue::U64(val) => val.to_string(),
            OperandValue::I64(val) => val.to_string(),
            OperandValue::F64(val) => val.to_string(),
            OperandValue::USize(val) => val.to_string(),
            OperandValue::ISize(val) => val.to_string(),
            OperandValue::Bool(val) => val.to_string(),
            OperandValue::Str(val) => "\"".to_string() + val + "\"",
        };
        write!(f, "{}", str)
    }
}

pub fn operand_value_from_value(value: &Value) -> Option<OperandValue> {
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
mod tests {
    use crate::value::{
        Value,
        stub::{arr_bool_stub, arr_f64_stub, arr_i64_stub, arr_isize_stub, arr_str_stub, arr_u64_stub, arr_usize_stub, obj_stub},
    };

    use super::{OperandValue, operand_value_from_value};

    #[test]
    fn operand_value_from() {
        let u64_num: u64 = 8;
        let i64_num: i64 = -3;
        let f64_num: f64 = -9.8;
        let usize_num: usize = 183;
        let isize_num: isize = -892;
        assert_eq!(OperandValue::from(u64_num), OperandValue::U64(u64_num));
        assert_eq!(OperandValue::from(i64_num), OperandValue::I64(i64_num));
        assert_eq!(OperandValue::from(f64_num), OperandValue::F64(f64_num));
        assert_eq!(OperandValue::from(usize_num), OperandValue::USize(usize_num));
        assert_eq!(OperandValue::from(isize_num), OperandValue::ISize(isize_num));
        assert_eq!(OperandValue::from(false), OperandValue::Bool(false));
        assert_eq!(OperandValue::from("in vino veritas"), OperandValue::Str("in vino veritas".into()));
    }

    #[test]
    fn operand_value_u64_smaller() {
        assert!(OperandValue::U64(42) != OperandValue::U64(41));
        assert!(OperandValue::U64(42) > OperandValue::U64(41));
        assert!(OperandValue::U64(42) >= OperandValue::U64(41));
    }

    #[test]
    fn operand_value_u64_equals() {
        assert!(OperandValue::U64(42) == OperandValue::U64(42));
        assert!(OperandValue::U64(42) >= OperandValue::U64(42));
        assert!(OperandValue::U64(42) <= OperandValue::U64(42));
    }

    #[test]
    fn operand_value_u64_greater() {
        assert!(OperandValue::U64(42) != OperandValue::U64(43));
        assert!(OperandValue::U64(42) < OperandValue::U64(43));
        assert!(OperandValue::U64(42) <= OperandValue::U64(43));
    }

    #[test]
    fn operand_value_i64_smaller() {
        assert!(OperandValue::I64(-42) != OperandValue::I64(-43));
        assert!(OperandValue::I64(-42) > OperandValue::I64(-43));
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-43));
    }

    #[test]
    fn operand_value_i64_equals() {
        assert!(OperandValue::I64(-42) == OperandValue::I64(-42));
        assert!(OperandValue::I64(-42) >= OperandValue::I64(-42));
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-42));
    }

    #[test]
    fn operand_value_i64_greater() {
        assert!(OperandValue::I64(-42) != OperandValue::I64(-41));
        assert!(OperandValue::I64(-42) < OperandValue::I64(-41));
        assert!(OperandValue::I64(-42) <= OperandValue::I64(-41));
    }

    #[test]
    fn operand_value_f64_smaller() {
        assert!(OperandValue::F64(-42.5) != OperandValue::F64(-43.5));
        assert!(OperandValue::F64(-42.5) > OperandValue::F64(-43.5));
        assert!(OperandValue::F64(-42.5) >= OperandValue::F64(-43.5));
    }

    #[test]
    fn operand_value_f64_equals() {
        assert!(OperandValue::F64(-42.5) == OperandValue::F64(-42.5));
        assert!(OperandValue::F64(-42.5) >= OperandValue::F64(-42.5));
        assert!(OperandValue::F64(-42.5) <= OperandValue::F64(-42.5));
    }

    #[test]
    fn operand_value_f64_greater() {
        assert!(OperandValue::F64(-42.5) != OperandValue::F64(-41.5));
        assert!(OperandValue::F64(-42.5) < OperandValue::F64(-41.5));
        assert!(OperandValue::F64(-42.5) <= OperandValue::F64(-41.5));
    }

    #[test]
    fn operand_value_usize_smaller() {
        assert!(OperandValue::USize(42) != OperandValue::USize(41));
        assert!(OperandValue::USize(42) > OperandValue::USize(41));
        assert!(OperandValue::USize(42) >= OperandValue::USize(41));
    }

    #[test]
    fn operand_value_usize_equals() {
        assert!(OperandValue::USize(42) == OperandValue::USize(42));
        assert!(OperandValue::USize(42) >= OperandValue::USize(42));
        assert!(OperandValue::USize(42) <= OperandValue::USize(42));
    }

    #[test]
    fn operand_value_usize_greater() {
        assert!(OperandValue::USize(42) != OperandValue::USize(43));
        assert!(OperandValue::USize(42) < OperandValue::USize(43));
        assert!(OperandValue::USize(42) <= OperandValue::USize(43));
    }

    #[test]
    fn operand_value_isize_smaller() {
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-43));
        assert!(OperandValue::ISize(-42) > OperandValue::ISize(-43));
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-43));
    }

    #[test]
    fn operand_value_isize_equals() {
        assert!(OperandValue::ISize(-42) == OperandValue::ISize(-42));
        assert!(OperandValue::ISize(-42) >= OperandValue::ISize(-42));
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-42));
    }

    #[test]
    fn operand_value_isize_greater() {
        assert!(OperandValue::ISize(-42) != OperandValue::ISize(-41));
        assert!(OperandValue::ISize(-42) < OperandValue::ISize(-41));
        assert!(OperandValue::ISize(-42) <= OperandValue::ISize(-41));
    }

    #[test]
    fn operand_value_bool_smaller() {
        assert!(OperandValue::Bool(true) != OperandValue::Bool(false));
        assert!(OperandValue::Bool(true) > OperandValue::Bool(false));
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(false));
    }

    #[test]
    fn operand_value_bool_equals() {
        assert!(OperandValue::Bool(true) == OperandValue::Bool(true));
        assert!(OperandValue::Bool(true) >= OperandValue::Bool(true));
        assert!(OperandValue::Bool(true) <= OperandValue::Bool(true));
        assert!(OperandValue::Bool(false) == OperandValue::Bool(false));
        assert!(OperandValue::Bool(false) >= OperandValue::Bool(false));
        assert!(OperandValue::Bool(false) <= OperandValue::Bool(false));
    }

    #[test]
    fn operand_value_bool_greater() {
        assert!(OperandValue::Bool(false) != OperandValue::Bool(true));
        assert!(OperandValue::Bool(false) < OperandValue::Bool(true));
        assert!(OperandValue::Bool(false) <= OperandValue::Bool(true));
    }

    #[test]
    fn operand_value_str_smaller() {
        assert!(OperandValue::from("rock lee") != OperandValue::from("neji"));
        assert!(OperandValue::from("rock lee") > OperandValue::from("neji"));
        assert!(OperandValue::from("rock lee") >= OperandValue::from("neji"));
    }

    #[test]
    fn operand_value_str_equals() {
        assert!(OperandValue::from("rock lee") == OperandValue::from("rock lee"));
        assert!(OperandValue::from("rock lee") >= OperandValue::from("rock lee"));
        assert!(OperandValue::from("rock lee") <= OperandValue::from("rock lee"));
    }

    #[test]
    fn operand_value_str_greter() {
        assert!(OperandValue::from("rock lee") != OperandValue::from("tenten"));
        assert!(OperandValue::from("rock lee") < OperandValue::from("tenten"));
        assert!(OperandValue::from("rock lee") <= OperandValue::from("tenten"));
    }

    #[test]
    fn operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::USize(42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::ISize(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::from("a b c")), None);

        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::USize(42)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::ISize(-42)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::from("a b c")), None);

        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::USize(42)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::ISize(-42)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::from("a b c")), None);

        assert_eq!(OperandValue::USize(42).partial_cmp(&OperandValue::ISize(-42)), None);
        assert_eq!(OperandValue::USize(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::USize(42).partial_cmp(&OperandValue::from("a b c")), None);

        assert_eq!(OperandValue::ISize(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::ISize(-42).partial_cmp(&OperandValue::from("a b c")), None);

        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::from("a b c")), None);
    }

    #[test]
    fn operand_value_from_value_some() {
        assert_eq!(operand_value_from_value(&Value::U64(42)), Some(OperandValue::U64(42)));
        assert_eq!(operand_value_from_value(&Value::I64(-42)), Some(OperandValue::I64(-42)));
        assert_eq!(operand_value_from_value(&Value::F64(-42.5)), Some(OperandValue::F64(-42.5)));
        assert_eq!(operand_value_from_value(&Value::USize(42)), Some(OperandValue::USize(42)));
        assert_eq!(operand_value_from_value(&Value::ISize(-42)), Some(OperandValue::ISize(-42)));
        assert_eq!(operand_value_from_value(&Value::Bool(false)), Some(OperandValue::Bool(false)));
        assert_eq!(operand_value_from_value(&Value::Str("Naruto".into())), Some(OperandValue::Str("Naruto".into())));
    }

    #[test]
    fn operand_value_from_value_none() {
        assert_eq!(operand_value_from_value(&Value::None), None);
        assert_eq!(operand_value_from_value(&arr_u64_stub()), None);
        assert_eq!(operand_value_from_value(&arr_i64_stub()), None);
        assert_eq!(operand_value_from_value(&arr_f64_stub()), None);
        assert_eq!(operand_value_from_value(&arr_usize_stub()), None);
        assert_eq!(operand_value_from_value(&arr_isize_stub()), None);
        assert_eq!(operand_value_from_value(&arr_bool_stub()), None);
        assert_eq!(operand_value_from_value(&arr_str_stub()), None);
        assert_eq!(operand_value_from_value(&obj_stub()), None);
    }

    #[test]
    fn test_operand_value_to_string() {
        assert_eq!(OperandValue::U64(4).to_string(), "4".to_string());
        assert_eq!(OperandValue::I64(-22).to_string(), "-22".to_string());
        assert_eq!(OperandValue::F64(-3.65).to_string(), "-3.65".to_string());
        assert_eq!(OperandValue::USize(19).to_string(), "19".to_string());
        assert_eq!(OperandValue::ISize(-47).to_string(), "-47".to_string());
        assert_eq!(OperandValue::Bool(true).to_string(), "true".to_string());
        assert_eq!(OperandValue::from("Non sequitur").to_string(), r#""Non sequitur""#.to_string());
    }
}
