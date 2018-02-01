use series::Primes;
use std::ops::{Add, Div, DivAssign, Mul, Rem, Sub};

pub fn collatz<T>(n: T) -> T
where
    T: Copy
        + Rem<usize, Output = T>
        + Div<usize, Output = T>
        + Add<usize, Output = T>
        + Mul<usize, Output = T>
        + PartialEq<usize>,
{
    if n % 2 == 0 { n / 2 } else { n * 3 + 1 }
}

pub fn digitsum<T>(n: &T) -> T
where
    T: Add<Output = T> + Rem<usize, Output = T> + Div<usize, Output = T> + Ord + Clone + From<usize>,
{
    let mut v = n.clone();
    let mut sum: T = T::from(0);
    let zero = T::from(0);
    while &v > &zero {
        sum = sum + (v.clone() % 10);
        v = v / 10;
    }
    sum
}

pub fn faculty<T>(n: &T) -> T
where
    T: Sub<usize, Output = T> + Mul<Output = T> + Ord + Clone + From<usize>,
{
    let one = T::from(1);
    let mut result: T = T::from(1);
    let mut i = n.clone();

    while &i > &one {
        result = result * i.clone();
        i = i - 1;
    }

    result
}

pub fn factorize<T>(mut n: T) -> Vec<usize>
where
    T: DivAssign<usize> + PartialEq<usize> + Rem<usize, Output = usize> + From<usize> + Copy,
{
    let primes: Primes<usize> = Primes::new();
    let mut factors = Vec::new();

    for p in primes {
        if n % p == 0 {
            factors.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        if n == 1 {
            break;
        }
    }

    factors
}

pub fn divisors(n: &usize) -> Vec<usize> {
    (2..(*n / 2 + 2))
        .filter(|d| *n % d == 0)
        .collect::<Vec<usize>>()
}

pub fn divisors1(n: &usize) -> Vec<usize> {
    (1..(*n / 2 + 2))
        .filter(|d| *n % d == 0)
        .collect::<Vec<usize>>()
}
