use crate::color::{Region, WhitePoint};
use crate::theme::Palette;
use num_traits::{Float, Signed};
use palette::{FromColor, IntoColor};
use rayon::prelude::*;

impl<C> Palette<Region<C>>
where
    C: palette::Component + Float + Signed,
{
    fn split<I>(&self, iter: I) -> Palette<Vec<I::Item>>
    where
        C: Sync,
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
    {
        self.clone()
            .map(|reg| iter.clone().filter(|c| reg.contains(c.clone())).collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AverageMethod {
    LabCentroid,
}

impl AverageMethod {
    fn average<I, C, R>(&self, iter: I) -> Option<R>
    where
        I: IndexedParallelIterator,
        I::Item: IntoColor<WhitePoint, C>,
        C: Send + palette::Component + Float + Signed,
        R: FromColor<WhitePoint, C>,
    {
        use crate::color::average::*;

        match self {
            Self::LabCentroid => lab_centroid(iter),
        }
    }
}

pub struct GenerationOpts {
    pub average_method: AverageMethod,
}

impl Default for GenerationOpts {
    fn default() -> Self {
        Self {
            average_method: AverageMethod::LabCentroid,
        }
    }
}

impl GenerationOpts {
    pub fn generate<I, C, R>(&self, cols: I, regs: Palette<Region<C>>) -> Palette<R>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: FromColor<WhitePoint, C>,
    {
        regs.split(cols)
            .map(|part| self.average_method.average(part.into_par_iter()))
            .map(Option::unwrap) // TODO: Handle images missing necessary colors.
    }
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::theme::Palette;
    use rayon::prelude::*;

    #[test]
    fn color_split() {
        use crate::persist;
        use palette::Hsl;

        let regs: Palette<color::Region<f32>> =
            Palette::<persist::RegionConfig>::default().map(Into::into);

        assert_eq!(
            regs.split([Hsl::new(0.0, 0.0, 0.0)].par_iter().cloned()),
            Palette {
                black: vec![Hsl::new(0.0, 0.0, 0.0)],
                ..Default::default()
            }
        );
        assert_eq!(
            regs.split([Hsl::new(0.0, 1.0, 0.5)].par_iter().cloned()),
            Palette {
                red: vec![Hsl::new(0.0, 1.0, 0.5)],
                ..Default::default()
            }
        );
    }
}
