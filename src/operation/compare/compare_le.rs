use std::cmp::Ordering;

use super::OperandValue;

pub fn compare_le(value_a: &OperandValue, value_b: &OperandValue) -> Option<Result<(), ()>> {
    match value_a.partial_cmp(value_b)? {
        Ordering::Less | Ordering::Equal => Some(Ok(())),
        Ordering::Greater => Some(Err(())),
    }
}

#[cfg(test)]
mod test {
    use super::super::OperandValue;

    use super::compare_le;

    #[test]
    fn test_compare_le_u64() {
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::U64(41)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_i64() {
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::I64(-43)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::I64(-42)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::I64(-41)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_f64() {
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::F64(-43.5)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::F64(-41.5)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_usize() {
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::USize(41)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_isize() {
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::ISize(-43)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::ISize(-42)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::ISize(-41)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_bool() {
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::Bool(true), &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_str() {
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::from("i")), Some(Err(())));
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::from("j")), Some(Ok(())));
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::from("k")), Some(Ok(())));
    }

    #[test]
    fn test_compare_le_u64_other_types() {
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_le(&OperandValue::U64(42), &OperandValue::from("j")), None);

    }

    #[test]
    fn test_compare_le_i64_other_types() {
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_le(&OperandValue::I64(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_le_f64_other_types() {
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::Bool(false)), None);
        assert_eq!(compare_le(&OperandValue::F64(-42.5), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_le_usize_other_types() {
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_le(&OperandValue::USize(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_le_isize_other_types() {
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_le(&OperandValue::ISize(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_le_bool_other_types() {
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::Bool(false), &OperandValue::from("j")), None);
    }

    #[test]
    fn test_compare_le_str_other_types() {
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::U64(42)), None);
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::I64(-42)), None);
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::USize(42)), None);
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_le(&OperandValue::from("j"), &OperandValue::Bool(false)), None);
    }
}
