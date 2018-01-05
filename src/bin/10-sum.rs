#[macro_use]
extern crate project_euler;

use project_euler::series::Primes;

fn main() {
    let primes: Primes<usize> = Primes::new();
    let sum = primes
        .into_iter()
        .take_while(|&p| p < 2_000_000)
        .sum::<usize>();
    answer!("{}", sum);
}
