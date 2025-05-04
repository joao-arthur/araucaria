use super::{OperandValue, compare_ge, compare_le};

pub fn compare_btwn(value: &OperandValue, operand_a: &OperandValue, operand_b: &OperandValue) -> Option<Result<(), ()>> {
    if let Ok(()) = compare_ge(value, operand_a)? {
        if let Ok(()) = compare_le(value, operand_b)? {
            return Some(Ok(()));
        }
    }
    Some(Err(()))
}

#[cfg(test)]
mod tests {
    use super::super::OperandValue;

    use super::compare_btwn;

    #[test]
    fn compare_btwn_u64() {
        assert_eq!(compare_btwn(&OperandValue::U64(40), &OperandValue::U64(41), &OperandValue::U64(43)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::U64(41), &OperandValue::U64(41), &OperandValue::U64(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(41), &OperandValue::U64(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::U64(43), &OperandValue::U64(41), &OperandValue::U64(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::U64(44), &OperandValue::U64(41), &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn compare_btwn_i64() {
        assert_eq!(compare_btwn(&OperandValue::I64(-44), &OperandValue::I64(-43), &OperandValue::I64(-41)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::I64(-43), &OperandValue::I64(-43), &OperandValue::I64(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-43), &OperandValue::I64(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::I64(-41), &OperandValue::I64(-43), &OperandValue::I64(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::I64(-40), &OperandValue::I64(-43), &OperandValue::I64(-41)), Some(Err(())));
    }

    #[test]
    fn compare_btwn_f64() {
        assert_eq!(compare_btwn(&OperandValue::F64(-44.5), &OperandValue::F64(-43.5), &OperandValue::F64(-41.5)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::F64(-43.5), &OperandValue::F64(-43.5), &OperandValue::F64(-41.5)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-43.5), &OperandValue::F64(-41.5)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::F64(-41.5), &OperandValue::F64(-43.5), &OperandValue::F64(-41.5)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::F64(-40.5), &OperandValue::F64(-43.5), &OperandValue::F64(-41.5)), Some(Err(())));
    }

    #[test]
    fn compare_btwn_usize() {
        assert_eq!(compare_btwn(&OperandValue::USize(40), &OperandValue::USize(41), &OperandValue::USize(43)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::USize(41), &OperandValue::USize(41), &OperandValue::USize(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(41), &OperandValue::USize(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::USize(43), &OperandValue::USize(41), &OperandValue::USize(43)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::USize(44), &OperandValue::USize(41), &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn compare_btwn_isize() {
        assert_eq!(compare_btwn(&OperandValue::ISize(-44), &OperandValue::ISize(-43), &OperandValue::ISize(-41)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::ISize(-43), &OperandValue::ISize(-43), &OperandValue::ISize(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-43), &OperandValue::ISize(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::ISize(-41), &OperandValue::ISize(-43), &OperandValue::ISize(-41)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::ISize(-40), &OperandValue::ISize(-43), &OperandValue::ISize(-41)), Some(Err(())));
    }

    #[test]
    fn compare_btwn_bool() {
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::Bool(true), &OperandValue::Bool(false), &OperandValue::Bool(true)), Some(Ok(())));

        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::Bool(true), &OperandValue::Bool(false), &OperandValue::Bool(false)), Some(Err(())));

        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::Bool(true), &OperandValue::Bool(true), &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn compare_btwn_str() {
        assert_eq!(compare_btwn(&OperandValue::from("h"), &OperandValue::from("i"), &OperandValue::from("k")), Some(Err(())));
        assert_eq!(compare_btwn(&OperandValue::from("i"), &OperandValue::from("i"), &OperandValue::from("k")), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("i"), &OperandValue::from("k")), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::from("k"), &OperandValue::from("i"), &OperandValue::from("k")), Some(Ok(())));
        assert_eq!(compare_btwn(&OperandValue::from("l"), &OperandValue::from("i"), &OperandValue::from("k")), Some(Err(())));
    }

    #[test]
    fn compare_btwn_u64_other_types() {
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_i64_other_types() {
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_f64_other_types() {
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_usize_other_types() {
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_isize_other_types() {
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_bool_other_types() {
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::from("j"), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_str_other_types() {
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::U64(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::I64(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::F64(-42.5), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::USize(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::ISize(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::Bool(false), &OperandValue::Bool(false)), None);
    }

    #[test]
    fn compare_btwn_u64_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::U64(42), &OperandValue::U64(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_i64_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::I64(-42), &OperandValue::I64(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_f64_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::F64(-42.5), &OperandValue::F64(-42.5), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_usize_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::USize(42), &OperandValue::USize(42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_isize_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::Bool(false)), None);
        assert_eq!(compare_btwn(&OperandValue::ISize(-42), &OperandValue::ISize(-42), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_bool_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::Bool(false), &OperandValue::Bool(false), &OperandValue::from("j")), None);
    }

    #[test]
    fn compare_btwn_str_operand_b_other_type() {
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::U64(42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::I64(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::F64(-42.5)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::USize(42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::ISize(-42)), None);
        assert_eq!(compare_btwn(&OperandValue::from("j"), &OperandValue::from("j"), &OperandValue::Bool(false)), None);
    }
}
