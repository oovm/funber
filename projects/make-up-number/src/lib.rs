#![feature(generators)]
#![feature(generator_trait)]
#![feature(box_syntax)]

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
use dashu::integer::{IBig, UBig};
use itertools::Itertools;

use latexify::Latexify;

mod errors;


///
#[derive(Debug)]
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

impl From<usize> for ExpressionTree {
    fn from(value: usize) -> Self {
        Self::Atomic {
            number: RBig::from(value),
        }
    }
}

#[derive(Debug)]
pub enum StopReason {
    NotInteger,
    NonAtomicConcat,
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
            ExpressionTree::Atomic { .. } => { true }
            ExpressionTree::Concat { lhs, rhs } => {
                lhs.atomic_concat() && rhs.atomic_concat()
            }
            _ => false,
        }
    }
}

impl ExpressionTree {
    pub fn eval(self) -> Result<IBig, StopReason> {
        let (num, den) = self.eval_nest()?.into_parts();
        if !den.is_one() {
            Err(StopReason::NotInteger)
        } else {
            Ok(num)
        }
    }
    fn eval_nest(self) -> Result<RBig, StopReason> {
        match self {
            ExpressionTree::Atomic { number } => {
                Ok(number)
            }
            ExpressionTree::Add { lhs, rhs } => {
                Ok(lhs.eval_nest()?.add(rhs.eval_nest()?))
            }
            ExpressionTree::Mul { lhs, rhs } => {
                Ok(lhs.eval_nest()?.mul(rhs.eval_nest()?))
            }
            ExpressionTree::Concat { lhs, rhs } => {
                Ok(lhs.eval_nest()?.mul(UBig::from(10usize)).add(rhs.eval_nest()?))
            }
        }
    }
}

#[test]
fn test() {
    let add = ExpressionTree::Add {
        lhs: box ExpressionTree::Concat {
            lhs: Box::new(ExpressionTree::from(1)),
            rhs: Box::new(ExpressionTree::from(2)),
        },
        rhs: Box::new(ExpressionTree::Atomic { number: RBig::from(1) }),
    };
    println!("{:#?}", add.eval().unwrap())
}
