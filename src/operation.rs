#[derive(Debug, PartialEq, Clone)]
pub enum OperationEq<T> {
    Eq(T),
    Ne(T),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationComp<T> {
    Gt(T),
    Lt(T),
    Ge(T),
    Le(T),
    Btwn(T, T),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation<T> {
    Eq(T),
    Ne(T),
    Gt(T),
    Ge(T),
    Lt(T),
    Le(T),
    Btwn(T, T),
}

pub fn compare_eq<T>(operation: &OperationEq<T>, value: &T) -> Result<(), OperationEq<T>>
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
    Ok(())
}

pub fn compare<T>(operation: &Operation<T>, value: &T) -> Result<(), Operation<T>>
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
    Ok(())
}

#[cfg(test)]
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
