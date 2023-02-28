use std::ops::{Add, Div, Mul, Sub};

use ahash::AHashMap;
use dashu::rational::RBig;

use crate::{ExpressionNode, StopReason};

pub type NodeID = usize;

#[derive(Default, Debug)]
pub struct ExpressionPool {
    cache: AHashMap<NodeID, EvaluatedState>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedState {
    is_initial: bool,
    is_evaluated: bool,
    expression: ExpressionNode,
    result: RBig,
    failure: Option<StopReason>,
}

impl EvaluatedState {
    pub fn initial(number: usize) -> EvaluatedState {
        Self {
            is_initial: true,
            is_evaluated: true,
            expression: ExpressionNode::Atomic,
            result: RBig::from(number),
            failure: None,
        }
    }
    pub fn get_node_id(&self) -> NodeID {
        self.expression.get_id()
    }
}

impl ExpressionPool {
    pub fn evaluate(&mut self, node: &NodeID) -> Result<RBig, StopReason> {
        let mut node = self.find(*node)?;
        let out = if node.is_evaluated {
            node.result.clone()
        }
        else {
            match self.try_evaluate(node) {
                Ok(o) => o,
                Err(e) => {}
            }
        };
        Ok(out)
    }
    fn try_evaluate(&mut self, mut node: EvaluatedState) -> Result<RBig, StopReason> {
        match &node.expression {
            ExpressionNode::Atomic => unreachable!("All atomic nodes should be evaluated"),
            ExpressionNode::Add { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                self.insert(node, lhs.add(rhs))
            }
            ExpressionNode::Sub { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                self.insert(node, lhs.sub(rhs))
            }
            ExpressionNode::Mul { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                self.insert(node, lhs.mul(rhs))
            }
            ExpressionNode::Div { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                self.insert(node, lhs.div(rhs))
            }
            ExpressionNode::Concat { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                self.insert(node, lhs.mul(RBig::from(10)).add(rhs))
            }
        }
        Ok(out)
    }
    pub fn initial(&mut self, value: usize) {
        let out = EvaluatedState::initial(value);
        self.cache.insert(out.get_node_id(), out);
    }
    pub fn insert(&mut self, mut state: EvaluatedState, result: RBig) -> RBig {
        state.is_evaluated = true;
        state.result = result.clone();
        self.cache.insert(state.get_node_id(), state);
        result
    }
    pub fn find(&self, node: NodeID) -> Result<EvaluatedState, StopReason> {
        match self.cache.get(&node) {
            Some(s) => match &s.failure {
                Some(s) => Err(s.clone()),
                None => Ok(s.clone()),
            },
            None => Err(StopReason::NotFound),
        }
    }
    pub fn rewrite(&self, node: NodeID) -> Result<(), StopReason> {
        todo!()
    }
}

#[test]
fn debug() {
    let mut pool = ExpressionPool::default();
    pool.initial(1);
    pool.initial(2);
    println!("{:?}", pool);
}
