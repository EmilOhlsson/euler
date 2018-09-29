#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::math::factorize;

fn main() {
    let factors = factorize(600_851_475_143);
    answer!("{}", factors.last().unwrap());
}
