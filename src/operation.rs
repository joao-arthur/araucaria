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
