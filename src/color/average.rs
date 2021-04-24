use palette::{FromColor, IntoColor};
use std::ops::Add;

pub fn lab<I, R>(iter: I) -> Option<R>
where
    I: ExactSizeIterator,
    I::Item: IntoColor,
    R: FromColor,
{
    let len = iter.len();

    iter.map(IntoColor::into_lab)
        .reduce(Add::add)
        .map(|c| c / len as f32)
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
