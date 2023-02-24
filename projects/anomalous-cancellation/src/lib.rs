use std::collections::BTreeMap;
use std::fmt::Binary;
use std::str::FromStr;

use dashu::integer::UBig;
use itertools::Itertools;

pub use errors::{Error, Result};

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
    numerator_rest: String,
    denominator_rest: String,
    /// Numerator removed indexes
    numerator_removed: Vec<usize>,
    /// Denominator removed indexes
    denominator_removed: Vec<usize>,
}

fn digit_cancellation(numerator: &UBig, denominator: &UBig) -> CancellationPlan {
    let mut numerator_digits = numerator.to_string().chars().collect_vec();
    let mut numerator_rest = String::new();
    let mut denominator_rest = String::new();
    let mut numerator_removed = vec![];
    let mut denominator_removed = vec![];
    for (idx_d, num) in denominator.to_string().chars().enumerate() {
        match numerator_digits.iter().position(|&n| n == num) {
            Some(idx_n) => {
                numerator_removed.push(idx_n);
                denominator_removed.push(idx_d);
                numerator_digits.remove(idx_n);
            }
            None => {
                denominator_rest.push(num);
            }
        }
    }
    CancellationPlan {
        numerator_rest,
        denominator_rest,
        numerator_removed,
        denominator_removed,
    }
}

impl CancellationPlan {}


#[test]
fn test() {
    let plan = digit_cancellation(&UBig::from(49usize), &UBig::from(98usize));
    println!("{:#?}", plan);
}