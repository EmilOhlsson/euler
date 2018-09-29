#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Digits;

fn main() {
    let mut sum = 0;
    for i in 2..500_000 {
        if Digits::new(i).map(|n| n * n * n * n * n).sum::<usize>() == i {
            sum += i;
        }
    }
    answer!("{}", sum);
}
