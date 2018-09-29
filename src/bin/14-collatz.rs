#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::series::Collatz;

fn main() {
    answer!(
        "{}",
        (1..1_000_000)
            .map(|i| (
                Collatz::new(i).into_iter().take_while(|&v| v != 1).count() + 1,
                i,
            ))
            .max()
            .unwrap()
            .1
    );
}
