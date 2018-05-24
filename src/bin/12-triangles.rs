#[macro_use]
extern crate project_euler;

use project_euler::math::divisors;
use project_euler::series::Triangles;

fn main() {
    let t = Triangles::new();
    answer!(
        "{}",
        t.into_iter()
            .find(|v| divisors(v).len() >= 500 - 1)
            .unwrap()
    );
}
