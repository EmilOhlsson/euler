#[macro_use]
extern crate project_euler;
extern crate num;

use num::bigint::BigUint;
use project_euler::series::Fibonacci;

fn main() {
    let fib = Fibonacci::new(BigUint::from(0u32), BigUint::from(1u32));
    answer!(
        "{}",
        fib.into_iter()
            .enumerate()
            .skip_while(|&(_, ref f)| f.to_str_radix(10).len() <= 1_000)
            .next()
            .unwrap()
            .0 - 3
    );
}
