use num_traits::{NumOps, Signed};
use std::ops;

#[derive(Clone, Copy)]
pub struct Space<M> {
    pub modulus: M,
}

impl<M> Space<M> {
    pub const fn new(modulus: M) -> Self {
        Self { modulus }
    }

    pub fn modulo<N>(&self, n: N) -> N
    where
        M: Copy,
        N: ops::Add<M, Output = N> + ops::Rem<M, Output = N>,
    {
        ((n % self.modulus) + self.modulus) % self.modulus
    }

    pub fn dist_pos<N>(&self, n1: N, n2: N) -> N
    where
        M: Copy,
        N: Copy + ops::Sub<Output = N> + ops::Rem<M, Output = N> + ops::Add<M, Output = N>,
    {
        (self.modulo(n2) + self.modulus - self.modulo(n1)) % self.modulus
    }

    pub fn range<N>(&self, start: N, end: N) -> Range<M, N>
    where
        M: Copy,
        N: Copy + PartialOrd + NumOps + NumOps<M> + Signed,
    {
        Range::new(*self, start, end)
    }
}

#[derive(Clone, Copy)]
pub struct Range<M, N>
where
    M: Copy,
    N: PartialOrd,
{
    pub space: Space<M>,
    start: N,
    length: N,
}

impl<M, N> Range<M, N>
where
    M: Copy,
    N: Copy + PartialOrd + NumOps + NumOps<M> + Signed,
{
    pub fn new(space: Space<M>, start: N, end: N) -> Self {
        Self {
            space,
            start: space.modulo(start),
            length: space.dist_pos(start, end),
        }
    }

    pub fn contains(&self, n: N) -> bool {
        let n = self.space.modulo(n);

        self.length.is_zero() || self.space.dist_pos(self.start, n) <= self.length
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

        assert_eq!(space.dist_pos(2, 1), 9);
        assert_eq!(space.dist_pos(1, 2), 1);

        assert_eq!(space.dist_pos(9, 1), 2);
        assert_eq!(space.dist_pos(1, 9), 8);

        assert_eq!(space.dist_pos(1, 1), 0);
        assert_eq!(space.dist_pos(10, 0), 0);
        assert_eq!(space.dist_pos(20, 0), 0);
        assert_eq!(space.dist_pos(1, -9), 0);

        assert_eq!(space.dist_pos(22, 1), 9);
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

        let range = Range::new(Space::new(360), 0, 360);
        assert!(range.contains(0));
        assert!(range.contains(180));
        assert!(range.contains(360));
    }
}
