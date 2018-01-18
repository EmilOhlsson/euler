#[macro_use]
extern crate project_euler;

use project_euler::representations::textualize;

fn main() {
    answer!("{}", (1..1001).map(|n| textualize(n).len()).sum::<usize>());
}
