use num_traits::{Float, Signed};
use palette::{FromColor, IntoColor};
use rayon::prelude::*;
use std::ops::Add;

pub fn lab_centroid<I, C, R>(iter: I) -> Option<R>
where
    I: IndexedParallelIterator,
    I::Item: IntoColor<super::WhitePoint, C>,
    C: Send + palette::Component + Float + Signed,
    R: FromColor<super::WhitePoint, C>,
{
    let len = iter.len();
    iter.map(IntoColor::into_lab)
        .reduce_with(Add::add)
        .map(|sum| sum / C::from(len).unwrap())
        .map(FromColor::from_lab)
}

#[cfg(test)]
mod tests {
    use palette::Lab;
    use rayon::prelude::*;

    #[test]
    fn lab_centroid() {
        assert_eq!(
            super::lab_centroid(
                [Lab::new(1.0, 2.0, 3.0), Lab::new(1.0, 2.0, 3.0)]
                    .par_iter()
                    .cloned()
            ),
            Some(Lab::new(1.0, 2.0, 3.0))
        );
        assert_eq!(
            super::lab_centroid(
                [Lab::new(1.0, 2.0, 3.0), Lab::new(2.0, 3.0, 4.0)]
                    .par_iter()
                    .cloned()
            ),
            Some(Lab::new(1.5, 2.5, 3.5))
        );
    }
}
