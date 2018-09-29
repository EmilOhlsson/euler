extern crate num;

#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::math::{digitsum, faculty};
use num::bigint::BigUint;

fn main() {
    let sum = digitsum(faculty(BigUint::from(100usize)));
    answer!("{}", sum);
}
