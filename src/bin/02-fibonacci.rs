#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Fibonacci;

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
