#[derive(Clone, Debug)]
pub enum StopReason {
    NotFound,
    DividedByZero,
    NotInteger,
    NonAtomicConcat,
}
