extern crate num;
#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::math::digitsum;
use num::bigint::BigUint;

fn main() {
    let mut pow = BigUint::from(2usize);
    for _ in 1..1000 {
        pow = pow * 2usize;
    }
    answer!("{}", digitsum(pow));
}
