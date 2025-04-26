use std::cmp::Ordering;

use super::OperandValue;

pub fn compare_ge(value: &OperandValue, operand: &OperandValue) -> Option<Result<(), ()>> {
    match value.partial_cmp(operand)? {
        Ordering::Less => Some(Err(())),
        Ordering::Equal | Ordering::Greater => Some(Ok(())),
    }
}

#[cfg(test)]
mod tests {
    use super::super::OperandValue;

    use super::compare_ge;

    #[test]
    fn compare_ge_u64() {
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn compare_ge_i64() {
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::I64(-43)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::I64(-42)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::I64(-41)), Some(Err(())));
    }

    #[test]
    fn compare_ge_f64() {
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::F64(-43.5)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::F64(-41.5)), Some(Err(())));
    }

    #[test]
    fn compare_ge_usize() {
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn compare_ge_isize() {
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::ISize(-43)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::ISize(-42)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::ISize(-41)), Some(Err(())));
    }

    #[test]
    fn compare_ge_bool() {
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Err(())));
        assert_eq!(compare_ge(&OperandValue::Bool(true), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn compare_ge_str() {
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::from("i")), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::from("j")), Some(Ok(())));
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::from("k")), Some(Err(())));
    }

    #[test]
    fn compare_ge_u64_other_types() {
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ge(&OperandValue::U64(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_i64_other_types() {
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ge(&OperandValue::I64(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_f64_other_types() {
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ge(&OperandValue::F64(-42.5), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_usize_other_types() {
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ge(&OperandValue::USize(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_isize_other_types() {
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_ge(&OperandValue::ISize(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_bool_other_types() {
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::Bool(false), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_ge_str_other_types() {
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::U64(42)), None);
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::I64(-42)), None);
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::USize(42)), None);
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_ge(&OperandValue::from("j"), &OperandValue::Bool(false)), None);
    }
}
