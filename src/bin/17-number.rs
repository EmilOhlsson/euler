#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::representations::textualize;

fn main() {
    answer!("{}", (1..1001).map(|n| textualize(n).len()).sum::<usize>());
}
