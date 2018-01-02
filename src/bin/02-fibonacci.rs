#[macro_use]
extern crate project_euler;

use project_euler::series::Fibonacci;

fn main() {
    let fib = Fibonacci::new(0, 1);
    answer!(
        "{}",
        fib.into_iter()
            .take_while(|&f| f < 4_000_000)
            .filter(|f| f % 2 == 0)
            .sum::<usize>()
    );
}
