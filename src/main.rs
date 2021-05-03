use log::{info, trace};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

mod color;
mod mod_arith;
mod palette_gen;
mod persist;
mod plugin;
mod theme;

#[derive(Debug, PartialEq, Eq, Clone, StructOpt)]
#[structopt(name = "luthien")]
struct Opt {
    /// Image to use for theming.
    ///
    /// If colors are specified, the image will be recolored with those colors (not yet
    /// implemented). Otherwise, the
    /// colors will be extracted from the image.
    #[structopt(short, long)]
    image: Option<PathBuf>,

    /// Base theme to use.
    ///
    /// This argument is required unless the image option is given, in which case the theme
    /// will be derived from the image.
    #[structopt(required_unless = "image")]
    theme: Option<PathBuf>,

    /// Skip applying the theme.
    #[structopt(short = "s", long = "skip", parse(from_flag = std::ops::Not::not))]
    apply: bool,

    /// Output file for the used theme.
    #[structopt(short, long)]
    output: Option<PathBuf>,
}

fn get_theme(opt: Opt, paths: persist::Paths, config: persist::Config) -> io::Result<theme::Theme> {
    if let Some(path) = &opt.theme {
        info!("Reading theme from filesystem.");
        paths.get_theme(path.clone())
    } else if let Some(path) = opt.image {
        info!("Generating theme from image.");

        trace!("Reading and decoding image.");
        let img = image::io::Reader::open(&path)?
            .with_guessed_format()?
            .decode()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .into_rgb8();

        Ok(theme::Theme {
            background: path,
            colors: {
                use palette::Srgb;

                trace!("Generating color palette.");
                let generator = palette_gen::GenerationOpts::default();
                generator.generate(
                    img.pixels().par_bridge().map(|pix| {
                        Srgb::from_components((pix.0[0] as f32, pix.0[1] as f32, pix.0[2] as f32))
                    }),
                    config.colors.map(Into::into),
                )
            },
        })
    } else {
        unreachable!()
    }
}

fn main() -> io::Result<()> {
    pretty_env_logger::init();

    trace!("Parsing opts.");
    let opt = Opt::from_args();

    trace!("Loading configuration.");
    let paths = persist::Paths::default();
    let config = paths.get_config()?;

    info!("Loading or generating theme.");
    let theme = get_theme(opt, paths, config)?;

    println!("{}", theme.colors);
    println!("{}", toml::to_string_pretty(&theme).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Opt;
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[test]
    fn opt_parsing() {
        // Should parse
        assert_eq!(
            Opt::from_iter(&["test", "theme.toml"]),
            Opt {
                image: None,
                theme: Some(PathBuf::from("theme.toml")),
                apply: true,
                output: None,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--image", "image.jpg"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: None,
                apply: true,
                output: None,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--skip", "--image", "image.jpg", "theme.toml"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: Some(PathBuf::from("theme.toml")),
                apply: false,
                output: None,
            }
        );
        assert_eq!(
            Opt::from_iter(&[
                "test",
                "--skip",
                "--image",
                "image.jpg",
                "--output",
                "output.toml",
                "theme.toml"
            ]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: Some(PathBuf::from("theme.toml")),
                apply: false,
                output: Some(PathBuf::from("output.toml")),
            }
        );

        // Shouldn't parse
        Opt::from_iter_safe(&["test"]).unwrap_err();
        Opt::from_iter_safe(&["test", "--skip"]).unwrap_err();
    }
}
