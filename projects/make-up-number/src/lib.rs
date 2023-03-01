#![feature(generators)]
#![feature(generator_trait)]

use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
};

use ahash::AHasher;
use catalan::{FullBinaryTrees, OperatorPermutation};
use dashu::{integer::IBig, rational::RBig};
use gen_iter::GenIter;

pub use crate::{
    errors::StopReason,
    operators::ExpressionAction,
    pool::{evaluate, ExpressionPool, NodeID},
};

mod errors;
mod expression;
mod operators;
mod pool;

#[derive(Default)]
pub struct ExpressionCache {
    pool: ExpressionPool,
    catalan: FullBinaryTrees,
    values: Vec<usize>,
    operators: Vec<ExpressionAction>,
}

impl Debug for ExpressionCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpressionCache")
            .field("pool", &self.pool)
            .field("catalan", &self.catalan)
            .field("values", &self.values)
            .field("operators", &self.operators)
            .finish()
    }
}

impl ExpressionCache {
    pub fn task(&mut self, values: Vec<usize>, operators: Vec<ExpressionAction>) {
        self.values = values;
        self.operators = operators;
        self.catalan.build_trees(self.values.len());
    }
    pub fn sequence(&self) -> impl Iterator<Item = NodeID> + '_ {
        GenIter(move || {
            for tree in self.catalan.inquire(self.values.len()) {
                for operator in OperatorPermutation::new(&self.operators, self.values.len() - 1) {
                    yield self.pool.register_binary_node(&tree, self.values.clone(), operator);
                }
            }
        })
    }
    pub fn run_expression(&self, id: NodeID) -> Result<IBig, StopReason> {
        evaluate(id, &self.pool)
    }
    pub fn add_expression(&mut self, node: ExpressionNode) -> NodeID {
        match node {
            ExpressionNode::Atomic { number } => self.pool.insert_atomic(number),
            ExpressionNode::Binary { lhs, rhs, action } => self.pool.insert_binary(action, lhs, rhs),
        }
    }
    pub fn get_expression(&self, id: NodeID) -> Result<ExpressionNode, StopReason> {
        self.pool.get_expression(&id)
    }
    pub fn get_display(&self, id: NodeID) -> String {
        let mut buffer = String::new();
        self.pool.rewrite(&id, &mut buffer).ok();
        buffer
    }
}
///
#[derive(Debug, Clone, Hash)]
pub enum ExpressionTree {
    Atomic { number: RBig },
    Add { lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree> },
    Sub { lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree> },
    Mul { lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree> },
    Div { lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree> },
    Concat { lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree> },
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExpressionNode {
    Atomic { number: NodeID },
    Binary { action: ExpressionAction, lhs: NodeID, rhs: NodeID },
}

impl ExpressionNode {
    #[inline]
    pub fn get_id(&self) -> NodeID {
        let mut hasher = AHasher::default();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
    pub fn get_priority(&self) -> usize {
        match self {
            ExpressionNode::Atomic { .. } => 1000,
            ExpressionNode::Binary { action, .. } => match action {
                ExpressionAction::Concat => 900,
                ExpressionAction::Times => 800,
                ExpressionAction::Divide => 800,
                ExpressionAction::Plus => 700,
                ExpressionAction::Minus => 700,
            },
        }
    }
}
