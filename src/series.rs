use std::ops::Add;

pub struct Fibonacci<T>
where
    T: Add + Copy,
{
    p0: T,
    p1: T,
}

impl<T> Fibonacci<T>
where
    T: Add + Copy,
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
