#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Primes;

fn main() {
    let p: Primes<usize> = Primes::new();
    answer!("{}", p.into_iter().nth(10_000).unwrap())
}
