use crate::value::Value;

pub fn resolve_path(value: &Value, field_path: String) -> Option<Value> {
    let mut paths: Vec<&str> = field_path.split(".").collect();

    if paths.len() > 0 {
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
            _ => None,
        }
    } else {
        None
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
    fn test_resolve_path_obj_() {
        assert_eq!(resolve_path(&obj_stub(), String::from("name")), Some(Value::from("The Beatles")));
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
}
