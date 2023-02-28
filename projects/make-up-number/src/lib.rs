#![feature(generators)]
#![feature(generator_trait)]
#![feature(box_syntax)]

use std::{
    fmt::{Display, Write},
    hash::{Hash, Hasher},
    io::Write as _,
    ops::{Add, Div, Generator, Mul, Sub},
    rc::Rc,
    str::FromStr,
};

use ahash::{AHashMap, AHasher};
use dashu::{
    base::UnsignedAbs,
    integer::{IBig, UBig},
    rational::RBig,
};
use itertools::Itertools;

use latexify::Latexify;

pub use crate::errors::StopReason;

mod errors;
mod pool;

pub use crate::pool::{ExpressionPool, NodeID};

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
    Atomic,
    Add { lhs: NodeID, rhs: NodeID },
    Sub { lhs: NodeID, rhs: NodeID },
    Mul { lhs: NodeID, rhs: NodeID },
    Div { lhs: NodeID, rhs: NodeID },
    Concat { lhs: NodeID, rhs: NodeID },
}

impl ExpressionNode {
    #[inline]
    pub fn get_id(&self) -> NodeID {
        let mut hasher = AHasher::default();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
}

// impl From<usize> for ExpressionNode {
//     fn from(value: usize) -> Self {
//         let mut hasher = AHasher::default();
//         value.hash(&mut hasher);
//         Self::Atomic { number: hasher.finish() as usize }
//     }
// }

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

impl ExpressionTree {
    pub fn evaluate(&self) -> Result<IBig, StopReason> {
        let (num, den) = self.eval_nest()?.into_parts();
        if !den.is_one() { Err(StopReason::NotInteger) } else { Ok(num) }
    }
    fn eval_nest(&self) -> Result<RBig, StopReason> {
        match self {
            ExpressionTree::Atomic { number } => Ok(number.clone()),
            ExpressionTree::Add { lhs, rhs } => Ok(lhs.eval_nest()?.add(rhs.eval_nest()?)),
            ExpressionTree::Sub { lhs, rhs } => Ok(lhs.eval_nest()?.sub(rhs.eval_nest()?)),
            ExpressionTree::Mul { lhs, rhs } => Ok(lhs.eval_nest()?.mul(rhs.eval_nest()?)),
            ExpressionTree::Div { lhs, rhs } => Ok(lhs.eval_nest()?.div(rhs.eval_nest()?)),
            ExpressionTree::Concat { lhs, rhs } => {
                if lhs.atomic_concat() && rhs.atomic_concat() {
                    Ok(lhs.eval_nest()?.mul(UBig::from(10usize)).add(rhs.eval_nest()?))
                }
                else {
                    Err(StopReason::NonAtomicConcat)
                }
            }
        }
    }
}
// pub struct ExpressionQuery {
//     trees: Vec<i8>,
// }
//
// impl ExpressionQuery {
//     pub fn build_action(&mut self, action: &[usize]) -> ExpressionTree {
//         assert!(self.trees.len() >= 2);
//         let mut items = self.trees.iter().rev();
//         let mut actions = action.iter();
//         let mut lhs = items.next().unwrap();
//         while let Some(rhs) = items.next() {
//             match actions.next() {
//                 Some(0) => {}
//                 _ => break,
//             }
//         }
//     }
//
//     pub fn build_add(&mut self, lhs: &Rc<ExpressionTree>, rhs: &Rc<ExpressionTree>) {
//         self.trees.push(Rc::new(ExpressionTree::Add { lhs: lhs.clone(), rhs: rhs.clone() }));
//         self.trees.push(Rc::new(ExpressionTree::Sub { lhs: rhs.clone(), rhs: lhs.clone() }));
//     }
//     pub fn build_mul(&mut self, lhs: &Rc<ExpressionTree>, rhs: &Rc<ExpressionTree>) {
//         self.trees.push(Rc::new(ExpressionTree::Mul { lhs: lhs.clone(), rhs: rhs.clone() }));
//         self.trees.push(Rc::new(ExpressionTree::Div { lhs: lhs.clone(), rhs: rhs.clone() }))
//     }
//     pub fn build_concat(&mut self, lhs: &Rc<ExpressionTree>, rhs: &Rc<ExpressionTree>) {
//         self.trees.push(Rc::new(ExpressionTree::Concat { lhs: lhs.clone(), rhs: rhs.clone() }))
//     }
// }
//
// #[test]
// fn test() {
//     let mut items = vec![ExpressionTree::from(1), ExpressionTree::from(2)];
//
//     let mut items = items.into_iter().rev();
//     let lhs = Rc::new(items.next()).unwrap();
//     while let Some(rhs) = items.next() {}
//
//     let lhs = Rc::new(ExpressionTree::from(1));
//     let mut expressions = vec![lhs];
//     let rhs = Rc::new(ExpressionTree::from(2));
//
//     for lhs in expressions.clone().iter() {
//         build_add(lhs, &rhs, &mut expressions);
//         build_mul(lhs, &rhs, &mut expressions);
//         build_concat(lhs, &rhs, &mut expressions);
//     }
//
//     build_add(&lhs, &rhs, &mut expressions);
//     build_concat(&lhs, &rhs, &mut expressions);
//
//     for expr in expressions {
//         match expr.evaluate() {
//             Ok(o) => {
//                 println!("{:?} => {:#?}", expr, o)
//             }
//             Err(_) => {
//                 continue;
//             }
//         }
//     }
// }
