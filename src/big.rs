extern crate num;

use big::num::bigint::{BigInt, ToBigInt};
use big::num::ToPrimitive;
use big::num::{Zero, One};

pub fn digitsum(n: &BigInt) -> isize {
    let mut v = n.clone();
    let mut sum: isize = 0;
    while &v > &Zero::zero() {
        sum += (&v % 10isize).to_isize().unwrap();
        v = v / 10;
    }
    sum
}

pub fn faculty(n: isize) -> BigInt {
    (2..(n + 1)).fold(One::one(), |a, v| a * v)
}

pub fn big(n: isize) -> BigInt {
    n.to_bigint().unwrap()
}

#[test]
fn test_digitsum() {
    assert_eq!(digitsum(&big(123)), 6);
}
