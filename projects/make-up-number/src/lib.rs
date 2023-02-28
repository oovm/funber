#![feature(generators)]
#![feature(generator_trait)]
#![feature(box_syntax)]

use std::{
    fmt::{Display, Write},
    io::Write as _,
    ops::Generator,
    str::FromStr,
};
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

use dashu::{
    base::UnsignedAbs,
    rational::RBig,
};
use dashu::integer::{IBig, UBig};
use itertools::Itertools;

use latexify::Latexify;

mod errors;


///
#[derive(Debug, Clone)]
pub enum ExpressionTree {
    Atomic {
        number: RBig,
    },
    Add {
        lhs: Rc<ExpressionTree>,
        rhs: Rc<ExpressionTree>,
    },
    Sub {
        lhs: Rc<ExpressionTree>,
        rhs: Rc<ExpressionTree>,
    },
    Mul {
        lhs: Rc<ExpressionTree>,
        rhs: Rc<ExpressionTree>,
    },
    Div {
        lhs: Rc<ExpressionTree>,
        rhs: Rc<ExpressionTree>,
    },
    Concat {
        lhs: Rc<ExpressionTree>,
        rhs: Rc<ExpressionTree>,
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
            ExpressionTree::Sub { lhs, rhs } => {
                Ok(lhs.eval_nest()?.sub(rhs.eval_nest()?))
            }
            ExpressionTree::Mul { lhs, rhs } => {
                Ok(lhs.eval_nest()?.mul(rhs.eval_nest()?))
            }
            ExpressionTree::Div { lhs, rhs } => {
                Ok(lhs.eval_nest()?.div(rhs.eval_nest()?))
            }
            ExpressionTree::Concat { lhs, rhs } => {
                if lhs.atomic_concat() && rhs.atomic_concat() {
                    Ok(lhs.eval_nest()?.mul(UBig::from(10usize)).add(rhs.eval_nest()?))
                } else {
                    Err(StopReason::NonAtomicConcat)
                }
            }
        }
    }
}

pub fn build_add(lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree>, trees: &mut Vec<ExpressionTree>) {
    trees.push(ExpressionTree::Add {
        lhs: Rc::new(self.clone()),
        rhs: Rc::new(rhs.clone()),
    });
    trees.push(ExpressionTree::Sub {
        lhs: Rc::new(self.clone()),
        rhs: Rc::new(rhs.clone()),
    })
}
pub fn build_mul(lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree>, trees: &mut Vec<ExpressionTree>) {
    trees.push(ExpressionTree::Mul {
        lhs: Rc::new(self.clone()),
        rhs: Rc::new(rhs.clone()),
    });
    trees.push(ExpressionTree::Div {
        lhs: Rc::new(self.clone()),
        rhs: Rc::new(rhs.clone()),
    })
}
pub fn build_concat(lhs: Rc<ExpressionTree>, rhs: Rc<ExpressionTree>, trees: &mut Vec<ExpressionTree>) {
    trees.push(ExpressionTree::Concat {
        lhs: self.clone(),
        rhs: rhs.clone(),
    })
}


#[test]
fn test() {
    let mut expressions = vec![];
    let lhs = ExpressionTree::from(1);
    let rhs = ExpressionTree::from(2);
    lhs.build_add(&rhs, &mut expressions);
    lhs.build_concat(&rhs, &mut expressions);
    for expr in expressions {
        println!("{:?} => {:#?}", expr, expr.clone().eval().unwrap())
    }
}
