use num_traits::{Float, Signed};
use palette::{FromColor, IntoColor};
use std::ops::Add;

pub fn lab<I, C, R>(iter: I) -> Option<R>
where
    I: ExactSizeIterator,
    I::Item: IntoColor<super::WhitePoint, C>,
    C: palette::Component + Float + Signed,
    R: FromColor<super::WhitePoint, C>,
{
    let len = iter.len();

    iter.map(IntoColor::into_lab)
        .reduce(Add::add)
        .map(|c| c / C::from(len).unwrap())
        .map(FromColor::from_lab)
}

#[cfg(test)]
mod tests {
    use palette::Lab;

    #[test]
    fn lab() {
        assert_eq!(
            super::lab(
                [Lab::new(1.0, 2.0, 3.0), Lab::new(1.0, 2.0, 3.0)]
                    .iter()
                    .cloned()
            ),
            Some(Lab::new(1.0, 2.0, 3.0))
        );
        assert_eq!(
            super::lab(
                [Lab::new(1.0, 2.0, 3.0), Lab::new(2.0, 3.0, 4.0)]
                    .iter()
                    .cloned()
            ),
            Some(Lab::new(1.5, 2.5, 3.5))
        );
    }
}
