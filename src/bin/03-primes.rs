#[macro_use]
extern crate cch_utils;

use cch_utils::math::factorize;

fn main() {
    let factors = factorize(600_851_475_143);
    answer!("{}", factors.last().unwrap());
}
