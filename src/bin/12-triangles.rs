#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::math::divisors;
use coding_challenge_utils::series::Triangles;

fn main() {
    let t = Triangles::new();
    answer!(
        "{}",
        t.into_iter()
            .find(|v| divisors(v).len() >= 500 - 1)
            .unwrap()
    );
}
