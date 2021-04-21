use num_traits::{NumOps, Signed};
use std::ops;

#[derive(Clone, Copy)]
pub struct Space<M: Copy> {
    modulus: M,
}

impl<M: Copy> Space<M> {
    pub fn new(modulus: M) -> Self {
        Self { modulus }
    }

    pub fn modulus(&self) -> &M {
        &self.modulus
    }

    pub fn modulo<N>(&self, n: N) -> N
    where
        N: ops::Add<M, Output = N> + ops::Rem<M, Output = N>,
    {
        ((n % self.modulus) + self.modulus) % self.modulus
    }

    pub fn sub<N>(&self, n1: N, n2: N) -> N
    where
        N: ops::Sub,
        <N as ops::Sub>::Output: ops::Rem<M, Output = N>,
    {
        (n1 - n2) % self.modulus
    }

    pub fn dist<N>(&self, n1: N, n2: N) -> N
    where
        N: Copy
            + PartialOrd
            + Signed
            + ops::Sub<Output = N>
            + ops::Sub<M, Output = N>
            + ops::Rem<M, Output = N>
            + ops::Add<M, Output = N>,
    {
        let p = self.modulo(n2 - n1);
        let n = p - self.modulus;

        // once it's stabilized, we can use `std::cmp::min_by_key`
        if n.abs() < p.abs() {
            n
        } else {
            p
        }
    }

    pub fn range<N>(&self, start: N, end: N) -> Range<M, N>
    where
        N: Copy + PartialOrd + NumOps + NumOps<M> + Signed,
    {
        Range::new(self.clone(), start, end)
    }
}

#[derive(Clone, Copy)]
pub struct Range<M, N>
where
    M: Copy,
    N: PartialOrd + ops::Sub,
    <N as ops::Sub>::Output: Copy,
{
    pub space: Space<M>,
    start: N,
    length: <N as ops::Sub>::Output,
}

impl<M, N> Range<M, N>
where
    M: Copy,
    N: Copy + PartialOrd + NumOps + NumOps<M> + Signed,
{
    pub fn new(space: Space<M>, start: N, end: N) -> Self {
        Self {
            space,
            length: space.dist(start, end),
            start: space.modulo(start),
        }
    }

    pub fn contains(&self, n: N) -> bool {
        let n = self.space.modulo(n);
        let d = self.space.dist(self.start, n);

        if self.length.is_negative() {
            !d.is_positive() && d >= self.length
        } else {
            !d.is_negative() && d <= self.length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Range, Space};

    #[test]
    fn modulo_operation() {
        let space = Space::new(10);

        assert_eq!(space.modulo(2), 2);
        assert_eq!(space.modulo(12), 2);
        assert_eq!(space.modulo(-2), 8);
        assert_eq!(space.modulo(0), 0);
    }

    #[test]
    fn modular_distance() {
        let space = Space::new(10);

        assert_eq!(space.dist(2, 1), -1);
        assert_eq!(space.dist(1, 2), 1);

        assert_eq!(space.dist(9, 1), 2);
        assert_eq!(space.dist(1, 9), -2);

        assert_eq!(space.dist(1, 1), 0);
        assert_eq!(space.dist(10, 0), 0);
        assert_eq!(space.dist(20, 0), 0);
        assert_eq!(space.dist(1, -9), 0);

        assert_eq!(space.dist(22, 1), -1);
    }

    #[test]
    fn modular_range() {
        let range = Range::new(Space::new(360), 90, 270);

        assert!(range.contains(180));
        assert!(range.contains(90));
        assert!(range.contains(480));
        assert!(range.contains(-480));
        assert!(range.contains(-90));

        assert!(!range.contains(0));
        assert!(!range.contains(45));
        assert!(!range.contains(-45));
        assert!(!range.contains(405));
        assert!(!range.contains(-405));

        let range = Range::new(Space::new(360), 270, 90);
        assert!(!range.contains(180));
        assert!(range.contains(0));
    }
}
