use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Err {
    Required,
    Bool,
    Obj,
    Eq(bool),
    Ne(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrWrap {
    Arr(Vec<Err>),
    Obj(HashMap<String, ErrWrap>),
}

impl ErrWrap {
    pub fn arr<const N: usize>(value: [Err; N]) -> Option<ErrWrap> {
        Some(ErrWrap::Arr(value.to_vec()))
    }

    pub fn obj(value: HashMap<String, ErrWrap>) -> Option<ErrWrap> {
        Some(ErrWrap::Obj(value))
    }
}

#[cfg(test)]
mod test {
    use crate::error::Err;

    use super::*;

    #[test]
    fn test_arr() {
        assert_eq!(ErrWrap::arr([Err::Required]), Some(ErrWrap::Arr(vec![Err::Required])));
    }

    #[test]
    fn test_obj() {
        assert_eq!(
            ErrWrap::obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Required]))])),
            Some(ErrWrap::Obj(HashMap::from([(
                String::from("is"),
                ErrWrap::Arr(vec![Err::Required])
            )])))
        );
    }
}
