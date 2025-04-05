#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Eq(usize),
    Ne(usize),
    Gt(usize),
    Lt(usize),
    Ge(usize),
    Le(usize),
    Btwn(usize, usize),
}
