#[macro_use]
extern crate cch_utils;

use cch_utils::checkers::is_palindrome;
use std::cmp;

fn main() {
    let mut max_palindrome = 0;
    for x in 100..999 {
        for y in 100..999 {
            let v = x * y;
            if is_palindrome(&v) {
                max_palindrome = cmp::max(v, max_palindrome);
            }
        }
    }
    answer!("{}", max_palindrome);
}
