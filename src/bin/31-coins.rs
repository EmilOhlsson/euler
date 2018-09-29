#[macro_use]
extern crate coding_challenge_utils;

fn rec(coins: &Vec<usize>, i: usize, acc: usize, target: usize) -> usize {
    if let Some(coin) = coins.get(i) {
        if acc == target {
            1
        } else if acc > target {
            0
        } else {
            rec(coins, i, acc + coin, target) + rec(coins, i + 1, acc, target)
        }
    } else {
        0
    }
}

fn main() {
    answer!("{}", rec(&vec![200, 100, 50, 20, 10, 5, 2, 1], 0, 0, 200));
}
