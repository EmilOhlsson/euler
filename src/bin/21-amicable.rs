#[macro_use]
extern crate project_euler;

use project_euler::math::divisors_proper;

fn main() {
    let mut sum = 0;
    for n in 1..10_001 {
        let s = divisors_proper(&n).iter().sum();
        if divisors_proper(&s).iter().sum::<usize>() == n && n != s {
            sum += n;
        }
    }
    answer!("{}", sum);
}
