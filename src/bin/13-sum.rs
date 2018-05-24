extern crate num;
#[macro_use]
extern crate project_euler;

use num::Zero;
use num::bigint::BigUint;

fn main() {
    let input = include_str!("input-13");
    let sum = input
        .lines()
        .map(|l| BigUint::parse_bytes(l.as_bytes(), 10).unwrap())
        .fold(BigUint::zero(), |a, v| a + v);
    answer!("{:.10}", sum.to_str_radix(10));
}
