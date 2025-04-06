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
    Lt(T),
    Ge(T),
    Le(T),
    Btwn(T, T),
}

fn compare_eq<T>(operation: &OperationEq<T>, value: &T) -> Result<(), OperationEq<T>>
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

#[cfg(test)]
mod test {
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
        assert_eq!(compare_eq(&OperationEq::Eq(10 as u64), &10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(10 as u64), &42), Ok(()));
    }

    #[test]
    fn test_compare_eq_u64_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(10 as u64), &8), Err(OperationEq::Eq(10 as u64)));
        assert_eq!(compare_eq(&OperationEq::Ne(10 as u64), &10), Err(OperationEq::Ne(10 as u64)));
    }

    #[test]
    fn test_compare_eq_i64_ok() {
        assert_eq!(compare_eq(&OperationEq::Eq(-10 as i64), &-10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(-10 as i64), &-42), Ok(()));
    }

    #[test]
    fn test_compare_eq_i64_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(-10 as i64), &8), Err(OperationEq::Eq(-10 as i64)));
        assert_eq!(compare_eq(&OperationEq::Ne(-10 as i64), &-10), Err(OperationEq::Ne(-10 as i64)));
    }

    #[test]
    fn test_compare_eq_f64_ok() {
        assert_eq!(compare_eq(&OperationEq::Eq(-10.5 as f64), &-10.5), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(-10.5 as f64), &-42.5), Ok(()));
    }

    #[test]
    fn test_compare_eq_f64_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(-10.5 as f64), &-4.25), Err(OperationEq::Eq(-10.5 as f64)));
        assert_eq!(compare_eq(&OperationEq::Ne(-10.5 as f64), &-10.5), Err(OperationEq::Ne(-10.5 as f64)));
    }

    #[test]
    fn test_compare_eq_usize_ok() {
        assert_eq!(compare_eq(&OperationEq::Eq(10 as usize), &10), Ok(()));
        assert_eq!(compare_eq(&OperationEq::Ne(10 as usize), &42), Ok(()));
    }

    #[test]
    fn test_compare_eq_usize_err() {
        assert_eq!(compare_eq(&OperationEq::Eq(10 as usize), &8), Err(OperationEq::Eq(10)));
        assert_eq!(compare_eq(&OperationEq::Ne(10 as usize), &10), Err(OperationEq::Ne(10)));
    }
}
