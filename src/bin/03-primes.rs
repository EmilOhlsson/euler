#[macro_use]
extern crate project_euler;

use project_euler::math::factorize;

fn main() {
    let factors = factorize(600_851_475_143);
    answer!("{}", factors.last().unwrap());
}
