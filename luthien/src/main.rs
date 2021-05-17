use log::{error, warn, info, trace, debug};
use rayon::prelude::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::iter::{self, FromIterator};
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;

mod color;
mod color_gen;
mod mod_arith;
mod persist;
mod plugin;
mod theme;

use persist::{Config, Paths};
use theme::Theme;

#[derive(Debug, PartialEq, Eq, Clone, StructOpt)]
#[structopt(name = "luthien")]
struct Opt {
    /// Base theme to use.
    ///
    /// This argument is required unless the image option is given, in which case the theme
    /// will be derived from the image.
    #[structopt(required_unless = "image")]
    theme: Option<PathBuf>,

    /// Image to use for theming.
    ///
    /// If colors are specified, the image will be recolored with those colors (not yet
    /// implemented). Otherwise, the
    /// colors will be extracted from the image.
    #[structopt(short, long)]
    image: Option<PathBuf>,

    /// Output file for the used theme.
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Override the config file.
    #[structopt(short, long)]
    config: Option<PathBuf>,

    /// Skip running plugins.
    #[structopt(short = "s", long = "skip", parse(from_flag = std::ops::Not::not))]
    apply: bool,

    /// Don't cache the generated theme.
    ///
    /// The cached theme is labeled by the hash of its source image and generation options, so
    /// there are no problems with changing filenames.
    #[structopt(long = "no-cache", parse(from_flag = std::ops::Not::not))]
    cache: bool,
}

impl Opt {
    fn get_paths(&self) -> Paths {
        let mut paths = Paths::default();

        if let Some(path) = &self.config {
            paths.set_config(path.clone());
        }

        paths
    }
}

fn get_theme(opt: &Opt, paths: &Paths, config: &Config) -> io::Result<theme::Theme> {
    fn hash<T: Hash>(data: &T, _opt: &Opt, config: &Config) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);

        config.colors.hash(&mut hasher);

        hasher.finish()
    }

    if let Some(path) = &opt.theme {
        info!("Reading theme from filesystem...");
        paths.get_theme(path.clone())
    } else if let Some(path) = &opt.image {
        trace!("Reading and decoding image...");
        let img = image::io::Reader::open(&path)?
            .with_guessed_format()?
            .decode()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .into_rgb8();

        let cached_path = paths
            .cache
            .join(format!("{:16x}", hash(&img, &opt, &config)));

        if let Ok(Ok(theme)) = File::open(&cached_path).map(serde_json::from_reader::<File, Theme>)
        {
            info!("Cache hit; using cached theme...");
            Ok(theme)
        } else {
            info!("Cache missed; generating theme from image...");
            let theme = Theme {
                wallpaper: Some(path.clone()),
                colors: {
                    use palette::Srgb;

                    trace!("Generating color palette...");
                    let generator = color_gen::Generator::default();
                    generator.generate(
                        img.par_chunks(3).map(|pix| {
                            Srgb::from_components((
                                pix[0] as f32 / 255.0,
                                pix[1] as f32 / 255.0,
                                pix[2] as f32 / 255.0,
                            ))
                        }),
                        config.colors.map(Into::into),
                    )
                },
            };

            if opt.cache {
                trace!("Caching generated theme.");
                File::create(&cached_path)
                    .map(|file| {
                        serde_json::to_writer(file, &theme)
                            .unwrap_or_else(|_| warn!("Failed to write theme to cache file."))
                    })
                    .unwrap_or_else(|_| error!("Failed to create theme cache file."));
            }

            Ok(theme)
        }
    } else {
        unreachable!()
    }
}

fn normalize_plugins(paths: &Paths, config: &mut Config) {
    for pl in config.plugins.iter_mut() {
        if pl.executable.iter().next() == Some("~".as_ref()) {
            pl.executable = PathBuf::from_iter(
                iter::once(
                    dirs::home_dir()
                        .ok_or_else(std::env::current_dir)
                        .expect("Couldn't expand tilde (~) in plugin executable path."),
                )
                .chain(pl.executable.iter().skip(1).map(PathBuf::from)),
            );
        }

        if pl.executable.is_relative() {
            pl.executable = paths
                .config
                .parent()
                .map(PathBuf::from)
                .or_else(dirs::home_dir)
                .unwrap_or_else(PathBuf::new)
                .join(pl.executable.clone());
        }
    }
}

fn run_plugins(config: &Config, theme: Theme) -> io::Result<()> {
    use plugin::Plugin;

    fn get_pipe() -> ipipe::Result<ipipe::Pipe> {
        let pipe_path =
            std::env::temp_dir().join(format!("luthien_plugin_stdio_{}", std::process::id()));
        ipipe::Pipe::open(&pipe_path, ipipe::OnCleanup::Delete)
    }

    thread::spawn(|| io::copy(&mut io::stdin(), &mut get_pipe()?));
    thread::spawn(|| io::copy(&mut get_pipe()?, &mut io::stdout()));

    for pl in config.plugins.iter() {
        match pl.run(
            theme.clone(),
            get_pipe()
                .map(|p| p.path().to_path_buf())
                .map_err(|_| warn!("Failed to get a named pipe for the plugin."))
                .ok(),
        ) {
            Ok(status) => {
                if status.success() {
                    info!("Plugin {} exited successfully.", pl.name());
                } else {
                    error!("Plugin {} exited with a non-zero error code.", pl.name());
                }
            }
            Err(e) => {
                // TODO: Add descriptive error message
                error!("Failed to run plugin {}.", pl.name());
                debug!("{}", e);
            }
        }
    }

    drop(get_pipe()?);
    Ok(())
}

fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init()
}

fn main() -> io::Result<()> {
    init_logger();

    trace!("Parsing opts...");
    let opt = Opt::from_args();

    trace!("Loading configuration...");
    let paths = opt.get_paths();
    paths.ensure_initialized()?;

    let mut config = paths.get_config()?;
    normalize_plugins(&paths, &mut config);

    info!("Getting theme...");
    let theme = get_theme(&opt, &paths, &config)?;

    info!("Theme Preview:\n{}", theme);

    if let Some(out) = opt.output {
        info!("Writing theme to output...");
        serde_json::to_writer_pretty(std::fs::File::create(out)?, &theme)?;
    }

    if opt.apply {
        info!("Running plugins...");
        run_plugins(&config, theme)?;
    }

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
                output: None,
                config: None,
                apply: true,
                cache: true,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--image", "image.jpg"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: None,
                output: None,
                config: None,
                apply: true,
                cache: true,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--skip", "--image", "image.jpg", "theme.toml"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: Some(PathBuf::from("theme.toml")),
                output: None,
                config: None,
                apply: false,
                cache: true,
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
                "--no-cache",
                "theme.toml"
            ]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: Some(PathBuf::from("theme.toml")),
                output: Some(PathBuf::from("output.toml")),
                config: None,
                apply: false,
                cache: false,
            }
        );

        // Shouldn't parse
        Opt::from_iter_safe(&["test"]).unwrap_err();
        Opt::from_iter_safe(&["test", "--skip"]).unwrap_err();
    }
}
