use std::collections::HashMap;

#[cfg(test)]
pub mod stub;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    Bool(bool),
    NumU(u64),
    NumI(i64),
    NumF(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::NumU(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::NumI(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::NumF(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(String::from(value))
    }
}

impl<const N: usize> From<[Value; N]> for Value {
    fn from(value: [Value; N]) -> Self {
        Value::Arr(value.to_vec())
    }
}

impl<const N: usize> From<[(String, Value); N]> for Value {
    fn from(value: [(String, Value); N]) -> Self {
        Value::Obj(HashMap::from(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_from() {
        assert_eq!(Value::from(false), Value::Bool(false));
        assert_eq!(Value::from(8 as u64), Value::NumU(8));
        assert_eq!(Value::from(-3 as i64), Value::NumI(-3));
        assert_eq!(Value::from(-9.8), Value::NumF(-9.8));
        assert_eq!(Value::from("in vino veritas"), Value::Str(String::from("in vino veritas")));
        assert_eq!(
            Value::from([Value::from("veni"), Value::from("vidi"), Value::from("vici")]),
            Value::Arr(vec![
                Value::Str(String::from("veni")),
                Value::Str(String::from("vidi")),
                Value::Str(String::from("vici"))
            ])
        );
        assert_eq!(
            Value::from([
                (String::from("age"), Value::from(82 as u64)),
                (String::from("name"), Value::from("Paul")),
                (String::from("alive"), Value::from(true)),
            ]),
            Value::Obj(HashMap::from([
                (String::from("age"), Value::NumU(82)),
                (String::from("name"), Value::Str(String::from("Paul"))),
                (String::from("alive"), Value::Bool(true)),
            ]))
        );
    }
}
