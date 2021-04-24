use crate::mod_arith;
use num_traits::{Float, Signed};
use palette::{encoding::Srgb, white_point, IntoColor};
use std::ops::RangeInclusive;

mod average;

#[derive(Clone)]
struct ColorRegion<T>
where
    T: palette::Component + Float,
{
    pub hue: mod_arith::Range<T, T>,
    pub saturation: RangeInclusive<T>,
    pub lightness: RangeInclusive<T>,
}

impl<T> ColorRegion<T>
where
    T: palette::Component + Float + Signed,
{
    pub fn new(
        hue: RangeInclusive<T>,
        saturation: RangeInclusive<T>,
        lightness: RangeInclusive<T>,
    ) -> Self {
        Self {
            hue: mod_arith::Space::new(T::from(360.0).unwrap()).range(*hue.start(), *hue.end()),
            saturation,
            lightness,
        }
    }

    pub fn contains<C>(&self, color: C) -> bool
    where
        C: IntoColor<white_point::D65, T>,
    {
        let hsv = color.into_hsl::<Srgb>();

        self.hue.contains(hsv.hue.to_degrees())
            && self.saturation.contains(&hsv.saturation)
            && self.lightness.contains(&hsv.lightness)
    }
}

#[cfg(test)]
mod tests {
    use super::ColorRegion;
    use palette::{encoding::Srgb, Hsl, IntoColor};

    #[test]
    fn region_contains() {
        let region = ColorRegion::new(0.0..=180.0, 0.0..=0.5, 0.0..=0.5);

        assert!(region.contains(Hsl::new(0.0, 0.0, 0.0)));
        assert!(region.contains(Hsl::new(180.0, 0.5, 0.5)));

        assert!(!region.contains(Hsl::new(270.0, 0.0, 0.0)));
        assert!(!region.contains(Hsl::new(0.0, 1.0, 0.0)));
        assert!(!region.contains(Hsl::new(0.0, 0.0, 1.0)));

        assert!(region.contains(Hsl::new(180.0, 0.5, 0.5).into_rgb::<Srgb>()));
    }
}
