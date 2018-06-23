#[macro_use]
extern crate cch_utils;

fn main() {
    let mut names = include_str!("input-22")
        .split(",")
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>();
    names.sort();
    let sum = names
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let wsum = w.as_bytes().iter().map(|c| c - 64u8).sum::<u8>() as usize;
            let res = (i + 1) * wsum;
            println!("{}: {} * {} = {}", w, (i + 1), wsum, res);
            res
        })
        .sum::<usize>();
    answer!("{}", sum);
}
