#[derive(Clone, Debug)]
pub enum StopReason {
    NotFound,
    DividedByZero,
    NotInteger,
    NonAtomicConcat,
    RuntimeError { message: String },
}

impl From<std::fmt::Error> for StopReason {
    fn from(e: std::fmt::Error) -> Self {
        StopReason::RuntimeError { message: e.to_string() }
    }
}
