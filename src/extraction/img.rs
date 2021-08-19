use super::{Extractor, HashResult};
use crate::color::{average, Region, WhitePoint};
use crate::persist::ExtractionConfig;
use crate::theme::{Colors, Palette, Theme};
use color_eyre::eyre::{Result, WrapErr};
use log::{info, trace};
use num_traits::{Float, Signed};
use palette::{FromColor, IntoColor, Srgb};
use rayon::prelude::*;
use std::hash::{Hash, Hasher};
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Opt {
    /// Source image to extract theme from.
    path: std::path::PathBuf,

    /// Color "mode" preference.
    ///
    /// If no preference is specified, the theme closest to the source image will be used.
    #[structopt(short, long)]
    preference: Option<Preference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Preference {
    Dark,
    Light,
}

impl std::str::FromStr for Preference {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            _ => Err("Invalid preference"),
        }
    }
}

impl Extractor for Opt {
    fn hash<H: Hasher>(&self, _config: &ExtractionConfig, state: &mut H) -> Result<HashResult> {
        let img = image::io::Reader::open(&self.path)
            .wrap_err("Failed to read image file")?
            .with_guessed_format()?
            .decode()
            .wrap_err("Failed to decode image")?
            .into_rgb8();

        img.hash(state);
        self.preference.hash(state);
        // FIXME: Also hash extraction target.

        Ok(HashResult::Finished)
    }

    fn extract(&self, config: &ExtractionConfig) -> Result<Theme> {
        info!("Reading and decoding image...");
        let img = image::io::Reader::open(&self.path)
            .wrap_err("Failed to read image file")?
            .with_guessed_format()
            .wrap_err("Failed to guess image format")?
            .decode()
            .wrap_err("Failed to decode image")?
            .into_rgb8();

        Ok(Theme {
            wallpaper: Some(self.path.clone()),
            colors: {
                info!("Splitting and averaging colors...");
                // TODO: Test other chunking strategies for performance.
                self.gen_colors(
                    img.par_chunks(3).map(|pix| {
                        Srgb::from_components((
                            pix[0] as f32 / 255.0,
                            pix[1] as f32 / 255.0,
                            pix[2] as f32 / 255.0,
                        ))
                    }),
                    config.target.map(Into::into),
                )
            },
        })
    }
}

impl Opt {
    fn gen_palette<I, C, R>(&self, cols: I, regs: &Palette<Region<C>>) -> Palette<(R, usize)>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Clone + FromColor<WhitePoint, C>,
    {
        let split = regs.split(cols).map(IntoParallelIterator::into_par_iter);
        trace!("Averaging image colors...");
        let partial = split
            .map(|part| (part.len(), average::lab_centroid::<_, _, R>(part)))
            .map(|(count, col)| (col, count));

        let extrapolate = |targ: &Region<C>| {
            R::from_lab((targ.start().into_lab() + targ.end().into_lab()) / C::from(2).unwrap())
        };

        partial
            .zip(regs.as_ref())
            .map(|((col, count), reg)| (col.unwrap_or_else(|| extrapolate(reg)), count))
    }

    fn gen_colors<I, C, R>(&self, cols: I, regs: Palette<Region<C>>) -> Colors<R>
    where
        I: Clone + ParallelIterator,
        I::Item: Clone + IntoColor<WhitePoint, C>,
        C: Send + Sync + palette::Component + Float + Signed,
        R: Copy + FromColor<WhitePoint, C>,
    {
        let pal = self.gen_palette(cols, &regs);

        trace!("Finding and sorting accents...");
        let mut accents = pal.accents();
        accents.sort_by(|(_, n1), (_, n2)| n2.cmp(n1));

        Colors {
            palette: pal.map(|(c, _)| c),
            accents: accents.iter().map(|(c, _)| *c).collect(),

            foreground: match self.preference {
                Some(Preference::Light) => pal.black.0,
                Some(Preference::Dark) => pal.white.0,

                // Use least common from black or white, or white
                None if pal.white.1 > pal.black.1 => pal.black.0,
                _ => pal.white.0,
            },
            background: match self.preference {
                Some(Preference::Light) => pal.white.0,
                Some(Preference::Dark) => pal.black.0,

                // Use most common from black or white, or black
                None if pal.white.1 > pal.black.1 => pal.white.0,
                _ => pal.black.0,
            },
        }
    }
}

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
