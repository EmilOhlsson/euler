#[macro_use]
extern crate cch_utils;

use cch_utils::sets::permutations;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ps = permutations(&mut v);
    ps.sort();
    answer!(
        "{}",
        ps[1_000_000 - 1]
            .iter()
            .map(usize::to_string)
            .collect::<String>()
    );
}
