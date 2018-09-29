#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Primes;
use std::collections::HashSet;

fn main() {
    let mut a_best = 0;
    let mut b_best = 0;
    let mut best = 0;

    let primes: HashSet<isize> = {
        let mut p = Primes::new();
        p.take(1000)
            .map(|n: usize| n as isize)
            .collect::<HashSet<isize>>()
    };

    for a in -1000..1001 {
        for b in -1000..1001 {
            for n in 0.. {
                let pc = n * n + a * n + b;
                if !primes.contains(&pc) {
                    if n > best {
                        println!("Best is {}", best);
                        a_best = a;
                        b_best = b;
                        best = n;
                    }
                    break;
                }
            }
        }
    }

    answer!("{}", a_best * b_best);
}
