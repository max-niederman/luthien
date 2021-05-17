use crate::color::{Region, WhitePoint};
use crate::theme::{Palette, Colors};
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

pub struct Generator {
    pub average_method: AverageMethod,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            average_method: AverageMethod::LabCentroid,
        }
    }
}

impl Generator {
    fn gen_palette<I, C, R>(&self, cols: I, regs: Palette<Region<C>>) -> Palette<(R, usize)>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Clone + FromColor<WhitePoint, C>,
    {
        let split = regs.split(cols).map(IntoParallelIterator::into_par_iter);

        split
            .map(|part| (part.len(), self.average_method.average::<_, _, R>(part)))
            .map(|(a, l)| (l.unwrap_or_else(|| todo!("handle images missing necessary colors")), a))
    }

    pub fn generate<I, C, R>(&self, cols: I, regs: Palette<Region<C>>) -> Colors<R>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Copy + FromColor<WhitePoint, C>,
    {
        // Generate the palette of colors.
        let pal = self.gen_palette(cols, regs);
        
        // Get accents by color prevalence.
        let mut accents = [pal.red, pal.green, pal.yellow, pal.blue, pal.purple, pal.cyan];
        accents.sort_by_key(|(_, c)| *c);
        accents.reverse();

        Colors {
            palette: pal.map(|(c, _)| c),
            accents: accents.iter().map(|(c, _)| *c).collect(),
            // TODO: Intelligently decide foreground and background.
            foreground: pal.white.0,
            background: pal.black.0,
        }
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
