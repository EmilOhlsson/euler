#[macro_use]
extern crate project_euler as euler;

use euler::series::Collatz;

fn main() {
    answer!(
        "{}",
        (1..1_000_000)
            .map(|i| {
                (
                    Collatz::new(i).into_iter().take_while(|&v| v != 1).count() + 1,
                    i,
                )
            })
            .max()
            .unwrap()
            .1
    );
}
