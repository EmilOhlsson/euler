extern crate num;
#[macro_use]
extern crate cch_utils;

use num::bigint::BigUint;
use std::collections::HashSet;

fn bigpow(a: usize, b: usize) -> BigUint {
    let mut res = BigUint::from(a);
    for _ in 1..b {
        res *= a;
    }
    return res;
}

fn main() {
    let mut set = HashSet::new();
    for a in 2..101 {
        for b in 2..101 {
            set.insert(bigpow(a, b));
        }
    }
    answer!("{}", set.len());
}
