use crate::value::Value;

pub fn resolve_path(value: &Value, field_path: String) -> Option<Value> {
    let mut paths: Vec<&str> = field_path.split(".").collect();
    let field = paths.remove(0);
    match value {
        Value::Obj(obj) => {
            let curr_value = obj.get(field);
            match curr_value {
                Some(v) => {
                    if paths.len() > 0 {
                        return resolve_path(v, paths.join("."));
                    } else {
                        return Some(v.clone());
                    }
                }
                None => {
                    return None;
                }
            }
        }
        Value::Arr(arr) => {
            if field.chars().all(|c| c.is_ascii_digit()) {
                println!("{}", field);
                let index = field.parse::<usize>();
                match index {
                    Ok(index) => {
                        let curr_value = arr.get(index);
                        match curr_value {
                            Some(v) => {
                                if paths.len() > 0 {
                                    return resolve_path(v, paths.join("."));
                                } else {
                                    return Some(v.clone());
                                }
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            return None;
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::value::{
        stub::{
            arr_bool_stub, arr_num_f_stub, arr_num_i_stub, arr_num_stub, arr_num_u_stub, arr_str_stub, bool_stub, num_f_stub, num_i_stub, num_u_stub,
            obj_stub, str_stub,
        },
        Value,
    };

    use super::resolve_path;

    #[test]
    fn test_resolve_path_empty_path() {
        assert_eq!(resolve_path(&num_u_stub(), String::from("")), None);
        assert_eq!(resolve_path(&num_i_stub(), String::from("")), None);
        assert_eq!(resolve_path(&num_f_stub(), String::from("")), None);
        assert_eq!(resolve_path(&bool_stub(), String::from("")), None);
        assert_eq!(resolve_path(&str_stub(), String::from("")), None);
        assert_eq!(resolve_path(&str_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_bool_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_num_u_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_num_i_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_num_stub(), String::from("")), None);
        assert_eq!(resolve_path(&arr_str_stub(), String::from("")), None);
        assert_eq!(resolve_path(&obj_stub(), String::from("")), None);
    }

    #[test]
    fn test_resolve_path_dot_path() {
        assert_eq!(resolve_path(&num_u_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&num_i_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&num_f_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&bool_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&str_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&str_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_bool_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_num_u_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_num_i_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_num_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&arr_str_stub(), String::from(".")), None);
        assert_eq!(resolve_path(&obj_stub(), String::from(".")), None);
    }

    #[test]
    fn test_resolve_path_obj_other_values() {
        assert_eq!(resolve_path(&num_u_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&num_i_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&num_f_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&bool_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&str_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&str_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_bool_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_num_u_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_num_i_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_num_stub(), String::from("name")), None);
        assert_eq!(resolve_path(&arr_str_stub(), String::from("name")), None);
    }

    #[test]
    fn test_resolve_path_arr_other_values() {
        assert_eq!(resolve_path(&num_u_stub(), String::from("0")), None);
        assert_eq!(resolve_path(&num_i_stub(), String::from("0")), None);
        assert_eq!(resolve_path(&num_f_stub(), String::from("0")), None);
        assert_eq!(resolve_path(&bool_stub(), String::from("0")), None);
        assert_eq!(resolve_path(&obj_stub(), String::from("0")), None);
    }

    #[test]
    fn test_resolve_path_obj_existent() {
        assert_eq!(resolve_path(&obj_stub(), String::from("name")), Some(Value::from("The Beatles")));
    }

    #[test]
    fn test_resolve_path_obj_not_found() {
        assert_eq!(resolve_path(&obj_stub(), String::from("name.name")), None);
    }

    #[test]
    fn test_resolve_path_obj_nested() {
        let obj = Value::Obj(HashMap::from([(
            String::from("user"),
            Value::Obj(HashMap::from([(
                String::from("account"),
                Value::Obj(HashMap::from([(
                    String::from("details"),
                    Value::Obj(HashMap::from([(String::from("birthdate"), Value::from("2000-08-22"))])),
                )])),
            )])),
        )]));
        assert_eq!(resolve_path(&obj, String::from("user.account.details.birthdate")), Some(Value::from("2000-08-22")));
    }

    #[test]
    fn test_resolve_path_arr() {
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("-2")), None);
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("-1")), None);
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("0")), Some(Value::F64(-10.5)));
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("1")), Some(Value::F64(0.5)));
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("2")), Some(Value::F64(10.5)));
        assert_eq!(resolve_path(&arr_num_f_stub(), String::from("3")), None);
    }

    #[test]
    fn test_resolve_path_arr_nested() {
        let arr = Value::from([
            Value::U64(1),
            Value::U64(2),
            Value::from([
                Value::U64(10),
                Value::U64(20),
                Value::from([Value::U64(100), Value::U64(200), Value::from([Value::U64(1000), Value::U64(2000), Value::U64(3000)])]),
            ]),
        ]);
        assert_eq!(resolve_path(&arr, String::from("2.2.2.2")), Some(Value::U64(3000)));
    }

    #[test]
    fn test_resolve_path_nested() {
        let value = Value::Obj(HashMap::from([(
            String::from("0"),
            Value::from([
                Value::U64(1),
                Value::U64(2),
                Value::from([
                    Value::U64(10),
                    Value::U64(20),
                    Value::Obj(HashMap::from([(
                        String::from("user"),
                        Value::Obj(HashMap::from([(
                            String::from("account"),
                            Value::Obj(HashMap::from([(String::from("details"), Value::from([Value::U64(111), Value::U64(222), Value::U64(333)]))])),
                        )])),
                    )])),
                ]),
            ]),
        )]));
        assert_eq!(resolve_path(&value, String::from("0.2.2.user.account.details.1")), Some(Value::U64(222)));
    }
}
