#[macro_use]
extern crate coding_challenge_utils;

use std::cmp::max;
use std::collections::HashMap;

fn find_max(
    pyramid: &Vec<Vec<usize>>,
    cached: &mut HashMap<(usize, usize), usize>,
    row: usize,
    col: usize,
) -> usize {
    if row >= 15 {
        return 0;
    }
    if let Some(&v) = cached.get(&(row, col)) {
        return v;
    }

    let sum = pyramid[row][col]
        + max(
            find_max(pyramid, cached, row + 1, col),
            find_max(pyramid, cached, row + 1, col + 1),
        );
    cached.insert((row, col), sum);

    sum
}

fn main() {
    let input = include_str!("input-18")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| t.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    answer!("{}", find_max(&input, &mut HashMap::new(), 0, 0));
}
