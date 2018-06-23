extern crate num;

#[macro_use]
extern crate cch_utils;

use cch_utils::math::{digitsum, faculty};
use num::bigint::BigUint;

fn main() {
    let sum = digitsum(&faculty(BigUint::from(100usize)));
    answer!("{}", sum);
}
