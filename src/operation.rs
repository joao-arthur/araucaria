use std::cmp::Ordering;

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
pub enum Operation {
    Eq(Operand),
    Ne(Operand),
    Gt(Operand),
    Ge(Operand),
    Lt(Operand),
    Le(Operand),
    Btwn(Operand, Operand),
}

pub fn compare(operation: &Operation, value_a: &OperandValue) -> Option<Result<(), ()>> {
    match operation {
        Operation::Eq(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less => Some(Err(())),
                    Ordering::Equal => Some(Ok(())),
                    Ordering::Greater => Some(Err(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Ne(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less | Ordering::Greater => Some(Ok(())),
                    Ordering::Equal => Some(Err(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Gt(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less | Ordering::Equal => Some(Err(())),
                    Ordering::Greater => Some(Ok(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Ge(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less => Some(Err(())),
                    Ordering::Equal | Ordering::Greater => Some(Ok(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Lt(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less => Some(Ok(())),
                    Ordering::Equal | Ordering::Greater => Some(Err(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Le(operand) => match operand {
            Operand::Value(value_b) => match value_a.partial_cmp(value_b) {
                Some(ord) => match ord {
                    Ordering::Less | Ordering::Equal => Some(Ok(())),
                    Ordering::Greater => Some(Err(())),
                },
                None => None,
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
        Operation::Btwn(operand_a, operand_b) => match operand_a {
            Operand::Value(value_operand_a) => match operand_b {
                Operand::Value(value_operand_b) => {
                    let result_a = value_a.partial_cmp(value_operand_a);
                    let result_b = value_a.partial_cmp(value_operand_b);
                    match result_a {
                        Some(ord) => match ord {
                            Ordering::Less => Some(Err(())),
                            Ordering::Equal | Ordering::Greater => match result_b {
                                Some(ord) => match ord {
                                    Ordering::Less | Ordering::Equal => Some(Ok(())),
                                    Ordering::Greater => Some(Err(())),
                                },
                                None => None,
                            },
                        },
                        None => None,
                    }
                }
                Operand::FieldPath(_) => Some(Ok(())),
            },
            Operand::FieldPath(_) => Some(Ok(())),
        },
    }
}

#[cfg(test)]
mod test {
    use super::{compare, Operand, OperandValue, Operation};

    #[test]
    fn test_operand_value_u64() {
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) == OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) != OperandValue::U64(43), true);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) > OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(41), true);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) >= OperandValue::U64(43), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(42), false);
        assert_eq!(OperandValue::U64(42) < OperandValue::U64(43), true);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(41), false);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(42), true);
        assert_eq!(OperandValue::U64(42) <= OperandValue::U64(43), true);
    }

    #[test]
    fn test_operand_value_i64() {
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) == OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) != OperandValue::I64(-41), true);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) > OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-43), true);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) >= OperandValue::I64(-41), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-42), false);
        assert_eq!(OperandValue::I64(-42) < OperandValue::I64(-41), true);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-43), false);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-42), true);
        assert_eq!(OperandValue::I64(-42) <= OperandValue::I64(-41), true);
    }

    #[test]
    fn test_operand_value_f64() {
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) == OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) != OperandValue::F64(41.0), true);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) > OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(-43.0), true);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) >= OperandValue::F64(41.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(-42.0), false);
        assert_eq!(OperandValue::F64(-42.0) < OperandValue::F64(41.0), true);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-43.0), false);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(-42.0), true);
        assert_eq!(OperandValue::F64(-42.0) <= OperandValue::F64(41.0), true);
    }

    #[test]
    fn test_operand_value_usize() {
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) == OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) != OperandValue::USize(43), true);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) > OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(41), true);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) >= OperandValue::USize(43), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(42), false);
        assert_eq!(OperandValue::USize(42) < OperandValue::USize(43), true);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(41), false);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(42), true);
        assert_eq!(OperandValue::USize(42) <= OperandValue::USize(43), true);
    }

    #[test]
    fn test_operand_value_bool() {
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) == OperandValue::Bool(true), true);
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) != OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) > OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) >= OperandValue::Bool(false), true);
        assert_eq!(OperandValue::Bool(true) >= OperandValue::Bool(true), true);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) < OperandValue::Bool(true), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(false), false);
        assert_eq!(OperandValue::Bool(true) <= OperandValue::Bool(true), true);
    }

    #[test]
    fn test_operand_value_string() {
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) == OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) != OperandValue::Str(String::from("k")), true);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) > OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("i")), true);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) >= OperandValue::Str(String::from("k")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("j")), false);
        assert_eq!(OperandValue::Str(String::from("j")) < OperandValue::Str(String::from("k")), true);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("i")), false);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("j")), true);
        assert_eq!(OperandValue::Str(String::from("j")) <= OperandValue::Str(String::from("k")), true);
    }

    #[test]
    fn test_operand_value_different_type() {
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::I64(-42)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::U64(42).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::F64(-4.2)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::I64(-42).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Bool(false)), None);
        assert_eq!(OperandValue::F64(-4.2).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
        assert_eq!(OperandValue::Bool(false).partial_cmp(&OperandValue::Str(String::from("a b c"))), None);
    }

    #[test]
    fn test_compare_u64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_u64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::U64(24)), Operand::Value(OperandValue::U64(42)));
        assert_eq!(compare(&v, &OperandValue::U64(23)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::U64(24)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(25)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::U64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_i64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_i64_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::I64(24)), Operand::Value(OperandValue::I64(42)));
        assert_eq!(compare(&v, &OperandValue::I64(23)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::I64(24)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(25)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::I64(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Ok(())));
    }

    #[test]
    fn test_compare_f64_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_le() {
        let v = Operation::Le(Operand::Value(OperandValue::F64(42.0)));
        assert_eq!(compare(&v, &OperandValue::F64(41.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.0)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.0)), Some(Err(())));
    }

    #[test]
    fn test_compare_f64_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::F64(24.5)), Operand::Value(OperandValue::F64(42.5)));
        assert_eq!(compare(&v, &OperandValue::F64(23.5)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::F64(24.5)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(25.5)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(41.5)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(42.5)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::F64(43.5)), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Ok(())));
    }

    #[test]
    fn test_compare_usize_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_le() {
        let v = Operation::Le(Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_usize_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(42)));
        assert_eq!(compare(&v, &OperandValue::USize(23)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::USize(24)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(25)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(42)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::USize(43)), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::Bool(false)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Err(())));
    }

    #[test]
    fn test_compare_bool_le() {
        let v = Operation::Le(Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_bool_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)));
        assert_eq!(compare(&v, &OperandValue::Bool(false)), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Bool(true)), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_eq() {
        let v = Operation::Eq(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(())));
    }

    #[test]
    fn test_compare_string_ne() {
        let v = Operation::Ne(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_gt() {
        let v = Operation::Gt(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_ge() {
        let v = Operation::Ge(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Ok(())));
    }

    #[test]
    fn test_compare_string_lt() {
        let v = Operation::Lt(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(())));
    }

    #[test]
    fn test_compare_string_le() {
        let v = Operation::Le(Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(())));
    }

    #[test]
    fn test_compare_string_btwn() {
        let v = Operation::Btwn(Operand::Value(OperandValue::Str(String::from("f"))), Operand::Value(OperandValue::Str(String::from("j"))));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("e"))), Some(Err(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("f"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("g"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("i"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("j"))), Some(Ok(())));
        assert_eq!(compare(&v, &OperandValue::Str(String::from("k"))), Some(Err(())));
    }

    #[test]
    fn test_compare_u64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::U64(42))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::U64(42))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::U64(42))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::U64(42))), &OperandValue::Bool(false)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::U64(42))), &OperandValue::Str(String::from("abc"))), None);
    }

    #[test]
    fn test_compare_i64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::I64(42))), &OperandValue::U64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::I64(42))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::I64(42))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::I64(42))), &OperandValue::Bool(false)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::I64(42))), &OperandValue::Str(String::from("abc"))), None);
    }

    #[test]
    fn test_compare_f64_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::F64(42.0))), &OperandValue::U64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::F64(42.0))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Bool(false)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::F64(42.0))), &OperandValue::Str(String::from("abc"))), None);
    }

    #[test]
    fn test_compare_usize_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::USize(42))), &OperandValue::U64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::USize(42))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::USize(42))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::USize(42))), &OperandValue::Bool(false)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::USize(42))), &OperandValue::Str(String::from("abc"))), None);
    }

    #[test]
    fn test_compare_bool_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Bool(true))), &OperandValue::U64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Bool(true))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Bool(true))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Bool(true))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Bool(true))), &OperandValue::Str(String::from("abc"))), None);
    }

    #[test]
    fn test_compare_string_other_types() {
        assert_eq!(compare(&Operation::Eq(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::U64(41)), None);
        assert_eq!(compare(&Operation::Ne(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::I64(41)), None);
        assert_eq!(compare(&Operation::Gt(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::F64(41.5)), None);
        assert_eq!(compare(&Operation::Ge(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::USize(41)), None);
        assert_eq!(compare(&Operation::Lt(Operand::Value(OperandValue::Str(String::from("abc")))), &OperandValue::Bool(false)), None);
    }

    #[test]
    fn test_compare_field_path() {
        assert_eq!(compare(&Operation::Eq(Operand::FieldPath(String::from("info"))), &OperandValue::U64(41)), Some(Ok(())));
        assert_eq!(compare(&Operation::Ne(Operand::FieldPath(String::from("info"))), &OperandValue::I64(41)), Some(Ok(())));
        assert_eq!(compare(&Operation::Gt(Operand::FieldPath(String::from("info"))), &OperandValue::F64(41.5)), Some(Ok(())));
        assert_eq!(compare(&Operation::Ge(Operand::FieldPath(String::from("info"))), &OperandValue::USize(41)), Some(Ok(())));
        assert_eq!(compare(&Operation::Lt(Operand::FieldPath(String::from("info"))), &OperandValue::Bool(false)), Some(Ok(())));
    }
}
