#[macro_use]
extern crate cch_utils;

use cch_utils::representations::textualize;

fn main() {
    answer!("{}", (1..1001).map(|n| textualize(n).len()).sum::<usize>());
}
