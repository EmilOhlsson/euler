#[macro_use]
extern crate project_euler;

use project_euler::series::Primes;

fn main() {
    let mut num: usize = 600_851_475_143;
    let primes: Primes<usize> = Primes::new();

    for p in primes {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            if num == 1 {
                answer!("{}", p);
                return;
            }
        }
    }
}
