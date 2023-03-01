#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExpressionAction {
    Concat = 0,
    Plus = 1,
    Minus = 2,
    Times = 3,
    Divide = 4,
}

impl Default for ExpressionAction {
    fn default() -> Self {
        Self::Concat
    }
}

impl From<usize> for ExpressionAction {
    fn from(value: usize) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
