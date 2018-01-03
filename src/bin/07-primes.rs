#[macro_use]
extern crate project_euler;

use project_euler::series::Primes;

fn main() {
    let p: Primes<usize> = Primes::new();
    answer!("{}", p.into_iter().nth(10_000).unwrap())
}
