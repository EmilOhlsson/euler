#[macro_use]
extern crate project_euler;

use project_euler::math::divisors1;

fn main() {
    let mut sum = 0;
    for n in 1..10_001 {
        let s = divisors1(&n).iter().sum();
        if divisors1(&s).iter().sum::<usize>() == n && n != s {
            sum += n;
        }
    }
    answer!("{}", sum);
}
