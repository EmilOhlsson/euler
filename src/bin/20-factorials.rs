extern crate project_euler;
extern crate num;

use project_euler::big::{digitsum, faculty};
use project_euler::answer;

fn main() {
    let sum = digitsum(&faculty(100));
    answer(format!("{}", sum));
}
