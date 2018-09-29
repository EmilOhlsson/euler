#[macro_use]
extern crate coding_challenge_utils;

fn main() {
    let input = include_str!("input-08");
    let digs = input
        .lines()
        .flat_map(|s| s.chars())
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let ans = digs.windows(13)
        .map(|w| w.iter().fold(1, |a, v| a * v))
        .max()
        .unwrap();
    answer!("{}", ans);
}
