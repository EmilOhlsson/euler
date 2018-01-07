#[macro_use]

extern crate project_euler;
extern crate num;

use num::bigint::BigUint;
use num::Zero;

fn main() {
    let input = include_str!("input-13");
    let sum = input
        .lines()
        .map(|l| BigUint::parse_bytes(l.as_bytes(), 10).unwrap())
        .fold(BigUint::zero(), |a, v| a + v);
    answer!("{:.10}", sum.to_str_radix(10));
}