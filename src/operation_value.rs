use crate::{error::ValidationErr, value::Value};

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Ge(Value),
    Le(Value),
}

fn compare(value_a: &Value, operation: &Operation) -> Option<ValidationErr> {
    match operation {
        Operation::Eq(value_b) => match value_b {
            Value::Bool(b) => match value_a {
                Value::Bool(a) => {
                    if a == b {
                        None
                    } else {
                        Some(ValidationErr::Eq(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Eq(value_b.clone())),
            },
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a == b {
                        None
                    } else {
                        Some(ValidationErr::Eq(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Eq(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a == b {
                        None
                    } else {
                        Some(ValidationErr::Eq(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Eq(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a == b {
                        None
                    } else {
                        Some(ValidationErr::Eq(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Eq(value_b.clone())),
            },
            Value::Str(b) => match value_a {
                Value::Str(a) => {
                    if a == b {
                        None
                    } else {
                        Some(ValidationErr::Eq(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Eq(value_b.clone())),
            },
            _ => None,
        },
        Operation::Ne(value_b) => match value_b {
            Value::Bool(b) => match value_a {
                Value::Bool(a) => {
                    if a != b {
                        None
                    } else {
                        Some(ValidationErr::Ne(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ne(value_b.clone())),
            },
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a != b {
                        None
                    } else {
                        Some(ValidationErr::Ne(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ne(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a != b {
                        None
                    } else {
                        Some(ValidationErr::Ne(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ne(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a != b {
                        None
                    } else {
                        Some(ValidationErr::Ne(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ne(value_b.clone())),
            },
            Value::Str(b) => match value_a {
                Value::Str(a) => {
                    if a != b {
                        None
                    } else {
                        Some(ValidationErr::Ne(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ne(value_b.clone())),
            },
            _ => None,
        },
        Operation::Gt(value_b) => match value_b {
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a > b {
                        None
                    } else {
                        Some(ValidationErr::Gt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Gt(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a > b {
                        None
                    } else {
                        Some(ValidationErr::Gt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Gt(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a > b {
                        None
                    } else {
                        Some(ValidationErr::Gt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Gt(value_b.clone())),
            },
            _ => None,
        },
        Operation::Lt(value_b) => match value_b {
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a < b {
                        None
                    } else {
                        Some(ValidationErr::Lt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Lt(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a < b {
                        None
                    } else {
                        Some(ValidationErr::Lt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Lt(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a < b {
                        None
                    } else {
                        Some(ValidationErr::Lt(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Lt(value_b.clone())),
            },
            _ => None,
        },
        Operation::Ge(value_b) => match value_b {
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a >= b {
                        None
                    } else {
                        Some(ValidationErr::Ge(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ge(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a >= b {
                        None
                    } else {
                        Some(ValidationErr::Ge(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ge(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a >= b {
                        None
                    } else {
                        Some(ValidationErr::Ge(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Ge(value_b.clone())),
            },
            _ => None,
        },
        Operation::Le(value_b) => match value_b {
            Value::U64(b) => match value_a {
                Value::U64(a) => {
                    if a <= b {
                        None
                    } else {
                        Some(ValidationErr::Le(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Le(value_b.clone())),
            },
            Value::I64(b) => match value_a {
                Value::I64(a) => {
                    if a <= b {
                        None
                    } else {
                        Some(ValidationErr::Le(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Le(value_b.clone())),
            },
            Value::F64(b) => match value_a {
                Value::F64(a) => {
                    if a <= b {
                        None
                    } else {
                        Some(ValidationErr::Le(value_b.clone()))
                    }
                }
                _ => Some(ValidationErr::Le(value_b.clone())),
            },
            _ => None,
        },
    }
}

#[cfg(test)]
mod test {
    use crate::value::stub::{arr_bool_stub, arr_str_stub, bool_stub, num_f_stub, num_i_stub, num_u_stub, str_stub};

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
        assert_eq!(compare(&value, &Operation::Eq(Value::Bool(false))), Some(ValidationErr::Eq(Value::Bool(false))));
        assert_eq!(compare(&value, &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
    }

    #[test]
    fn test_compare_bool_other_types() {
        assert_eq!(compare(&num_u_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));
        assert_eq!(compare(&num_i_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));
        assert_eq!(compare(&num_f_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));
        assert_eq!(compare(&str_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Value::Bool(true))), Some(ValidationErr::Eq(Value::Bool(true))));

        assert_eq!(compare(&num_u_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
        assert_eq!(compare(&str_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Value::Bool(true))), Some(ValidationErr::Ne(Value::Bool(true))));
    }

    #[test]
    fn test_compare_num_u_none() {
        let value = Value::U64(42);
        assert_eq!(compare(&value, &Operation::Eq(Value::U64(42))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::U64(109))), None);
        assert_eq!(compare(&value, &Operation::Gt(Value::U64(41))), None);
        assert_eq!(compare(&value, &Operation::Lt(Value::U64(43))), None);
        assert_eq!(compare(&value, &Operation::Ge(Value::U64(42))), None);
        assert_eq!(compare(&value, &Operation::Le(Value::U64(42))), None);
    }

    #[test]
    fn test_compare_num_u_some() {
        let value = Value::U64(42);
        assert_eq!(compare(&value, &Operation::Eq(Value::U64(22))), Some(ValidationErr::Eq(Value::U64(22))));
        assert_eq!(compare(&value, &Operation::Ne(Value::U64(42))), Some(ValidationErr::Ne(Value::U64(42))));
        assert_eq!(compare(&value, &Operation::Gt(Value::U64(42))), Some(ValidationErr::Gt(Value::U64(42))));
        assert_eq!(compare(&value, &Operation::Lt(Value::U64(42))), Some(ValidationErr::Lt(Value::U64(42))));
        assert_eq!(compare(&value, &Operation::Ge(Value::U64(43))), Some(ValidationErr::Ge(Value::U64(43))));
        assert_eq!(compare(&value, &Operation::Le(Value::U64(41))), Some(ValidationErr::Le(Value::U64(41))));
    }

    #[test]
    fn test_compare_num_u_other_types() {
        assert_eq!(compare(&bool_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Value::U64(5))), Some(ValidationErr::Eq(Value::U64(5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Value::U64(5))), Some(ValidationErr::Ne(Value::U64(5))));

        assert_eq!(compare(&bool_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(Value::U64(5))), Some(ValidationErr::Gt(Value::U64(5))));

        assert_eq!(compare(&bool_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(Value::U64(5))), Some(ValidationErr::Lt(Value::U64(5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(Value::U64(5))), Some(ValidationErr::Ge(Value::U64(5))));

        assert_eq!(compare(&bool_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
        assert_eq!(compare(&str_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Le(Value::U64(5))), Some(ValidationErr::Le(Value::U64(5))));
    }

    #[test]
    fn test_compare_num_i_none() {
        let value = Value::I64(-42);
        assert_eq!(compare(&value, &Operation::Eq(Value::I64(-42))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::I64(-109))), None);
        assert_eq!(compare(&value, &Operation::Gt(Value::I64(-43))), None);
        assert_eq!(compare(&value, &Operation::Lt(Value::I64(-41))), None);
        assert_eq!(compare(&value, &Operation::Ge(Value::I64(-42))), None);
        assert_eq!(compare(&value, &Operation::Le(Value::I64(-42))), None);
    }

    #[test]
    fn test_compare_num_i_some() {
        let value = Value::I64(-42);
        assert_eq!(compare(&value, &Operation::Eq(Value::I64(-22))), Some(ValidationErr::Eq(Value::I64(-22))));
        assert_eq!(compare(&value, &Operation::Ne(Value::I64(-42))), Some(ValidationErr::Ne(Value::I64(-42))));
        assert_eq!(compare(&value, &Operation::Gt(Value::I64(-42))), Some(ValidationErr::Gt(Value::I64(-42))));
        assert_eq!(compare(&value, &Operation::Lt(Value::I64(-42))), Some(ValidationErr::Lt(Value::I64(-42))));
        assert_eq!(compare(&value, &Operation::Ge(Value::I64(-41))), Some(ValidationErr::Ge(Value::I64(-41))));
        assert_eq!(compare(&value, &Operation::Le(Value::I64(-43))), Some(ValidationErr::Le(Value::I64(-43))));
    }

    #[test]
    fn test_compare_num_i_other_types() {
        assert_eq!(compare(&bool_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Value::I64(-5))), Some(ValidationErr::Eq(Value::I64(-5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Value::I64(-5))), Some(ValidationErr::Ne(Value::I64(-5))));

        assert_eq!(compare(&bool_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(Value::I64(-5))), Some(ValidationErr::Gt(Value::I64(-5))));

        assert_eq!(compare(&bool_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(Value::I64(-5))), Some(ValidationErr::Lt(Value::I64(-5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(Value::I64(-5))), Some(ValidationErr::Ge(Value::I64(-5))));

        assert_eq!(compare(&bool_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
        assert_eq!(compare(&num_f_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
        assert_eq!(compare(&str_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Le(Value::I64(-5))), Some(ValidationErr::Le(Value::I64(-5))));
    }

    #[test]
    fn test_compare_num_f_none() {
        let value = Value::F64(-42.5);
        assert_eq!(compare(&value, &Operation::Eq(Value::F64(-42.5))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::F64(-10.5))), None);
        assert_eq!(compare(&value, &Operation::Gt(Value::F64(-43.5))), None);
        assert_eq!(compare(&value, &Operation::Lt(Value::F64(-41.5))), None);
        assert_eq!(compare(&value, &Operation::Ge(Value::F64(-42.5))), None);
        assert_eq!(compare(&value, &Operation::Le(Value::F64(-42.5))), None);
    }

    #[test]
    fn test_compare_num_f_some() {
        let value = Value::F64(-42.5);
        assert_eq!(compare(&value, &Operation::Eq(Value::F64(-22.5))), Some(ValidationErr::Eq(Value::F64(-22.5))));
        assert_eq!(compare(&value, &Operation::Ne(Value::F64(-42.5))), Some(ValidationErr::Ne(Value::F64(-42.5))));
        assert_eq!(compare(&value, &Operation::Gt(Value::F64(-42.5))), Some(ValidationErr::Gt(Value::F64(-42.5))));
        assert_eq!(compare(&value, &Operation::Lt(Value::F64(-42.5))), Some(ValidationErr::Lt(Value::F64(-42.5))));
        assert_eq!(compare(&value, &Operation::Ge(Value::F64(-41.5))), Some(ValidationErr::Ge(Value::F64(-41.5))));
        assert_eq!(compare(&value, &Operation::Le(Value::F64(-43.5))), Some(ValidationErr::Le(Value::F64(-43.5))));
    }

    #[test]
    fn test_compare_num_f_other_types() {
        assert_eq!(compare(&bool_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Value::F64(-5.5))), Some(ValidationErr::Eq(Value::F64(-5.5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Value::F64(-5.5))), Some(ValidationErr::Ne(Value::F64(-5.5))));

        assert_eq!(compare(&bool_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(Value::F64(-5.5))), Some(ValidationErr::Gt(Value::F64(-5.5))));

        assert_eq!(compare(&bool_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(Value::F64(-5.5))), Some(ValidationErr::Lt(Value::F64(-5.5))));

        assert_eq!(compare(&bool_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(Value::F64(-5.5))), Some(ValidationErr::Ge(Value::F64(-5.5))));

        assert_eq!(compare(&bool_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
        assert_eq!(compare(&num_u_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
        assert_eq!(compare(&num_i_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
        assert_eq!(compare(&str_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
        assert_eq!(compare(&arr_str_stub(), &Operation::Le(Value::F64(-5.5))), Some(ValidationErr::Le(Value::F64(-5.5))));
    }

    #[test]
    fn test_compare_str_none() {
        let value = Value::Str(String::from("Belisarius"));
        assert_eq!(compare(&value, &Operation::Eq(Value::Str(String::from("Belisarius")))), None);
        assert_eq!(compare(&value, &Operation::Ne(Value::Str(String::from("Iustinianus")))), None);
    }

    #[test]
    fn test_compare_str_some() {
        let value = Value::Str(String::from("Belisarius"));
        assert_eq!(
            compare(&value, &Operation::Eq(Value::Str(String::from("Iustinianus")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Iustinianus"))))
        );
        assert_eq!(
            compare(&value, &Operation::Ne(Value::Str(String::from("Belisarius")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Belisarius"))))
        );
    }

    #[test]
    fn test_compare_str_other_types() {
        assert_eq!(
            compare(&bool_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_u_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_i_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_f_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&arr_bool_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&arr_str_stub(), &Operation::Eq(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Eq(Value::Str(String::from("Lemouria"))))
        );

        assert_eq!(
            compare(&bool_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_u_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_i_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&num_f_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&arr_bool_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
        assert_eq!(
            compare(&arr_str_stub(), &Operation::Ne(Value::Str(String::from("Lemouria")))),
            Some(ValidationErr::Ne(Value::Str(String::from("Lemouria"))))
        );
    }
}
