use std::cmp::Ordering;

use super::OperandValue;

pub fn compare_eq(value: &OperandValue, operand: &OperandValue) -> Option<Result<(), ()>> {
    match value.partial_cmp(operand)? {
        Ordering::Less | Ordering::Greater => Some(Err(())),
        Ordering::Equal => Some(Ok(())),
    }
}

#[cfg(test)]
mod tests {
    use super::super::OperandValue;

    use super::compare_eq;

    #[test]
    fn compare_eq_u64() {
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::U64(41)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn compare_eq_i64() {
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::I64(-43)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::I64(-42)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::I64(-41)), Some(Err(())));
    }

    #[test]
    fn compare_eq_f64() {
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::F64(-43.5)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::F64(-41.5)), Some(Err(())));
    }

    #[test]
    fn compare_eq_usize() {
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::USize(41)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn compare_eq_isize() {
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::ISize(-43)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::ISize(-42)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::ISize(-41)), Some(Err(())));
    }

    #[test]
    fn compare_eq_bool() {
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::Bool(true), &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn compare_eq_str() {
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::from("i")), Some(Err(())));
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::from("j")), Some(Ok(())));
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::from("k")), Some(Err(())));
    }

    #[test]
    fn compare_eq_u64_other_types() {
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_eq(&OperandValue::U64(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_i64_other_types() {
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_eq(&OperandValue::I64(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_f64_other_types() {
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::Bool(false)), None);
        assert_eq!(compare_eq(&OperandValue::F64(-42.5), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_usize_other_types() {
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_eq(&OperandValue::USize(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_isize_other_types() {
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_eq(&OperandValue::ISize(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_bool_other_types() {
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::Bool(false), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_eq_str_other_types() {
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::U64(42)), None);
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::I64(-42)), None);
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::USize(42)), None);
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_eq(&OperandValue::from("j"), &OperandValue::Bool(false)), None);
    }
}
