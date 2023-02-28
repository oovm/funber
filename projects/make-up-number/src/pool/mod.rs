use crate::{EvaluatedState, ExpressionNode, StopReason};
use ahash::AHashMap;
use dashu::rational::RBig;

pub type NodeID = usize;

#[derive(Default, Debug)]
pub struct ExpressionPool {
    cache: AHashMap<NodeID, EvaluatedState>,
}

#[derive(Debug)]
pub enum EvaluatedState {
    Unevaluated(ExpressionNode),
    Initial(RBig),
    Success(RBig),
    Failure(StopReason),
}
