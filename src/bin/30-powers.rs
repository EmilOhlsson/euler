#[macro_use]
extern crate cch_utils;

use cch_utils::series::Digits;

fn main() {
    let mut sum = 0;
    for i in 2..500_000 {
        if Digits::new(i).map(|n| n * n * n * n * n).sum::<usize>() == i {
            sum += i;
        }
    }
    answer!("{}", sum);
}
