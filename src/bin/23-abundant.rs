#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::math::divisors_proper;
use std::collections::HashSet;

fn main() {
    let mut abundants = HashSet::new();
    let mut sum = 0;
    for n in 1..(28_123 + 1) {
        if divisors_proper(&n).iter().sum::<usize>() > n {
            abundants.insert(n);
        }
        let mut found = false;
        for t1 in &abundants {
            if abundants.contains(&(n - t1)) {
                found = true;
                break;
            }
        }
        if !found {
            sum += n;
        }
    }
    answer!("{}", sum);
}
