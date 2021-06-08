use crate::mod_arith;
use num_traits::{Float, Signed};
use palette::{encoding::Srgb, Hsl, IntoColor};
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Region<T>
where
    T: palette::Component + Float,
{
    pub hue: mod_arith::Range<T, T>,
    pub saturation: RangeInclusive<T>,
    pub lightness: RangeInclusive<T>,
}

impl<T> Region<T>
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
        C: IntoColor<super::WhitePoint, T>,
    {
        let hsl = color.into_hsl::<Srgb>();

        self.hue.contains(hsl.hue.to_degrees())
            && self.saturation.contains(&hsl.saturation)
            && self.lightness.contains(&hsl.lightness)
    }

    pub fn start(&self) -> Hsl<Srgb, T> {
        Hsl::new(
            self.hue.start(),
            *self.saturation.start(),
            *self.lightness.start(),
        )
    }

    pub fn end(&self) -> Hsl<Srgb, T> {
        Hsl::new(
            self.hue.length(),
            *self.saturation.end(),
            *self.lightness.end(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Region;
    use palette::{encoding::Srgb, Hsl, IntoColor};

    #[test]
    fn region_contains() {
        let region = Region::new(0.0..=180.0, 0.0..=0.5, 0.0..=0.5);

        assert!(region.contains(Hsl::new(0.0, 0.0, 0.0)));
        assert!(region.contains(Hsl::new(180.0, 0.5, 0.5)));

        assert!(!region.contains(Hsl::new(270.0, 0.0, 0.0)));
        assert!(!region.contains(Hsl::new(0.0, 1.0, 0.0)));
        assert!(!region.contains(Hsl::new(0.0, 0.0, 1.0)));

        assert!(region.contains(Hsl::new(180.0, 0.5, 0.5).into_rgb::<Srgb>()));
    }
}
