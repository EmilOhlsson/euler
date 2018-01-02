extern crate project_euler;

use project_euler::answer;

fn main() {
    answer(format!(
        "{}",
        (0..1000)
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .sum::<usize>()
    ));
}
