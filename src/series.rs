use std::ops::{Add, Div, Rem};
use std::cmp::Ord;

use constants;

pub struct Fibonacci<T>
where
    T: Add<Output = T> + Copy,
{
    p0: T,
    p1: T,
}

impl<T> Fibonacci<T>
where
    T: Add<Output = T> + Copy,
{
    pub fn new(p0: T, p1: T) -> Fibonacci<T> {
        Fibonacci { p0: p0, p1: p1 }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let nxt = self.p0 + self.p1;
        self.p0 = self.p1;
        self.p1 = nxt;
        Some(nxt)
    }
}

pub struct Primes<T>
where
    T: Rem<Output = T> + Copy + From<usize>,
{
    produced: usize,
    primes: Vec<T>,
}

impl<T> Primes<T>
where
    T: Rem<Output = T> + Copy + From<usize>,
{
    pub fn new() -> Primes<T> {
        Primes {
            produced: 0,
            primes: constants::PRIMES
                .iter()
                .map(|&p| T::from(p))
                .collect::<Vec<_>>(),
        }
    }
}

impl<T> Iterator for Primes<T>
where
    T: Add<Output = T>
        + Rem<Output = T>
        + Div<Output = T>
        + Ord
        + Copy
        + From<usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if let Some(&p) = self.primes.get(self.produced) {
            result = Some(p);
        } else {
            let zero = T::from(0);
            let inc = T::from(2);
            let mut candidate = *self.primes.last().unwrap() + inc;
            while self.primes
                .iter()
                .take_while(|&p| *p < candidate / inc + inc)
                .find(|&p| candidate % *p == zero)
                .is_some()
            {
                candidate = candidate + inc;
            }
            result = Some(candidate);
        }
        self.produced += 1;
        result
    }
}
