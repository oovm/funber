#![feature(generators)]
#![feature(generator_trait)]

use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter, Write},
    ops::Generator,
    str::FromStr,
};

use dashu::{
    base::UnsignedAbs,
    integer::{IBig, UBig},
    rational::RBig,
};
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
    numerator_removes: Vec<bool>,
    denominator_removes: Vec<bool>,
}

impl CancellationPlan {
    pub fn contains_zero(&self) -> bool {
        self.numerator.to_string().contains('0') || self.denominator.to_string().contains('0')
    }
}

fn digit_cancellation(numerator: &UBig, denominator: &UBig) -> Result<CancellationPlan, &'static str> {
    let mut numerator_digits = numerator.to_string().chars().collect_vec();
    let mut denominator_rest = String::new();
    let mut numerator_removes = vec![];
    let mut denominator_removes = vec![];
    for (_, d) in denominator.to_string().chars().enumerate() {
        match numerator_digits.iter().position(|&n| n == d) {
            Some(idx_n) => {
                numerator_removes.push(true);
                denominator_removes.push(true);
                numerator_digits.remove(idx_n);
            }
            None => {
                numerator_removes.push(false);
                denominator_removes.push(true);
                denominator_rest.push(d);
            }
        }
    }
    if numerator_digits.is_empty() || denominator_rest.is_empty() {
        Err("All digits are cancelled")?
    }
    if numerator_removes.is_empty() && denominator_removes.is_empty() {
        Err("No digit is cancelled")?
    }
    Ok(CancellationPlan {
        numerator: numerator.clone(),
        denominator: denominator.clone(),
        numerator_rest: UBig::from_str(&String::from_iter(numerator_digits)).unwrap(),
        denominator_rest: UBig::from_str(&denominator_rest).unwrap(),
        numerator_removes,
        denominator_removes,
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

impl Display for CancellationPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Latexify for CancellationPlan {
    fn latexify<W>(&self, f: &mut W) -> std::fmt::Result
    where
        W: Write,
    {
        f.write_str(r"\frac{")?;
        let lhs = self.numerator.to_string().chars().zip()
        let rhs = self.numerator_removes.iter()

        for (idx, c) in self.numerator.to_string().chars().enumerate() {
            if self.numerator_removes.contains(&idx) {
                f.write_str(r"\color{red}{")?;
                f.write_char(c)?;
                f.write_str("}")?;
            }
            else {
                f.write_char(c)?;
            }
        }
        f.write_str("}{")?;
        for (idx, c) in self.denominator.to_string().chars().enumerate() {
            if self.denominator_removes.contains(&idx) {
                f.write_str(r"\color{red}{")?;
                f.write_char(c)?;
                f.write_str("}")?;
            }
            else {
                f.write_char(c)?;
            }
        }
        f.write_str("}")
    }
}

// latexify
fn find_in(numerator: usize, denominator: usize, start: &UBig) -> impl Iterator<Item = CancellationPlan> {
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
        println!("{}", i.to_latex());
    }
    for i in find_in(1, 3, &UBig::ONE).filter(|r| !r.contains_zero()).take(10) {
        println!("{}", i.to_latex());
    }
    for i in find_in(1, 4, &UBig::ONE).filter(|r| !r.contains_zero()).take(10) {
        println!("{}", i.to_latex());
    }
}
