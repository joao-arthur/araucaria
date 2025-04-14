#[derive(Debug, PartialEq, Clone)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    Bool(bool),
    Str(String),
}

impl PartialOrd for OperandValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let OperandValue::U64(a) = self {
            if let OperandValue::U64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::I64(a) = self {
            if let OperandValue::I64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::F64(a) = self {
            if let OperandValue::F64(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::USize(a) = self {
            if let OperandValue::USize(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Bool(a) = self {
            if let OperandValue::Bool(b) = other {
                return a.partial_cmp(b);
            }
        }
        if let OperandValue::Str(a) = self {
            if let OperandValue::Str(b) = other {
                return a.partial_cmp(b);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Value(OperandValue),
    FieldPath(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationEq {
    Eq(Operand),
    Ne(Operand),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Eq(Operand),
    Ne(Operand),
    Gt(Operand),
    Ge(Operand),
    Lt(Operand),
    Le(Operand),
    Btwn(Operand, Operand),
}

fn compare(operation: &Operation, value_a: &OperandValue) -> Option<Result<(), Operation>> {
    match operation {
        Operation::Eq(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
                OperandValue::USize(b) => match value_a {
                    OperandValue::USize(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
                OperandValue::Bool(b) => match value_a {
                    OperandValue::Bool(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
                OperandValue::Str(b) => match value_a {
                    OperandValue::Str(a) => {
                        if a == b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                    _ => None,
                },
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Ne(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::USize(b) => match value_a {
                    OperandValue::USize(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Str(b) => match value_a {
                    OperandValue::Str(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Bool(b) => match value_a {
                    OperandValue::Bool(a) => {
                        if a != b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Gt(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a > b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a > b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a > b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::USize(b) =>  match value_a {
                    OperandValue::USize(a) => {
                        if a > b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Str(b) =>match value_a {
                    OperandValue::Str(a) => {
                        if a > b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Bool(b) => None,
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Ge(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a >= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a >= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a >= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::USize(b) =>  match value_a {
                    OperandValue::USize(a) => {
                        if a >= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Str(b) => match value_a {
                    OperandValue::Str(a) => {
                        if a >= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Bool(b) => None,
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Lt(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a < b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a < b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a < b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::USize(b) => match value_a {
                    OperandValue::USize(a) => {
                        if a < b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Str(b) => match value_a {
                    OperandValue::Str(a) => {
                        if a < b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Bool(b) => None,
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Le(operand) => match operand {
            Operand::Value(value_b) => match value_b {
                OperandValue::U64(b) => match value_a {
                    OperandValue::U64(a) => {
                        if a <= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::I64(b) => match value_a {
                    OperandValue::I64(a) => {
                        if a <= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::F64(b) => match value_a {
                    OperandValue::F64(a) => {
                        if a <= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::USize(b) =>  match value_a {
                    OperandValue::USize(a) => {
                        if a <= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Str(b) => match value_a {
                    OperandValue::Str(a) => {
                        if a <= b {
                            Some(Ok(()))
                        } else {
                            Some(Err(operation.clone()))
                        }
                    }
                   _ => None,
                },
                OperandValue::Bool(b) => None,
            },
            Operand::FieldPath(path_b) => Some(Ok(())),
        },
        Operation::Btwn(operand_a, operand_b) => Some(Ok(())),
    }
}

#[cfg(test)]
mod test {
    use super::{compare, Operand, OperandValue, Operation};

    #[test]
    fn test_operand_value_eq_true() {
        assert!(OperandValue::U64(42) == OperandValue::U64(42));
        assert!(OperandValue::I64(-42) == OperandValue::I64(-42));
        assert!(OperandValue::F64(-4.2) == OperandValue::F64(-4.2));
        assert!(OperandValue::Bool(false) == OperandValue::Bool(false));
        assert!(OperandValue::Str(String::from("a b c")) == OperandValue::Str(String::from("a b c")));
    }

    #[test]
    fn test_operand_value_eq_false() {
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(24), false);
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-24), false);
        assert_eq!(OperandValue::F64(-4.2) == OperandValue::F64(-2.4), false);
        assert_eq!(OperandValue::Bool(false) == OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Str(String::from("a b c")) == OperandValue::Str(String::from("z y w")), false);
    }









    #[test]
    fn test_compare_u64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::U64(42)));   
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_u64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_u64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_i64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::I64(42)));   
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_i64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_i64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_f64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::F64(42.0)));   
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_f64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_f64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_usize_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::USize(42)));   
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_usize_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_usize_le() {
        let v = Operation::Le(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_string_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::Str(String::from("j"))));   
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_string_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(v.clone())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_string_le() {
        let v = Operation::Le(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(v.clone())));
    }

    #[test]
    fn test_compare_u64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::U64(42))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::U64(42))), &OperandValue::Bool(false)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::U64(42))), &OperandValue::Str(String::from("abc"))), None);
        //assert_eq!(compare(&Operation::Btwn(Operand::Value(OperandValue::U64(24)), Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41)), None);
    }

    //#[test]
    //fn test_compare_u64_btwn() {
    //    let v = Operation::Btwn(Operand::Value(OperandValue::U64(24)), Operand::Value(OperandValue::U64(42)));
    //    assert_eq!(compare(&v, &OperandValue::U64(23)), Err(v.clone()));
    //    assert_eq!(compare(&v, &OperandValue::U64(24)), Ok(()));
    //    assert_eq!(compare(&v, &OperandValue::U64(25)), Ok(()));
    //    assert_eq!(compare(&v, &OperandValue::U64(41)), Ok(()));
    //    assert_eq!(compare(&v, &OperandValue::U64(42)), Ok(()));
    //    assert_eq!(compare(&v, &OperandValue::U64(43)), Err(v.clone()));
    //}

    //    #[test]
    //    fn test_compare_bool_ok() {
    //        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(true))), &OperandValue::Bool(true)), Ok(()));
    //        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &OperandValue::Bool(false)), Ok(()));
    //    }

    //    #[test]
    //    fn test_compare_bool_some() {
    //        let value = OperandValue::Bool(true);
    //        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(false))), &value), Err(Operand::Bool(false)));
    //        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &value), Err(Operand::Bool(true)));
    //    }
    //
    //    #[test]
    //    fn test_compare_bool_other_types() {
    //        assert_eq!(compare(&num_u_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&str_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(Operand::Value(OperandValue::Bool(true)))), Err(Operand::Bool(true)));
    //
    //        assert_eq!(compare(&num_u_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&str_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(Operand::Value(OperandValue::Bool(true))), Err(Operand::Bool(true)));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_u_none() {
    //        let value = OperandValue::U64(42);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::U64(42))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::U64(109))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::U64(41))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::U64(43))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::U64(42))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::U64(42))), Ok(()));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_u_some() {
    //        let value = OperandValue::U64(42);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::U64(22))), Err(Operand::U64(22))));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::U64(42))), Err(Operand::U64(42))));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::U64(42))), Err(Operand::U64(42))));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::U64(42))), Err(Operand::U64(42))));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::U64(43))), Err(Operand::U64(43))));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::U64(41))), Err(Operand::U64(41))));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_u_other_types() {
    //        assert_eq!(compare(&bool_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(OperandValue::U64(5))), Err(Operand::U64(5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(OperandValue::U64(5))), Err(Operand::U64(5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(OperandValue::U64(5))), Err(Operand::U64(5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(OperandValue::U64(5))), Err(Operand::U64(5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Le(OperandValue::U64(5))), Err(Operand::U64(5))));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_i_none() {
    //        let value = OperandValue::I64(-42);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::I64(-42))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::I64(-109))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::I64(-43))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::I64(-41))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::I64(-42))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::I64(-42))), Ok(()));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_i_some() {
    //        let value = OperandValue::I64(-42);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::I64(-22))), Err(Operand::I64(-22))));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::I64(-42))), Err(Operand::I64(-42))));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::I64(-42))), Err(Operand::I64(-42))));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::I64(-42))), Err(Operand::I64(-42))));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::I64(-41))), Err(Operand::I64(-41))));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::I64(-43))), Err(Operand::I64(-43))));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_i_other_types() {
    //        assert_eq!(compare(&bool_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&num_f_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Le(OperandValue::I64(-5))), Err(Operand::I64(-5))));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_f_none() {
    //        let value = OperandValue::F64(-42.5);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::F64(-42.5))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::F64(-10.5))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::F64(-43.5))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::F64(-41.5))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::F64(-42.5))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::F64(-42.5))), Ok(()));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_f_some() {
    //        let value = OperandValue::F64(-42.5);
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::F64(-22.5))), Err(Operand::F64(-22.5))));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::F64(-42.5))), Err(Operand::F64(-42.5))));
    //        assert_eq!(compare(&value, &Operation::Gt(OperandValue::F64(-42.5))), Err(Operand::F64(-42.5))));
    //        assert_eq!(compare(&value, &Operation::Lt(OperandValue::F64(-42.5))), Err(Operand::F64(-42.5))));
    //        assert_eq!(compare(&value, &Operation::Ge(OperandValue::F64(-41.5))), Err(Operand::F64(-41.5))));
    //        assert_eq!(compare(&value, &Operation::Le(OperandValue::F64(-43.5))), Err(Operand::F64(-43.5))));
    //    }
    //
    //    #[test]
    //    fn test_compare_num_f_other_types() {
    //        assert_eq!(compare(&bool_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Eq(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ne(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Gt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Lt(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Ge(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //
    //        assert_eq!(compare(&bool_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_u_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&num_i_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&str_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_bool_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //        assert_eq!(compare(&arr_str_stub(), &Operation::Le(OperandValue::F64(-5.5))), Err(Operand::F64(-5.5))));
    //    }
    //
    //    #[test]
    //    fn test_compare_str_none() {
    //        let value = OperandValue::Str(String::from("Belisarius"));
    //        assert_eq!(compare(&value, &Operation::Eq(OperandValue::Str(String::from("Belisarius")))), Ok(()));
    //        assert_eq!(compare(&value, &Operation::Ne(OperandValue::Str(String::from("Iustinianus")))), Ok(()));
    //    }
    //
    //    #[test]
    //    fn test_compare_str_some() {
    //        let value = OperandValue::Str(String::from("Belisarius"));
    //        assert_eq!(
    //            compare(&value, &Operation::Eq(OperandValue::Str(String::from("Iustinianus")))),
    //            Err(Operand::Str(String::from("Iustinianus"))))
    //        );
    //        assert_eq!(
    //            compare(&value, &Operation::Ne(OperandValue::Str(String::from("Belisarius")))),
    //            Err(Operand::Str(String::from("Belisarius"))))
    //        );
    //    }
    //
    //    #[test]
    //    fn test_compare_str_other_types() {
    //        assert_eq!(
    //            compare(&bool_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_u_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_i_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_f_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&arr_bool_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&arr_str_stub(), &Operation::Eq(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //
    //        assert_eq!(
    //            compare(&bool_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_u_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_i_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&num_f_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&arr_bool_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //        assert_eq!(
    //            compare(&arr_str_stub(), &Operation::Ne(OperandValue::Str(String::from("Lemouria")))),
    //            Err(Operand::Str(String::from("Lemouria"))))
    //        );
    //    }
}

/*

pub fn compare_eq<T>(operation: &OperationEq, value: &T) -> Result<(), OperationEq>
where
    T: PartialEq + Clone,
{
    match operation {
        OperationEq::Eq(v) => {
            if value != v {
                return Err(OperationEq::Eq(v.clone()));
            }
        }
        OperationEq::Ne(v) => {
            if value == v {
                return Err(OperationEq::Ne(v.clone()));
            }
        }
    }
    SomSome(e(Ok(()))
}

pub fn co)mpare<T>(operation: &Operation, value: &T) -> Result<(), Operation>
where
    T: PartialOrd + Clone,
{
    match operation {
        Operation::Eq(v) => {
            if value != v {
                return Err(Operation::Eq(v.clone()));
            }
        }
        Operation::Ne(v) => {
            if value == v {
                return Err(Operation::Ne(v.clone()));
            }
        }
        Operation::Gt(v) => {
            if value <= v {
                return Err(Operation::Gt(v.clone()));
            }
        }
        Operation::Ge(v) => {
            if value < v {
                return Err(Operation::Ge(v.clone()));
            }
        }
        Operation::Lt(v) => {
            if value >= v {
                return Err(Operation::Lt(v.clone()));
            }
        }
        Operation::Le(v) => {
            if value > v {
                return Err(Operation::Le(v.clone()));
            }
        }
        Operation::Btwn(v_a, v_b) => {
            if value < v_a || value > v_b {
                return Err(Operation::Btwn(v_a.clone(), v_b.clone()));
            }
        }
    }
    SomSome(e(Ok(()))
}

#[cfg(tes)t)]
mod test {
    use crate::operation::compare;

    use super::{compare_eq, Operation, OperationEq};

    #[test]
    fn test_compare_eq_boolean_ok() {
        assert_eq!(compare_eq(&OperationEq::Eq(false), &false), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Eq(true), &true), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(false), &true), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(true), &false), Ok(()));
    }

    #[test]
    fn test_compare_eq_boolean_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(false), &true), Err(OperationEq::Eq(false)));
        assert_eq!(compare_eq(&OperationEq::Eq(true), &false), Err(OperationEq::Eq(true)));
        assert_eq!(compare_eq(&OperationEq::Ne(false), &false), Err(OperationEq::Ne(false)));
        assert_eq!(compare_eq(&OperationEq::Ne(true), &true), Err(OperationEq::Ne(true)));
    }

    #[test]
    fn test_compare_eq_str_ok() {
        assert_eq!(compare_eq(&OperationEq::Eq(String::from("swords")), &String::from("swords")), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(String::from("swords")), &String::from("sandals")), Ok(()));
    }

    #[test]
    fn test_compare_eq_str_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(String::from("swords")), &String::from("sandals")), Err(OperationEq::Eq(String::from("swords"))));
        assert_eq!(compare_eq(&OperationEq::Ne(String::from("swords")), &String::from("swords")), Err(OperationEq::Ne(String::from("swords"))));
    }

    #[test]
    fn test_compare_eq_u64_ok() {
        let v: u64 = 10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &42), Ok(()));
    }

    #[test]
    fn test_compare_eq_u64_err() {
        let v: u64 = 10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &8), Err(OperationEq::Eq(v)));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &10), Err(OperationEq::Ne(v)));
    }

    #[test]
    fn test_compare_eq_i64_ok() {
        let v: i64 = -10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &-10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &-42), Ok(()));
    }

    #[test]
    fn test_compare_eq_i64_err() {
        let v: i64 = -10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &8), Err(OperationEq::Eq(v)));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &-10), Err(OperationEq::Ne(v)));
    }

    #[test]
    fn test_compare_eq_f64_ok() {
        let v: f64 = -10.5;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &-10.5), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &-42.5), Ok(()));
    }

    #[test]
    fn test_compare_eq_f64_err() {
        let v: f64 = -10.5;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &-4.25), Err(OperationEq::Eq(v)));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &-10.5), Err(OperationEq::Ne(v)));
    }

    #[test]
    fn test_compare_eq_usize_ok() {
        let v: usize = 10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &42), Ok(()));
    }

    #[test]
    fn test_compare_eq_usize_err() {
        let v: usize = 10;
        assert_eq!(compare_eq(&OperationEq::Eq(v), &8), Err(OperationEq::Eq(v)));
        assert_eq!(compare_eq(&OperationEq::Ne(v), &10), Err(OperationEq::Ne(v)));
    }

    #[test]
    fn test_compare_boolean_ok() {
        assert_eq!(compare(&Operation::Eq(false), &false), Ok(()));
        assert_eq!(compare(&Operation::Eq(true), &true), Ok(()));
        assert_eq!(compare(&Operation::Ne(false), &true), Ok(()));
        assert_eq!(compare(&Operation::Ne(true), &false), Ok(()));
    }

    #[test]
    fn test_compare_boolean_err() {
        assert_eq!(compare(&Operation::Eq(false), &true), Err(Operation::Eq(false)));
        assert_eq!(compare(&Operation::Eq(true), &false), Err(Operation::Eq(true)));
        assert_eq!(compare(&Operation::Ne(false), &false), Err(Operation::Ne(false)));
        assert_eq!(compare(&Operation::Ne(true), &true), Err(Operation::Ne(true)));
    }

    #[test]
    fn test_compare_str_ok() {
        assert_eq!(compare(&Operation::Eq(String::from("swords")), &String::from("swords")), Ok(()));
        assert_eq!(compare(&Operation::Ne(String::from("swords")), &String::from("sandals")), Ok(()));
    }

    #[test]
    fn test_compare_str_err() {
        assert_eq!(compare(&Operation::Eq(String::from("swords")), &String::from("sandals")), Err(Operation::Eq(String::from("swords"))));
        assert_eq!(compare(&Operation::Ne(String::from("swords")), &String::from("swords")), Err(Operation::Ne(String::from("swords"))));
    }

    #[test]
    fn test_compare_u64_eq() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Eq(v), &9), Err(Operation::Eq(v)));
        assert_eq!(compare(&Operation::Eq(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Eq(v), &11), Err(Operation::Eq(v)));
    }

    #[test]
    fn test_compare_u64_ne() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Ne(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Ne(v), &10), Err(Operation::Ne(v)));
        assert_eq!(compare(&Operation::Ne(v), &11), Ok(()));
    }

    #[test]
    fn test_compare_u64_gt() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Gt(v), &9), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &10), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &11), Ok(()));
        assert_eq!(compare(&Operation::Gt(v), &12), Ok(()));
    }

    #[test]
    fn test_compare_u64_ge() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Ge(v), &8), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &9), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Ge(v), &11), Ok(()));
    }

    #[test]
    fn test_compare_u64_lt() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Lt(v), &8), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &10), Err(Operation::Lt(v)));
        assert_eq!(compare(&Operation::Lt(v), &11), Err(Operation::Lt(v)));
    }

    #[test]
    fn test_compare_u64_le() {
        let v: u64 = 10;
        assert_eq!(compare(&Operation::Le(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &11), Err(Operation::Le(v)));
        assert_eq!(compare(&Operation::Le(v), &12), Err(Operation::Le(v)));
    }

    #[test]
    fn test_compare_u64_btwn() {
        let a: u64 = 10;
        let b: u64 = 100;
        assert_eq!(compare(&Operation::Btwn(a, b), &9), Err(Operation::Btwn(a, b)));
        assert_eq!(compare(&Operation::Btwn(a, b), &10), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &100), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &101), Err(Operation::Btwn(a, b)));
    }

    #[test]
    fn test_compare_i64_eq() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Eq(v), &-11), Err(Operation::Eq(v)));
        assert_eq!(compare(&Operation::Eq(v), &-10), Ok(()));
        assert_eq!(compare(&Operation::Eq(v), &-9), Err(Operation::Eq(v)));
    }

    #[test]
    fn test_compare_i64_ne() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Ne(v), &-11), Ok(()));
        assert_eq!(compare(&Operation::Ne(v), &-10), Err(Operation::Ne(v)));
        assert_eq!(compare(&Operation::Ne(v), &-9), Ok(()));
    }

    #[test]
    fn test_compare_i64_gt() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Gt(v), &-11), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &-10), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &-9), Ok(()));
        assert_eq!(compare(&Operation::Gt(v), &-8), Ok(()));
    }

    #[test]
    fn test_compare_i64_ge() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Ge(v), &-12), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &-11), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &-10), Ok(()));
        assert_eq!(compare(&Operation::Ge(v), &-9), Ok(()));
    }

    #[test]
    fn test_compare_i64_lt() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Lt(v), &-12), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &-11), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &-10), Err(Operation::Lt(v)));
        assert_eq!(compare(&Operation::Lt(v), &-9), Err(Operation::Lt(v)));
    }

    #[test]
    fn test_compare_i64_le() {
        let v: i64 = -10;
        assert_eq!(compare(&Operation::Le(v), &-11), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &-10), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &-9), Err(Operation::Le(v)));
        assert_eq!(compare(&Operation::Le(v), &-8), Err(Operation::Le(v)));
    }

    #[test]
    fn test_compare_i64_btwn() {
        let a: i64 = -10;
        let b: i64 = 10;
        assert_eq!(compare(&Operation::Btwn(a, b), &-11), Err(Operation::Btwn(a, b)));
        assert_eq!(compare(&Operation::Btwn(a, b), &-10), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &10), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &11), Err(Operation::Btwn(a, b)));
    }

    #[test]
    fn test_compare_f64_eq() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Eq(v), &-11.5), Err(Operation::Eq(v)));
        assert_eq!(compare(&Operation::Eq(v), &-10.5), Ok(()));
        assert_eq!(compare(&Operation::Eq(v), &-9.5), Err(Operation::Eq(v)));
    }

    #[test]
    fn test_compare_f64_ne() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Ne(v), &-11.5), Ok(()));
        assert_eq!(compare(&Operation::Ne(v), &-10.5), Err(Operation::Ne(v)));
        assert_eq!(compare(&Operation::Ne(v), &-9.5), Ok(()));
    }

    #[test]
    fn test_compare_f64_gt() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Gt(v), &-11.5), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &-10.5), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &-9.5), Ok(()));
        assert_eq!(compare(&Operation::Gt(v), &-8.5), Ok(()));
    }

    #[test]
    fn test_compare_f64_ge() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Ge(v), &-12.5), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &-11.5), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &-10.5), Ok(()));
        assert_eq!(compare(&Operation::Ge(v), &-9.5), Ok(()));
    }

    #[test]
    fn test_compare_f64_lt() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Lt(v), &-12.5), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &-11.5), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &-10.5), Err(Operation::Lt(v)));
        assert_eq!(compare(&Operation::Lt(v), &-9.5), Err(Operation::Lt(v)));
    }

    #[test]
    fn test_compare_f64_le() {
        let v: f64 = -10.5;
        assert_eq!(compare(&Operation::Le(v), &-11.5), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &-10.5), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &-9.5), Err(Operation::Le(v)));
        assert_eq!(compare(&Operation::Le(v), &-8.5), Err(Operation::Le(v)));
    }

    #[test]
    fn test_compare_f64_btwn() {
        let a: f64 = -10.5;
        let b: f64 = 10.5;
        assert_eq!(compare(&Operation::Btwn(a, b), &-11.5), Err(Operation::Btwn(a, b)));
        assert_eq!(compare(&Operation::Btwn(a, b), &-10.5), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &10.5), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &11.5), Err(Operation::Btwn(a, b)));
    }

    #[test]
    fn test_compare_usize_eq() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Eq(v), &9), Err(Operation::Eq(v)));
        assert_eq!(compare(&Operation::Eq(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Eq(v), &11), Err(Operation::Eq(v)));
    }

    #[test]
    fn test_compare_usize_ne() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Ne(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Ne(v), &10), Err(Operation::Ne(v)));
        assert_eq!(compare(&Operation::Ne(v), &11), Ok(()));
    }

    #[test]
    fn test_compare_usize_gt() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Gt(v), &9), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &10), Err(Operation::Gt(v)));
        assert_eq!(compare(&Operation::Gt(v), &11), Ok(()));
        assert_eq!(compare(&Operation::Gt(v), &12), Ok(()));
    }

    #[test]
    fn test_compare_usize_ge() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Ge(v), &8), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &9), Err(Operation::Ge(v)));
        assert_eq!(compare(&Operation::Ge(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Ge(v), &11), Ok(()));
    }

    #[test]
    fn test_compare_usize_lt() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Lt(v), &8), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Lt(v), &10), Err(Operation::Lt(v)));
        assert_eq!(compare(&Operation::Lt(v), &11), Err(Operation::Lt(v)));
    }

    #[test]
    fn test_compare_usize_le() {
        let v: usize = 10;
        assert_eq!(compare(&Operation::Le(v), &9), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &10), Ok(()));
        assert_eq!(compare(&Operation::Le(v), &11), Err(Operation::Le(v)));
        assert_eq!(compare(&Operation::Le(v), &12), Err(Operation::Le(v)));
    }

    #[test]
    fn test_compare_usize_btwn() {
        let a: usize = 10;
        let b: usize = 100;
        assert_eq!(compare(&Operation::Btwn(a, b), &9), Err(Operation::Btwn(a, b)));
        assert_eq!(compare(&Operation::Btwn(a, b), &10), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &100), Ok(()));
        assert_eq!(compare(&Operation::Btwn(a, b), &101), Err(Operation::Btwn(a, b)));
    }
}
*/
