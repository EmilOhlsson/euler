extern crate num;
#[macro_use]
extern crate cch_utils;

use cch_utils::math::digitsum;
use num::bigint::BigUint;

fn main() {
    let mut pow = BigUint::from(2usize);
    for _ in 1..1000 {
        pow = pow * 2usize;
    }
    answer!("{}", digitsum(pow));
}
