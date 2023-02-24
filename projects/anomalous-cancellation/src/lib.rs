#![feature(generators)]
#![feature(generator_trait)]

use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter, Write},
    fs::File,
    io::Write as _,
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
use serde_derive::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn tailing_zero(&self) -> bool {
        self.numerator.to_string().ends_with('0') || self.denominator.to_string().ends_with('0')
    }
}

fn digit_cancellation(numerator: &UBig, denominator: &UBig) -> Result<CancellationPlan, &'static str> {
    // numerator [1, 6, 3]
    // denominator [3?, 2?, 6?]
    let mut n_digits = numerator.to_string().chars().enumerate().collect_vec();
    let mut d_digits = denominator.to_string().chars().enumerate().collect_vec();
    let mut n_removed = vec![];
    let mut d_removed = vec![];

    for (_, n) in n_digits.clone().iter() {
        let d = match d_digits.iter().position(|(_, d)| n.eq(d)) {
            Some(d_idx) => {
                let rd = d_digits.remove(d_idx);
                d_removed.push(rd.0);
                rd.1
            }
            None => continue,
        };
        match n_digits.iter().position(|(_, n)| d.eq(n)) {
            Some(n_idx) => {
                let rn = n_digits.remove(n_idx);
                n_removed.push(rn.0);
            }
            None => continue,
        }
    }

    let mut n_rest = n_digits.iter().map(|(_, c)| c).collect::<String>();
    let mut d_rest = d_digits.iter().map(|(_, c)| c).collect::<String>();
    if n_rest.is_empty() || d_rest.is_empty() {
        Err("All digits are cancelled")?
    }
    if n_removed.is_empty() && d_removed.is_empty() {
        Err("No digit is cancelled")?
    }
    Ok(CancellationPlan {
        numerator: numerator.clone(),
        denominator: denominator.clone(),
        numerator_rest: UBig::from_str(&n_rest).unwrap(),
        denominator_rest: UBig::from_str(&d_rest).unwrap(),
        numerator_removed: n_removed,
        denominator_removed: d_removed,
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
        for (idx, c) in self.numerator.to_string().chars().enumerate() {
            if self.numerator_removed.contains(&idx) {
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
            if self.denominator_removed.contains(&idx) {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    numerator: usize,
    denominator: usize,
    current: UBig,
    collection: Vec<CancellationPlan>,
}

pub fn debug_latex(numerator: usize, denominator: usize, limit: usize) -> String {
    let mut latex = format!("### {}/{}\n", numerator, denominator);
    latex.push_str("$$\\begin{aligned}\n");
    for i in find_in(numerator, denominator, &UBig::ONE).filter(|r| !r.contains_zero()).take(limit) {
        i.latexify(&mut latex).unwrap();
        write!(latex, "&=\\frac{{{}}}{{{}}}\\\\\n", numerator, denominator).unwrap();
    }
    latex.push_str("\\end{aligned}$$\n\n");
    latex
}
