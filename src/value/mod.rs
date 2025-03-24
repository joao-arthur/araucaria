use std::collections::HashMap;

#[cfg(test)]
pub mod stub;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    None,
    NumU(u64),
    NumI(i64),
    NumF(f64),
    Bool(bool),
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

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::None => String::from(""),
        Value::NumU(val) => val.to_string(),
        Value::NumI(val) => val.to_string(),
        Value::NumF(val) => val.to_string(),
        Value::Bool(val) => val.to_string(),
        Value::Str(val) => String::from("\"") + val + "\"",
        Value::Arr(val) => String::from("[") + &val.iter().map(|v| value_to_string(v)).collect::<Vec<String>>().join(", ") + "]",
        Value::Obj(val) => String::from("{ ") + &val.iter().map(|(k, v)| k.clone() + ": " + &value_to_string(v)).collect::<Vec<String>>().join(", ") +  " }",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_from() {
        assert_eq!(Value::from(8 as u64), Value::NumU(8));
        assert_eq!(Value::from(-3 as i64), Value::NumI(-3));
        assert_eq!(Value::from(-9.8), Value::NumF(-9.8));
        assert_eq!(Value::from(false), Value::Bool(false));
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

    #[test]
    fn test_value_to_string() {
        assert_eq!(value_to_string(&Value::None), String::from(""));
        assert_eq!(value_to_string(&Value::NumU(4)), String::from("4"));
        assert_eq!(value_to_string(&Value::NumI(-22)), String::from("-22"));
        assert_eq!(value_to_string(&Value::NumF(-3.65)), String::from("-3.65"));
        assert_eq!(value_to_string(&Value::Bool(true)), String::from("true"));
        assert_eq!(value_to_string(&Value::from("Non sequitur")), String::from(r#""Non sequitur""#));
        assert_eq!(value_to_string(
            &Value::from([
                Value::from("Ad nauseam"),
                Value::from("Ad ignorantiam"),
                Value::from([
                    Value::from("Ad hominem"),
                    Value::from("Ad verecundiam"),
                ])
            ])),
            String::from(r#"["Ad nauseam", "Ad ignorantiam", ["Ad hominem", "Ad verecundiam"]]"#)
        );
        assert_eq!(value_to_string(
            &Value::from([
                (String::from("k_num"), Value::NumU(837)),
                (String::from("k_bool"), Value::Bool(false)),
                (String::from("k_str"), Value::from("Augustus")),
                (String::from("k_nested"), Value::from([
                    (String::from("l_1"), Value::from([
                        (String::from("l_2"), Value::from([
                            Value::from([
                                (String::from("id"), Value::NumU(0))
                            ])
                        ]))
                    ]))
                ])),
            ])),
            String::from(r#"{ k_num: 837, k_bool: false, k_str: "Augustus", k_nested: { l_1: { l_2: [{ id: 0 }] } } }"#)
        );
    }
}
