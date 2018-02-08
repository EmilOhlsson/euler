#[macro_use]
extern crate project_euler;

fn main() {
    let diags = (1..501)
        .enumerate()
        .map(|(i, n)| (2 * (i + 1), 2 * n + 1))
        .map(|(i, n)| (n * n, n * n - i, n * n - 2 * i, n * n - 3 * i))
        .fold((0, 0, 0, 0), |a, i| {
            (a.0 + i.0, a.1 + i.1, a.2 + i.2, a.3 + i.3)
        });
    answer!("{}", diags.0 + diags.1 + diags.2 + diags.3 + 1);
}
