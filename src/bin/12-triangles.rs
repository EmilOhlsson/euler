#[macro_use]
extern crate cch_utils;

use cch_utils::math::divisors;
use cch_utils::series::Triangles;

fn main() {
    let t = Triangles::new();
    answer!(
        "{}",
        t.into_iter()
            .find(|v| divisors(v).len() >= 500 - 1)
            .unwrap()
    );
}
