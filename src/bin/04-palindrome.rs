#[macro_use]
extern crate project_euler;

use std::cmp;

fn main() {
    let mut max_palindrome = 0;
    for x in 100..999 {
        for y in 100..999 {
            let v = x * y;
            let a = format!("{}", v);
            let b = a.chars().rev().collect::<String>();
            if b == a {
                max_palindrome = cmp::max(v, max_palindrome);
            }
        }
    }
    answer!("{}", max_palindrome);
}
