extern crate num;
#[macro_use]
extern crate project_euler;

use num::bigint::BigUint;
use project_euler::math::digitsum;

fn main() {
    let mut pow = BigUint::from(2usize);
    for _ in 1..1000 {
        pow = pow * 2usize;
    }
    answer!("{}", digitsum(&pow));
}
