#[macro_use]
extern crate project_euler;
extern crate num;

use project_euler::math::{digitsum, faculty};
use num::bigint::BigUint;

fn main() {
    let sum = digitsum(&faculty(&BigUint::from(100usize)));
    answer!("{}", sum);
}
