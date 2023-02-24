#![feature(generators)]
#![feature(generator_trait)]

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::Generator;
use std::str::FromStr;

use dashu::base::UnsignedAbs;
use dashu::integer::{IBig, UBig};
use dashu::rational::RBig;
use gen_iter::GenIter;
use itertools::Itertools;
use latexify::Latexify;

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
    numerator: UBig,
    denominator: UBig,
    numerator_rest: UBig,
    denominator_rest: UBig,
    /// Numerator removed indexes
    numerator_removed: Vec<usize>,
    /// Denominator removed indexes
    denominator_removed: Vec<usize>,
}

impl CancellationPlan {
    pub fn contains_zero(&self) -> bool {
        self.numerator.to_string().contains('0') || self.denominator.to_string().contains('0')
    }
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
        numerator: numerator.clone(),
        denominator: denominator.clone(),
        numerator_rest: UBig::from_str(&String::from_iter(numerator_digits)).unwrap(),
        denominator_rest: UBig::from_str(&denominator_rest).unwrap(),
        numerator_removed,
        denominator_removed,
    })
}

impl Display for CancellationPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
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

impl Latexify for CancellationPlan {
    type Context = ();

    fn latexify(&self) -> String {
        let mut out = String::new();

    }
}


// latexify
fn find_in(numerator: usize, denominator: usize, start: &UBig) -> impl Iterator<Item=CancellationPlan> {
    let mut times = start.clone();
    GenIter(move || {
        loop {
            let num = UBig::from(numerator) * times.clone();
            let den = UBig::from(denominator) * times.clone();
            let r = RBig::from_parts(IBig::from(&num), den.clone());
            match digit_cancellation(&num, &den) {
                Ok(o) if o.eq(&r) => {
                    yield o;
                }
                _ => {}
            }
            times += UBig::ONE;
        }
    })
}

// \frac{1\enclose{updiagonalstrike}[mathcolor="red"]{\color{black}{2}}3}{456}
#[test]
fn test() {
    for i in find_in(1, 2, &UBig::ONE).filter(|r| !r.contains_zero()).take(10) {
        println!("{}", i);
    }
    for i in find_in(1, 3, &UBig::ONE).filter(|r| !r.contains_zero()).take(10) {
        println!("{}", i);
    }
    for i in find_in(1, 4, &UBig::ONE).filter(|r| !r.contains_zero()).take(10) {
        println!("{}", i);
    }
}