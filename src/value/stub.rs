use std::collections::BTreeMap;

use crate::value::Value;

pub fn u64_stub() -> Value {
    Value::U64(42)
}

pub fn i64_stub() -> Value {
    Value::I64(-42)
}

pub fn f64_stub() -> Value {
    Value::F64(-21.5)
}

pub fn usize_stub() -> Value {
    Value::USize(42)
}

pub fn isize_stub() -> Value {
    Value::ISize(-42)
}

pub fn bool_stub() -> Value {
    Value::Bool(true)
}

pub fn str_stub() -> Value {
    Value::Str("Lorem ipsum".into())
}

pub fn arr_bool_stub() -> Value {
    Value::from([Value::Bool(false), Value::Bool(true), Value::Bool(false), Value::Bool(true)])
}

pub fn arr_u64_stub() -> Value {
    Value::from([Value::U64(1), Value::U64(10), Value::U64(100)])
}

pub fn arr_i64_stub() -> Value {
    Value::from([Value::I64(-100), Value::I64(0), Value::I64(100)])
}

pub fn arr_f64_stub() -> Value {
    Value::from([Value::F64(-10.5), Value::F64(0.5), Value::F64(10.5)])
}

pub fn arr_usize_stub() -> Value {
    Value::from([Value::U64(1), Value::U64(10), Value::U64(100)])
}

pub fn arr_isize_stub() -> Value {
    Value::from([Value::I64(-100), Value::I64(0), Value::I64(100)])
}

pub fn arr_num_stub() -> Value {
    Value::from([Value::U64(83), Value::I64(-19), Value::F64(1.25), Value::USize(28), Value::ISize(-378)])
}

pub fn arr_str_stub() -> Value {
    Value::from([Value::from("George Harrison"), Value::from("John Lennon"), Value::from("Paul McCartney"), Value::from("Ringo Starr")])
}

pub fn obj_stub() -> Value {
    Value::Obj(BTreeMap::from([
        ("name".into(), Value::from("The Beatles")),
        ("members".into(), Value::from(["George Harrison", "John Lennon", "Paul McCartney", "Ringo Starr"])),
        ("start_year".into(), Value::U64(1960)),
        ("end_year".into(), Value::U64(1960)),
        ("number_of_albums".into(), Value::U64(13)),
        ("greatest_band".into(), Value::Bool(true)),
    ]))
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::value::Value;

    use super::{
        arr_bool_stub, arr_f64_stub, arr_i64_stub, arr_num_stub, arr_u64_stub, arr_str_stub, bool_stub, f64_stub, i64_stub, u64_stub,
        obj_stub, str_stub,isize_stub, usize_stub, arr_usize_stub, arr_isize_stub
    };

    #[test]
    fn test_stub() {
        assert_eq!(bool_stub(), Value::Bool(true));
        assert_eq!(u64_stub(), Value::U64(42));
        assert_eq!(i64_stub(), Value::I64(-42));
        assert_eq!(f64_stub(), Value::F64(-21.5));
        assert_eq!(usize_stub(), Value::USize(42));
        assert_eq!(isize_stub(), Value::ISize(-42));
        assert_eq!(str_stub(), Value::Str("Lorem ipsum".into()));
        assert_eq!(arr_bool_stub(), Value::Arr(vec![Value::Bool(false), Value::Bool(true), Value::Bool(false), Value::Bool(true)]));
        assert_eq!(arr_u64_stub(), Value::Arr(vec![Value::U64(1), Value::U64(10), Value::U64(100)]));
        assert_eq!(arr_i64_stub(), Value::Arr(vec![Value::I64(-100), Value::I64(0), Value::I64(100)]));
        assert_eq!(arr_f64_stub(), Value::Arr(vec![Value::F64(-10.5), Value::F64(0.5), Value::F64(10.5)]));
        assert_eq!(arr_usize_stub(), Value::Arr(vec![Value::U64(1), Value::U64(10), Value::U64(100)]));
        assert_eq!(arr_isize_stub(), Value::Arr(vec![Value::I64(-100), Value::I64(0), Value::I64(100)]));
        assert_eq!(arr_num_stub(), Value::Arr(vec![Value::U64(83), Value::I64(-19), Value::F64(1.25), Value::USize(28), Value::ISize(-378)]));
        assert_eq!(
            arr_str_stub(),
            Value::Arr(vec![
                Value::Str("George Harrison".into()),
                Value::Str("John Lennon".into()),
                Value::Str("Paul McCartney".into()),
                Value::Str("Ringo Starr".into()),
            ])
        );
        assert_eq!(
            obj_stub(),
            Value::Obj(BTreeMap::from([
                ("name".into(), Value::Str("The Beatles".into())),
                (
                    "members".into(),
                    Value::Arr(vec![
                        Value::Str("George Harrison".into()),
                        Value::Str("John Lennon".into()),
                        Value::Str("Paul McCartney".into()),
                        Value::Str("Ringo Starr".into()),
                    ]),
                ),
                ("start_year".into(), Value::U64(1960)),
                ("end_year".into(), Value::U64(1960)),
                ("number_of_albums".into(), Value::U64(13)),
                ("greatest_band".into(), Value::Bool(true)),
            ]))
        );
    }
}
