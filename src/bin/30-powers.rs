#[macro_use]
extern crate project_euler;

use project_euler::series::Digits;

fn main() {
    let mut sum = 0;
    for i in 2..500_000 {
        if Digits::new(i).map(|n| n * n * n * n * n).sum::<usize>() == i {
            sum += i;
        }
    }
    answer!("{}", sum);
}
