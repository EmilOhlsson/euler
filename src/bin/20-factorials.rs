#[macro_use]
extern crate project_euler;
extern crate num;

use project_euler::big::{digitsum, faculty};

fn main() {
    let sum = digitsum(&faculty(100));
    answer!("{}", sum);
}
