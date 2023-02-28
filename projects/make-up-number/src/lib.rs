#![feature(generators)]
#![feature(generator_trait)]

use std::{
    fmt::{Display, Write},
    io::Write as _,
    ops::Generator,
    str::FromStr,
};
use std::ops::{Add, Mul};

use dashu::{
    base::UnsignedAbs,
    rational::RBig,
};
use itertools::Itertools;

use latexify::Latexify;

mod errors;


///
pub enum ExpressionTree {
    Atomic {
        number: RBig,
    },
    Add {
        lhs: Box<ExpressionTree>,
        rhs: Box<ExpressionTree>,
    },
    Mul {
        lhs: Box<ExpressionTree>,
        rhs: Box<ExpressionTree>,
    },
    Concat {
        lhs: Box<ExpressionTree>,
        rhs: Box<ExpressionTree>,
    },
}

pub enum StopReason {}

impl ExpressionTree{
    pub fn is_atom(&self)

}

impl ExpressionTree {
    pub fn eval(self) -> Result<ExpressionTree, StopReason> {
        match &self {
            ExpressionTree::Atomic { .. } => {
                Ok(self)
            }
            ExpressionTree::Add { lhs, rhs } => {
                Ok(lhs.add(rhs))
            }
            ExpressionTree::Mul { lhs, rhs } => {
                Ok(lhs.mul(rhs))
            }
            ExpressionTree::Concat { lhs, rhs } => {
                lhs.mul(10).add(rhs)
            }
        }
    }
}


impl<'a> Add<&'a Self> for ExpressionTree {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {

    }
}


impl<'a> Mul<&'a Self> for ExpressionTree {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {}
}


