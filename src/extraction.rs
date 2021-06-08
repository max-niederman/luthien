use crate::color::{Region, WhitePoint};
use crate::theme::{Colors, Palette};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModePreference {
    Dark,
    Light,
}

pub struct Generator {
    pub mode_preference: Option<ModePreference>,
    pub average_method: AverageMethod,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            mode_preference: None,
            average_method: AverageMethod::LabCentroid,
        }
    }
}

impl Generator {
    fn gen_palette<I, C, R>(&self, cols: I, regs: &Palette<Region<C>>) -> Palette<(R, usize)>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Clone + FromColor<WhitePoint, C>,
    {
        let split = regs.split(cols).map(IntoParallelIterator::into_par_iter);
        let partial = split
            .map(|part| (part.len(), self.average_method.average::<_, _, R>(part)))
            .map(|(count, col)| (col, count));

        let extrapolate = |targ: &Region<C>| {
            R::from_lab((targ.start().into_lab() + targ.end().into_lab()) / C::from(2).unwrap())
        };

        partial
            .zip(regs.as_ref())
            .map(|((col, count), reg)| (col.unwrap_or_else(|| extrapolate(reg)), count))
    }

    pub fn generate<I, C, R>(&self, cols: I, regs: Palette<Region<C>>) -> Colors<R>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Copy + FromColor<WhitePoint, C>,
    {
        let pal = self.gen_palette(cols, &regs);

        Colors {
            palette: pal.map(|(c, _)| c),
            accents: accents_sorted(pal).iter().map(|(c, _)| *c).collect(),

            foreground: match self.mode_preference {
                Some(ModePreference::Light) => pal.black.0,
                Some(ModePreference::Dark) => pal.white.0,

                // Use least common from black or white, or white
                None if pal.white.1 > pal.black.1 => pal.black.0,
                _ => pal.white.0,
            },
            background: match self.mode_preference {
                Some(ModePreference::Light) => pal.white.0,
                Some(ModePreference::Dark) => pal.black.0,

                // Use most common from black or white, or black
                None if pal.white.1 > pal.black.1 => pal.white.0,
                _ => pal.black.0,
            },
        }
    }
}

fn accents_sorted<T, N: std::cmp::Ord>(pal: Palette<(T, N)>) -> [(T, N); 6] {
    let mut accents = pal.accents();
    accents.sort_by(|(_, n1), (_, n2)| n2.cmp(n1));
    accents
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
