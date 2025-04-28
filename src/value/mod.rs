use std::collections::BTreeMap;

pub mod stub;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
    Arr(Vec<Value>),
    Obj(BTreeMap<String, Value>),
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.into())
    }
}

impl<const N: usize> From<[Value; N]> for Value {
    fn from(value: [Value; N]) -> Self {
        Value::Arr(value.to_vec())
    }
}

impl<const N: usize> From<[u64; N]> for Value {
    fn from(value: [u64; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::U64(*v)).collect())
    }
}

impl<const N: usize> From<[i64; N]> for Value {
    fn from(value: [i64; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::I64(*v)).collect())
    }
}

impl<const N: usize> From<[f64; N]> for Value {
    fn from(value: [f64; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::F64(*v)).collect())
    }
}

impl<const N: usize> From<[usize; N]> for Value {
    fn from(value: [usize; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::USize(*v)).collect())
    }
}

impl<const N: usize> From<[isize; N]> for Value {
    fn from(value: [isize; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::ISize(*v)).collect())
    }
}

impl<const N: usize> From<[bool; N]> for Value {
    fn from(value: [bool; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::Bool(*v)).collect())
    }
}

impl<const N: usize> From<[&str; N]> for Value {
    fn from(value: [&str; N]) -> Self {
        Value::Arr(value.to_vec().iter().map(|v| Value::Str((*v).into())).collect())
    }
}

impl<const N: usize> From<[(String, Value); N]> for Value {
    fn from(value: [(String, Value); N]) -> Self {
        Value::Obj(BTreeMap::from(value))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", value_to_string(self))
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::None => "".into(),
        Value::U64(val) => val.to_string(),
        Value::I64(val) => val.to_string(),
        Value::F64(val) => val.to_string(),
        Value::USize(val) => val.to_string(),
        Value::ISize(val) => val.to_string(),
        Value::Bool(val) => val.to_string(),
        Value::Str(val) => "\"".to_string() + val + "\"",
        Value::Arr(val) => {
            let parts: Vec<String> = val.iter().map(value_to_string).collect();
            "[ ".to_string() + &parts.join(", ") + " ]"
        }
        Value::Obj(val) => {
            let mut parts: Vec<String> = val.iter().map(|(k, v)| k.clone() + ": " + &value_to_string(v)).collect();
            parts.sort();
            "{ ".to_string() + &parts.join(", ") + " }"
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::Value;

    #[test]
    fn value_from() {
        assert_eq!(Value::from(8_u64), Value::U64(8));
        assert_eq!(Value::from(-3_i64), Value::I64(-3));
        assert_eq!(Value::from(-9.8), Value::F64(-9.8));
        assert_eq!(Value::from(3_usize), Value::USize(3));
        assert_eq!(Value::from(-9_isize), Value::ISize(-9));
        assert_eq!(Value::from(false), Value::Bool(false));
        assert_eq!(Value::from("in vino veritas"), Value::Str("in vino veritas".into()));
        assert_eq!(
            Value::from([Value::from("veni"), Value::from("vidi"), Value::from("vici"), Value::Bool(false), Value::F64(-5.1)]),
            Value::Arr(vec![Value::Str("veni".into()), Value::Str("vidi".into()), Value::Str("vici".into()), Value::Bool(false), Value::F64(-5.1)])
        );
        assert_eq!(
            Value::from([("age".into(), Value::from(82_u64)), ("name".into(), Value::from("Paul")), ("alive".into(), Value::from(true))]),
            Value::Obj(BTreeMap::from([
                ("age".into(), Value::U64(82)),
                ("name".into(), Value::Str("Paul".into())),
                ("alive".into(), Value::Bool(true)),
            ]))
        );
    }

    #[test]
    fn value_from_slice() {
        let bool_slice: [bool; 3] = [false, true, true];
        let u64_slice: [u64; 3] = [9, 213897, 2394];
        let i64_slice: [i64; 3] = [-9, -213897, -2394];
        let f64_slice: [f64; 3] = [-9.5, -213897.5, -2394.5];
        let usize_slice: [usize; 3] = [9, 213897, 2394];
        let isize_slice: [isize; 3] = [-9, -213897, -2394];
        let str_slice: [&str; 3] = ["veni", "vidi", "vici"];

        let bool_value_arr = vec![Value::Bool(false), Value::Bool(true), Value::Bool(true)];
        let u64_value_arr = vec![Value::U64(9), Value::U64(213897), Value::U64(2394)];
        let i64_value_arr = vec![Value::I64(-9), Value::I64(-213897), Value::I64(-2394)];
        let f64_value_arr = vec![Value::F64(-9.5), Value::F64(-213897.5), Value::F64(-2394.5)];
        let usize_value_arr = vec![Value::USize(9), Value::USize(213897), Value::USize(2394)];
        let isize_value_arr = vec![Value::ISize(-9), Value::ISize(-213897), Value::ISize(-2394)];
        let str_value_arr = vec![Value::Str("veni".into()), Value::Str("vidi".into()), Value::Str("vici".into())];

        assert_eq!(Value::from(bool_slice), Value::Arr(bool_value_arr));
        assert_eq!(Value::from(u64_slice), Value::Arr(u64_value_arr));
        assert_eq!(Value::from(i64_slice), Value::Arr(i64_value_arr));
        assert_eq!(Value::from(f64_slice), Value::Arr(f64_value_arr));
        assert_eq!(Value::from(usize_slice), Value::Arr(usize_value_arr));
        assert_eq!(Value::from(isize_slice), Value::Arr(isize_value_arr));
        assert_eq!(Value::from(str_slice), Value::Arr(str_value_arr));
    }

    #[test]
    fn test_value_to_string() {
        let arr = Value::from([Value::from("Ad nauseam"), Value::from("Ad ignorantiam"), Value::from(["Ad hominem", "Ad verecundiam"])]);
        let obj = Value::from([
            ("k_num".into(), Value::U64(837)),
            ("k_bool".into(), Value::Bool(false)),
            ("k_str".into(), Value::from("Augustus")),
            (
                "k_nested".into(),
                Value::from([("l_1".into(), Value::from([("l_2".into(), Value::from([Value::from([("id".into(), Value::U64(0))])]))]))]),
            ),
        ]);
        assert_eq!(Value::None.to_string(), "".to_string());
        assert_eq!(Value::U64(4).to_string(), "4".to_string());
        assert_eq!(Value::I64(-22).to_string(), "-22".to_string());
        assert_eq!(Value::F64(-3.65).to_string(), "-3.65".to_string());
        assert_eq!(Value::USize(19).to_string(), "19".to_string());
        assert_eq!(Value::ISize(-47).to_string(), "-47".to_string());
        assert_eq!(Value::Bool(true).to_string(), "true".to_string());
        assert_eq!(Value::from("Non sequitur").to_string(), r#""Non sequitur""#.to_string());
        assert_eq!(arr.to_string(), r#"[ "Ad nauseam", "Ad ignorantiam", [ "Ad hominem", "Ad verecundiam" ] ]"#.to_string());
        assert_eq!(obj.to_string(), r#"{ k_bool: false, k_nested: { l_1: { l_2: [ { id: 0 } ] } }, k_num: 837, k_str: "Augustus" }"#.to_string());
    }
}
