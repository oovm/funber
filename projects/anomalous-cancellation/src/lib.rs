#![feature(generators)]
#![feature(generator_trait)]

use std::collections::BTreeMap;
use std::fmt::Binary;
use std::ops::GeneratorState;
use std::pin::Pin;
use std::str::FromStr;
use gen_iter::gen_iter;
use dashu::base::UnsignedAbs;
use dashu::integer::{IBig, UBig};
use dashu::rational::RBig;
use itertools::Itertools;

mod errors;


pub fn collect_digits(number: &UBig) -> BTreeMap<char, usize> {
    let mut digits = BTreeMap::new();
    for digit in number.to_string().chars() {
        let count = digits.entry(digit).or_insert(0);
        *count += 1;
    }
    digits
}

#[derive(Debug)]
pub struct CancellationPlan {
    numerator_rest: UBig,
    denominator_rest: UBig,
    /// Numerator removed indexes
    numerator_removed: Vec<usize>,
    /// Denominator removed indexes
    denominator_removed: Vec<usize>,
}

fn digit_cancellation(numerator: &UBig, denominator: &UBig) -> Result<CancellationPlan, &'static str>
{
    let mut numerator_digits = numerator.to_string().chars().collect_vec();
    let mut denominator_rest = String::new();
    let mut numerator_removed = vec![];
    let mut denominator_removed = vec![];
    for (idx_d, d) in denominator.to_string().chars().enumerate() {
        match numerator_digits.iter().position(|&n| n == d) {
            Some(idx_n) => {
                numerator_removed.push(idx_n);
                denominator_removed.push(idx_d);
                numerator_digits.remove(idx_n);
            }
            None => {
                denominator_rest.push(d);
            }
        }
    }
    if numerator_digits.is_empty() || denominator_rest.is_empty() {
        Err("All digits are cancelled")?
    }
    if numerator_removed.is_empty() && denominator_removed.is_empty() {
        Err("No digit is cancelled")?
    }
    Ok(CancellationPlan {
        numerator_rest: UBig::from_str(&String::from_iter(numerator_digits)).unwrap(),
        denominator_rest: UBig::from_str(&denominator_rest).unwrap(),
        numerator_removed,
        denominator_removed,
    })
}

impl CancellationPlan {
    pub fn as_factor(&self) -> RBig {
        let numerator = self.numerator_rest.clone();
        let denominator = self.denominator_rest.clone();
        RBig::from_parts(IBig::from(numerator), denominator)
    }
    pub fn equivalent_to(&self, numerator: &UBig, denominator: &UBig) -> bool {
        self.numerator_rest.eq(numerator) && self.denominator_rest.eq(denominator)
    }
}

impl PartialEq<RBig> for CancellationPlan {
    fn eq(&self, other: &RBig) -> bool {
        let num = other.numerator().unsigned_abs().eq(&self.numerator_rest);
        let den = other.denominator().eq(&self.denominator_rest);
        num && den
    }
}


fn find_in(numerator: usize, denominator: usize) -> Option<CancellationPlan> {
    let mut times = UBig::ONE;
    let mut generator = || {
        for _ in 0..10 {
            let num = UBig::from(numerator) * times.clone();
            let den = UBig::from(denominator) * times.clone();
            let r = RBig::from_parts(IBig::from(&num), den.clone());
            match digit_cancellation(&num, &den) {
                Ok(o) if o.eq(&r) => {
                    yield o;
                }
                _ => {}
            }
        }
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(a) => {}
        GeneratorState::Complete(b) => {}
    }

    unreachable!()
}

#[test]
fn test() {
    println!("{:?}", find_in(1, 2))
}