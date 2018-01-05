#[macro_use]
extern crate project_euler;

use std::cmp::max;

fn getval(mtx: &Vec<Vec<isize>>, r: isize, c: isize) -> isize {
    if let Some(row) = mtx.get(r as usize) {
        if let Some(&val) = row.get(c as usize) {
            return val;
        }
    }
    0
}

fn find_max(mtx: &Vec<Vec<isize>>) -> isize {
    let len = mtx.len() as isize;
    let mut mx = 0;

    for r in 0..len {
        for c in 0..len {
            let ver = (0..4).map(|o| getval(mtx, r - o, c)).fold(1, |a, v| a * v);
            let hor = (0..4).map(|o| getval(mtx, r, c - o)).fold(1, |a, v| a * v);
            let dlft = (0..4).map(|o| getval(mtx, r - o, c - o)).fold(
                1,
                |a, v| a * v,
            );
            let drght = (0..4).map(|o| getval(mtx, r - o, c + o)).fold(
                1,
                |a, v| a * v,
            );
            let m = max(ver, max(hor, max(dlft, drght)));
            mx = max(m, mx);

        }
    }

    mx
}

fn main() {
    let input = include_str!("input-11");
    let nums = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| t.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();
    answer!("{}", find_max(&nums));
}