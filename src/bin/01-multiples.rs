#[macro_use]
extern crate project_euler;

fn main() {
    answer!(
        "{}",
        (0..1000)
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .sum::<usize>()
    );
}
