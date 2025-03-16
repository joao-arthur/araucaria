use crate::{error::Err, value::Value};

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Ge(Value),
    Le(Value),
}

fn compare(value_a: &Value, operation: &Operation) -> Option<Err> {
    match operation {
        Operation::Eq(value_b) => match value_b {
            Value::Bool(b) => match value_a {
                Value::Bool(a) => {
                    if a == b {
                        None
                    } else {
                        Some(Err::Eq(value_b.clone()))
                    }
                }
                _ => Some(Err::Eq(value_b.clone())),
            },
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a == b {
                        None
                    } else {
                        Some(Err::Eq(value_b.clone()))
                    }
                }
                _ => Some(Err::Eq(value_b.clone())),
            },
            _ => None,
        },
        Operation::Ne(value_b) => match value_b {
            Value::Bool(b) => match value_a {
                Value::Bool(a) => {
                    if a != b {
                        None
                    } else {
                        Some(Err::Ne(value_b.clone()))
                    }
                }
                _ => Some(Err::Ne(value_b.clone())),
            },
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a != b {
                        None
                    } else {
                        Some(Err::Ne(value_b.clone()))
                    }
                }
                _ => Some(Err::Ne(value_b.clone())),
            },
            _ => None,
        },
        Operation::Gt(value_b) => match value_b {
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a > b {
                        None
                    } else {
                        Some(Err::Gt(value_b.clone()))
                    }
                }
                _ => Some(Err::Gt(value_b.clone())),
            },
            _ => None,
        },
        Operation::Lt(value_b) => match value_b {
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a < b {
                        None
                    } else {
                        Some(Err::Lt(value_b.clone()))
                    }
                }
                _ => Some(Err::Lt(value_b.clone())),
            },
            _ => None,
        },
        Operation::Ge(value_b) => match value_b {
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a >= b {
                        None
                    } else {
                        Some(Err::Ge(value_b.clone()))
                    }
                }
                _ => Some(Err::Ge(value_b.clone())),
            },
            _ => None,
        },
        Operation::Le(value_b) => match value_b {
            Value::NumU(b) => match value_a {
                Value::NumU(a) => {
                    if a <= b {
                        None
                    } else {
                        Some(Err::Le(value_b.clone()))
                    }
                }
                _ => Some(Err::Le(value_b.clone())),
            },
            _ => None,
        },
    }
}

#[cfg(test)]
mod test {
    use crate::value::stub::{
        arr_bool_stub, arr_str_stub, num_f_stub, num_i_stub, num_u_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_compare_bool_none() {
        let value = Value::Bool(true);
        assert_eq!(compare(&value, &Operation::Eq(Value::Bool(true))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::Bool(false))), None);
    }

    #[test]
    fn test_compare_bool_some() {
        let value = Value::Bool(true);
        assert_eq!(compare(&value, &Operation::Eq(Value::Bool(false))), Some(Err::Eq(Value::Bool(false))));
        assert_eq!(compare(&value, &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
    }

    #[test]
    fn test_compare_bool_other_types() {
        assert_eq!(compare(&num_u_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));
        assert_eq!(compare(&num_i_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));
        assert_eq!(compare(&num_f_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));
        assert_eq!(compare(&str_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Value::Bool(true))), Some(Err::Eq(Value::Bool(true))));

        assert_eq!(compare(&num_u_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
        assert_eq!(compare(&str_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Value::Bool(true))), Some(Err::Ne(Value::Bool(true))));
    }

    #[test]
    fn test_compare_num_u_none() {
        let value = Value::NumU(42);
        assert_eq!(compare(&value, &Operation::Eq(Value::NumU(42))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::NumU(109))), None);
        assert_eq!(compare(&value, &Operation::Gt(Value::NumU(41))), None);
        assert_eq!(compare(&value, &Operation::Lt(Value::NumU(43))), None);
        assert_eq!(compare(&value, &Operation::Ge(Value::NumU(42))), None);
        assert_eq!(compare(&value, &Operation::Le(Value::NumU(42))), None);
    }

    #[test]
    fn test_compare_num_u_some() {
        let value = Value::NumU(42);
        assert_eq!(compare(&value, &Operation::Eq(Value::NumU(22))), Some(Err::Eq(Value::NumU(22))));
        assert_eq!(compare(&value, &Operation::Ne(Value::NumU(42))), Some(Err::Ne(Value::NumU(42))));
        assert_eq!(compare(&value, &Operation::Gt(Value::NumU(42))), Some(Err::Gt(Value::NumU(42))));
        assert_eq!(compare(&value, &Operation::Lt(Value::NumU(42))), Some(Err::Lt(Value::NumU(42))));
        assert_eq!(compare(&value, &Operation::Ge(Value::NumU(43))), Some(Err::Ge(Value::NumU(43))));
        assert_eq!(compare(&value, &Operation::Le(Value::NumU(41))), Some(Err::Le(Value::NumU(41))));
    }
}
