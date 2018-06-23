#[macro_use]
extern crate cch_utils;

use cch_utils::series::Primes;

fn main() {
    let p: Primes<usize> = Primes::new();
    answer!("{}", p.into_iter().nth(10_000).unwrap())
}
