#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Primes;

fn main() {
    let primes: Primes<usize> = Primes::new();
    let sum = primes
        .into_iter()
        .take_while(|&p| p < 2_000_000)
        .sum::<usize>();
    answer!("{}", sum);
}
