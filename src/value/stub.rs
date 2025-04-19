use std::collections::HashMap;

use crate::value::Value;

pub fn num_u_stub() -> Value {
    Value::U64(42)
}

pub fn num_i_stub() -> Value {
    Value::I64(-42)
}

pub fn num_f_stub() -> Value {
    Value::F64(-21.5)
}

pub fn bool_stub() -> Value {
    Value::Bool(true)
}

pub fn str_stub() -> Value {
    Value::Str(String::from("Lorem ipsum"))
}

pub fn arr_bool_stub() -> Value {
    Value::from([Value::Bool(false), Value::Bool(true), Value::Bool(false), Value::Bool(true)])
}

pub fn arr_num_u_stub() -> Value {
    Value::from([Value::U64(1), Value::U64(10), Value::U64(100)])
}

pub fn arr_num_i_stub() -> Value {
    Value::from([Value::I64(-100), Value::I64(0), Value::I64(100)])
}

pub fn arr_num_f_stub() -> Value {
    Value::from([Value::F64(-10.5), Value::F64(0.5), Value::F64(10.5)])
}

pub fn arr_num_stub() -> Value {
    Value::from([Value::U64(10), Value::I64(-10), Value::F64(1.25)])
}

pub fn arr_str_stub() -> Value {
    Value::from([Value::from("George Harrison"), Value::from("John Lennon"), Value::from("Paul McCartney"), Value::from("Ringo Starr")])
}

pub fn obj_stub() -> Value {
    Value::Obj(HashMap::from([
        (String::from("name"), Value::from("The Beatles")),
        (String::from("members"), Value::from(["George Harrison", "John Lennon", "Paul McCartney", "Ringo Starr"])),
        (String::from("start_year"), Value::U64(1960)),
        (String::from("end_year"), Value::U64(1960)),
        (String::from("number_of_albums"), Value::U64(13)),
        (String::from("greatest_band"), Value::Bool(true)),
    ]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(bool_stub(), Value::Bool(true));
        assert_eq!(num_u_stub(), Value::U64(42));
        assert_eq!(num_i_stub(), Value::I64(-42));
        assert_eq!(num_f_stub(), Value::F64(-21.5));
        assert_eq!(str_stub(), Value::Str(String::from("Lorem ipsum")));
        assert_eq!(arr_bool_stub(), Value::Arr(vec![Value::Bool(false), Value::Bool(true), Value::Bool(false), Value::Bool(true)]));
        assert_eq!(arr_num_u_stub(), Value::Arr(vec![Value::U64(1), Value::U64(10), Value::U64(100)]));
        assert_eq!(arr_num_i_stub(), Value::Arr(vec![Value::I64(-100), Value::I64(0), Value::I64(100)]));
        assert_eq!(arr_num_f_stub(), Value::Arr(vec![Value::F64(-10.5), Value::F64(0.5), Value::F64(10.5)]));
        assert_eq!(arr_num_stub(), Value::Arr(vec![Value::U64(10), Value::I64(-10), Value::F64(1.25)]));
        assert_eq!(
            arr_str_stub(),
            Value::Arr(vec![
                Value::Str(String::from("George Harrison")),
                Value::Str(String::from("John Lennon")),
                Value::Str(String::from("Paul McCartney")),
                Value::Str(String::from("Ringo Starr")),
            ])
        );
        assert_eq!(
            obj_stub(),
            Value::Obj(HashMap::from([
                (String::from("name"), Value::Str(String::from("The Beatles"))),
                (
                    String::from("members"),
                    Value::Arr(vec![
                        Value::Str(String::from("George Harrison")),
                        Value::Str(String::from("John Lennon")),
                        Value::Str(String::from("Paul McCartney")),
                        Value::Str(String::from("Ringo Starr")),
                    ]),
                ),
                (String::from("start_year"), Value::U64(1960)),
                (String::from("end_year"), Value::U64(1960)),
                (String::from("number_of_albums"), Value::U64(13)),
                (String::from("greatest_band"), Value::Bool(true)),
            ]))
        );
    }
}
