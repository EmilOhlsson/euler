use std::ops::{Add, Div, Mul, Rem, Sub};

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
