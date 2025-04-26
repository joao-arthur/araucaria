use crate::value::Value;

pub fn resolve_path(value: &Value, field_path: &str) -> Option<Value> {
    let mut current = value;
    for key in field_path.split('.') {
        current = match current {
            Value::Obj(obj) => obj.get(key)?,
            Value::Arr(arr) => arr.get(key.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::value::{
        Value,
        stub::{
            arr_bool_stub, arr_f64_stub, arr_i64_stub, arr_isize_stub, arr_num_stub, arr_str_stub, arr_u64_stub, arr_usize_stub, bool_stub, f64_stub,
            i64_stub, isize_stub, obj_stub, str_stub, u64_stub, usize_stub,
        },
    };

    use super::resolve_path;

    #[test]
    fn resolve_path_empty_path_not_applyable_types() {
        assert_eq!(resolve_path(&u64_stub(), ""), None);
        assert_eq!(resolve_path(&i64_stub(), ""), None);
        assert_eq!(resolve_path(&f64_stub(), ""), None);
        assert_eq!(resolve_path(&usize_stub(), ""), None);
        assert_eq!(resolve_path(&isize_stub(), ""), None);
        assert_eq!(resolve_path(&bool_stub(), ""), None);
        assert_eq!(resolve_path(&str_stub(), ""), None);
    }

    #[test]
    fn resolve_path_dot_path_not_applyable_types() {
        assert_eq!(resolve_path(&u64_stub(), "."), None);
        assert_eq!(resolve_path(&i64_stub(), "."), None);
        assert_eq!(resolve_path(&f64_stub(), "."), None);
        assert_eq!(resolve_path(&usize_stub(), "."), None);
        assert_eq!(resolve_path(&isize_stub(), "."), None);
        assert_eq!(resolve_path(&bool_stub(), "."), None);
        assert_eq!(resolve_path(&str_stub(), "."), None);
    }

    #[test]
    fn resolve_path_zero_path_not_applyable_types() {
        assert_eq!(resolve_path(&u64_stub(), "0"), None);
        assert_eq!(resolve_path(&i64_stub(), "0"), None);
        assert_eq!(resolve_path(&f64_stub(), "0"), None);
        assert_eq!(resolve_path(&usize_stub(), "0"), None);
        assert_eq!(resolve_path(&isize_stub(), "0"), None);
        assert_eq!(resolve_path(&bool_stub(), "0"), None);
        assert_eq!(resolve_path(&str_stub(), "0"), None);
    }

    #[test]
    fn resolve_path_name_path_not_applyable_types() {
        assert_eq!(resolve_path(&u64_stub(), "name"), None);
        assert_eq!(resolve_path(&i64_stub(), "name"), None);
        assert_eq!(resolve_path(&f64_stub(), "name"), None);
        assert_eq!(resolve_path(&usize_stub(), "name"), None);
        assert_eq!(resolve_path(&isize_stub(), "name"), None);
        assert_eq!(resolve_path(&bool_stub(), "name"), None);
        assert_eq!(resolve_path(&str_stub(), "name"), None);
    }

    #[test]
    fn resolve_path_empty_path() {
        assert_eq!(resolve_path(&arr_str_stub(), ""), None);
        assert_eq!(resolve_path(&obj_stub(), ""), None);
        assert_eq!(resolve_path(&Value::Obj(BTreeMap::from([("".into(), Value::from(true))])), ""), Some(Value::from(true)));
    }

    #[test]
    fn resolve_path_dot_path() {
        assert_eq!(resolve_path(&arr_str_stub(), "."), None);
        assert_eq!(resolve_path(&obj_stub(), "."), None);
        assert_eq!(resolve_path(&Value::Obj(BTreeMap::from([("".into(), Value::from(true))])), "."), None);
    }

    #[test]
    fn resolve_path_obj_some() {
        assert_eq!(resolve_path(&obj_stub(), "name"), Some(Value::from("The Beatles")));
    }

    #[test]
    fn resolve_path_obj_none() {
        assert_eq!(resolve_path(&obj_stub(), "name.name"), None);
    }

    #[test]
    fn resolve_path_arr_some() {
        assert_eq!(resolve_path(&arr_u64_stub(), "0"), Some(Value::U64(1)));
        assert_eq!(resolve_path(&arr_i64_stub(), "1"), Some(Value::I64(0)));
        assert_eq!(resolve_path(&arr_f64_stub(), "2"), Some(Value::F64(10.5)));
        assert_eq!(resolve_path(&arr_usize_stub(), "0"), Some(Value::USize(1)));
        assert_eq!(resolve_path(&arr_isize_stub(), "1"), Some(Value::ISize(0)));
        assert_eq!(resolve_path(&arr_num_stub(), "2"), Some(Value::F64(1.25)));
        assert_eq!(resolve_path(&arr_bool_stub(), "0"), Some(Value::Bool(false)));
        assert_eq!(resolve_path(&arr_str_stub(), "1"), Some(Value::from("John Lennon")));
    }

    #[test]
    fn resolve_path_arr_none() {
        assert_eq!(resolve_path(&arr_u64_stub(), "zero"), None);
        assert_eq!(resolve_path(&arr_i64_stub(), "one"), None);
        assert_eq!(resolve_path(&arr_f64_stub(), "two"), None);
        assert_eq!(resolve_path(&arr_usize_stub(), "three"), None);
        assert_eq!(resolve_path(&arr_isize_stub(), "four"), None);
    }

    #[test]
    fn resolve_path_arr_zero_padding() {
        assert_eq!(resolve_path(&arr_u64_stub(), "00"), Some(Value::U64(1)));
        assert_eq!(resolve_path(&arr_i64_stub(), "001"), Some(Value::I64(0)));
        assert_eq!(resolve_path(&arr_f64_stub(), "0002"), Some(Value::F64(10.5)));
        assert_eq!(resolve_path(&arr_usize_stub(), "00000"), Some(Value::USize(1)));
        assert_eq!(resolve_path(&arr_isize_stub(), "000001"), Some(Value::ISize(0)));
        assert_eq!(resolve_path(&arr_num_stub(), "0000002"), Some(Value::F64(1.25)));
        assert_eq!(resolve_path(&arr_bool_stub(), "00000000"), Some(Value::Bool(false)));
        assert_eq!(resolve_path(&arr_str_stub(), "000000001"), Some(Value::from("John Lennon")));
    }

    #[test]
    fn resolve_path_arr_other_notations() {
        assert_eq!(resolve_path(&arr_u64_stub(), "0x1"), None);
        assert_eq!(resolve_path(&arr_i64_stub(), "0X1"), None);
        assert_eq!(resolve_path(&arr_f64_stub(), "0b1"), None);
        assert_eq!(resolve_path(&arr_usize_stub(), "0B1"), None);
        assert_eq!(resolve_path(&arr_isize_stub(), "0o1"), None);
        assert_eq!(resolve_path(&arr_num_stub(), "0O1"), None);
    }

    #[test]
    fn resolve_path_arr_out_of_bounds() {
        assert_eq!(resolve_path(&arr_num_stub(), "-2"), None);
        assert_eq!(resolve_path(&arr_num_stub(), "-1"), None);
        assert_eq!(resolve_path(&arr_num_stub(), "5"), None);
        assert_eq!(resolve_path(&arr_num_stub(), "6"), None);
    }

    #[test]
    fn resolve_path_obj_nested() {
        let obj = Value::Obj(BTreeMap::from([(
            "user".into(),
            Value::Obj(BTreeMap::from([(
                "account".into(),
                Value::Obj(BTreeMap::from([("details".into(), Value::Obj(BTreeMap::from([("birthdate".into(), Value::from("2000-08-22"))])))])),
            )])),
        )]));
        assert_eq!(resolve_path(&obj, "user.account.details.birthdate"), Some(Value::from("2000-08-22")));
    }

    #[test]
    fn resolve_path_arr_nested() {
        let arr = Value::from([
            Value::U64(1),
            Value::U64(2),
            Value::from([
                Value::U64(10),
                Value::U64(20),
                Value::from([Value::U64(100), Value::U64(200), Value::from([Value::U64(1000), Value::U64(2000), Value::U64(3000)])]),
            ]),
        ]);
        assert_eq!(resolve_path(&arr, "2.2.2.2"), Some(Value::U64(3000)));
    }

    #[test]
    fn resolve_path_nested() {
        let value = Value::Obj(BTreeMap::from([(
            "0".into(),
            Value::from([
                Value::U64(1),
                Value::U64(2),
                Value::from([
                    Value::U64(10),
                    Value::U64(20),
                    Value::Obj(BTreeMap::from([(
                        "user".into(),
                        Value::Obj(BTreeMap::from([(
                            "account".into(),
                            Value::Obj(BTreeMap::from([("details".into(), Value::from([Value::U64(111), Value::U64(222), Value::U64(333)]))])),
                        )])),
                    )])),
                ]),
            ]),
        )]));
        assert_eq!(resolve_path(&value, "0.2.2.user.account.details.1"), Some(Value::U64(222)));
    }
}
