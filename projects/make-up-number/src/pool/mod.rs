use std::{
    fmt::{Debug, Formatter, Write},
    ops::{Add, Div, Mul, Sub},
};

use ahash::RandomState;
use dashmap::{mapref::one::Ref, DashMap};
use dashu::{integer::IBig, rational::RBig};

use crate::{ExpressionNode, StopReason};

pub type NodeID = usize;

#[derive(Default, Debug)]
pub struct ExpressionPool {
    cache: DashMap<NodeID, EvaluatedState, RandomState>,
}

#[derive(Clone)]
pub struct EvaluatedState {
    is_initial: bool,
    is_evaluated: bool,
    expression: ExpressionNode,
    result: RBig,
    failure: Option<StopReason>,
}

impl Debug for EvaluatedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.failure {
            Some(s) => f.debug_struct("EvaluateFailure").field("reason", &s).finish(),
            None if self.is_evaluated => {
                f.debug_struct("EvaluateSuccess").field("expression", &self.expression).field("result", &self.result).finish()
            }
            None => f.debug_struct("EvaluatePending").field("expression", &self.expression).finish(),
        }
    }
}

impl EvaluatedState {
    pub fn initial(number: usize) -> EvaluatedState {
        Self {
            is_initial: true,
            is_evaluated: true,
            expression: ExpressionNode::Atomic { number },
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
        let mut node = self.find(node)?;
        if node.is_evaluated {
            return Ok(node.result.clone());
        }
        match self.try_evaluate(&node) {
            Ok(o) => Ok(self.update_success(node, o)),
            Err(e) => Err(self.update_failure(node, e)),
        }
    }
    fn try_evaluate(&mut self, node: &EvaluatedState) -> Result<RBig, StopReason> {
        let out = match &node.expression {
            ExpressionNode::Atomic { .. } => unreachable!("All atomic nodes should be evaluated"),
            ExpressionNode::Add { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                lhs.add(rhs)
            }
            ExpressionNode::Sub { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                lhs.sub(rhs)
            }
            ExpressionNode::Mul { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                lhs.mul(rhs)
            }
            ExpressionNode::Div { lhs, rhs } => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                if rhs.is_zero() {
                    return Err(StopReason::DividedByZero);
                }
                lhs.div(rhs)
            }
            ExpressionNode::Concat { lhs, rhs } => {
                if !node.expression.is_atomic_concat(self) {
                    Err(StopReason::NonAtomicConcat)?;
                }
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;
                lhs.mul(RBig::from(10)).add(rhs)
            }
        };
        Ok(out)
    }
    pub fn initial(&mut self, value: usize) -> NodeID {
        let out = EvaluatedState::initial(value);
        let id = out.get_node_id();
        self.cache.insert(id, out);
        id
    }
    pub fn expression(&mut self, node: ExpressionNode) -> NodeID {
        let out = EvaluatedState {
            //
            is_initial: false,
            is_evaluated: false,
            expression: node,
            result: RBig::default(),
            failure: None,
        };
        let id = out.get_node_id();
        self.cache.insert(id, out);
        id
    }
    pub fn update_success(&mut self, mut state: EvaluatedState, result: RBig) -> RBig {
        state.is_evaluated = true;
        state.result = result.clone();
        self.cache.insert(state.get_node_id(), state);
        result
    }
    pub fn update_failure(&mut self, mut state: EvaluatedState, reason: StopReason) -> StopReason {
        state.is_evaluated = true;
        state.failure = Some(reason.clone());
        self.cache.insert(state.get_node_id(), state);
        reason
    }
    pub fn find(&self, node: &NodeID) -> Result<EvaluatedState, StopReason> {
        match self.cache.get(node) {
            Some(s) => match &s.failure {
                Some(s) => Err(s.clone()),
                None => Ok(s.clone()),
            },
            None => Err(StopReason::NotFound),
        }
    }
    pub fn rewrite<W: Write>(&self, node: &NodeID, w: &mut W) -> Result<(), StopReason> {
        let node = self.cache.get(node).ok_or(StopReason::NotFound)?;
        match node.expression {
            ExpressionNode::Atomic { number } => {
                write!(w, "{}", number)?;
            }
            ExpressionNode::Add { lhs, rhs } => {
                self.rewrite(&lhs, w)?;
                write!(w, " + ")?;
                self.rewrite(&rhs, w)?;
            }
            ExpressionNode::Sub { lhs, rhs } => {
                self.rewrite(&lhs, w)?;
                write!(w, " - ")?;
                self.rewrite(&rhs, w)?;
            }
            ExpressionNode::Mul { lhs, rhs } => {
                // if inner is add
                match self.cache.get(&lhs).unwrap().expression {
                    ExpressionNode::Add { .. } | ExpressionNode::Sub { .. } => {
                        write!(w, "(")?;
                        self.rewrite(&lhs, w)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.rewrite(&lhs, w)?;
                    }
                }
                write!(w, " * ")?;
                if let ExpressionNode::Add { .. } | ExpressionNode::Sub { .. } = self.cache.get(&rhs).unwrap().expression {
                    write!(w, "(")?;
                    self.rewrite(&rhs, w)?;
                    write!(w, ")")?;
                }
                else {
                    self.rewrite(&rhs, w)?;
                }
            }
            ExpressionNode::Div { lhs, rhs } => {
                self.rewrite(&lhs, w)?;
                write!(w, " / ")?;
                self.rewrite(&rhs, w)?;
            }
            ExpressionNode::Concat { lhs, rhs } => {
                self.rewrite(&lhs, w)?;
                self.rewrite(&rhs, w)?;
            }
        }
        Ok(())
    }
}

impl ExpressionNode {
    pub fn is_atomic_concat(&self, pool: &ExpressionPool) -> bool {
        match self {
            ExpressionNode::Atomic { .. } => true,
            ExpressionNode::Concat { lhs, rhs } => match (pool.cache.get(lhs), pool.cache.get(rhs)) {
                (Some(lhs), Some(rhs)) => lhs.expression.is_atomic_concat(pool) && rhs.expression.is_atomic_concat(pool),
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn evaluate(id: NodeID, pool: &mut ExpressionPool) -> Result<IBig, StopReason> {
    let result = pool.evaluate(&id)?.into_parts();
    if !result.1.is_one() {
        Err(StopReason::NotInteger)?
    }
    Ok(result.0)
}

#[test]
fn debug() {
    let mut pool = ExpressionPool::default();
    let lhs = pool.initial(1);
    let rhs = pool.initial(2);
    let id = pool.expression(ExpressionNode::Add { lhs, rhs });
    let mut expression = String::new();
    pool.rewrite(&id, &mut expression).unwrap();
    println!("{:#?}", evaluate(id, &mut pool));
    println!("{:#?}", expression);
}

#[test]
fn debug2() {
    println!("{:#?}", RBig::from(3).div(RBig::from(4)).div(RBig::from(5)));
}
