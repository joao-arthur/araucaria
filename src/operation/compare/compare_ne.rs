use std::cmp::Ordering;

use super::OperandValue;

pub fn compare_ne(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Greater => Some(Ok(())),
        Ordering::Equal => Some(Err(())),
    }
}

#[cfg(test)]
mod test {
    use super::super::OperandValue;

    use super::compare_ne;

    #[test]
    fn test_compare_ne_u64() {
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::U64(42)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_i64() {
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::I64(-43)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::I64(-42)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::I64(-41)), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_f64() {
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::F64(-43.5)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::F64(-41.5)), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_usize() {
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::USize(42)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_isize() {
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::ISize(-43)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::ISize(-42)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::ISize(-41)), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_bool() {
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::Bool(true), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Err(())));
    }

    #[test]
    fn test_compare_ne_str() {
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::from("i")), Some(Ok(())));
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::from("j")), Some(Err(())));
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::from("k")), Some(Ok(())));
    }

    #[test]
    fn test_compare_ne_u64_other_types() {
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ne(&OperandValue::U64(42), &OperandValue::from("j")), None);

    }

    #[test]
    fn test_compare_ne_i64_other_types() {
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ne(&OperandValue::I64(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_ne_f64_other_types() {
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ne(&OperandValue::F64(-42.5), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_ne_usize_other_types() {
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ne(&OperandValue::USize(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_ne_isize_other_types() {
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ne(&OperandValue::ISize(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_ne_bool_other_types() {
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::Bool(false), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_ne_str_other_types() {
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::U64(42)), None);
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::USize(42)), None);
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ne(&OperandValue::from("j"), &OperandValue::Bool(false)), None);
    }
}
