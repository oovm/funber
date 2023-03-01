#![feature(generators)]
#![feature(generator_trait)]

mod expression;
mod operators;
use std::{
    fmt::Write,
    hash::{Hash, Hasher},
    io::Write as _,
    ops::{Add, Div, Generator, Mul, Sub},
    rc::Rc,
    str::FromStr,
};

use ahash::AHasher;
use dashu::{base::UnsignedAbs, rational::RBig};
use itertools::Itertools;

use latexify::Latexify;

pub use crate::{
    errors::StopReason,
    operators::ExpressionAction,
    pool::{evaluate, ExpressionPool, NodeID},
};

mod errors;
mod pool;

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
    Binary { lhs: NodeID, rhs: NodeID, action: ExpressionAction },
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

impl ExpressionTree {
    pub fn is_atom(&self) -> bool {
        match self {
            ExpressionTree::Atomic { .. } => true,
            _ => false,
        }
    }
    pub fn atomic_concat(&self) -> bool {
        match self {
            ExpressionTree::Atomic { .. } => true,
            ExpressionTree::Concat { lhs, rhs } => lhs.atomic_concat() && rhs.atomic_concat(),
            _ => false,
        }
    }
}

pub struct ArithmeticTraverse {
    initials: Vec<usize>,
    pointer: usize,
}

impl ArithmeticTraverse {
    pub fn new(initials: Vec<usize>) -> Self {
        Self { initials, pointer: 0 }
    }
}
pub struct ExpressionPlan {
    pub first: usize,
    pub items: Vec<(ExpressionAction, usize)>,
}

impl Iterator for ArithmeticTraverse {
    type Item = ExpressionPlan;

    fn next(&mut self) -> Option<Self::Item> {
        let actions = self.initials.len() - 1;
        // actions <= log pointer / log 5
        if self.pointer >= 5usize.pow(actions as u32) {
            return None;
        }
        // base 10 pointer to base 5 vec
        let mut actions = vec![ExpressionAction::default(); actions];
        let mut pointer = self.pointer;
        for i in 0..actions.len() {
            actions[i] = ExpressionAction::from(pointer % 5);
            pointer /= 5;
        }
        let plan = ExpressionPlan {
            first: self.initials[0],
            items: actions.into_iter().zip(self.initials[1..].iter().copied()).collect(),
        };
        self.pointer += 1;
        Some(plan)
    }
}
