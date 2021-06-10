mod img;

use crate::persist::{Config, ExtractionConfig, Paths};
use crate::theme::Theme;
use color_eyre::eyre::{Result, WrapErr};
use log::{error, info, trace, warn};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Clone, StructOpt)]
pub struct Opt {
    /// Disable the extractor cache.
    #[structopt(long = "no-cache", parse(from_flag = std::ops::Not::not))]
    cache: bool,

    /// Which extractor should be used.
    #[structopt(subcommand)]
    extractor: Extractors,
}

#[derive(Debug, PartialEq, Clone, StructOpt)]
enum Extractors {
    /// Extract common colors from an image
    #[structopt(aliases = &["img", "i"])]
    Image(img::Opt),
}

pub trait Extractor {
    fn extract(&self, config: &ExtractionConfig) -> Result<Theme>;
    fn hash<H: Hasher>(&self, state: &mut H) -> Result<()>;
}

// TODO: Rewrite with macros
impl Extractors {
    fn extract(&self, config: &ExtractionConfig) -> Result<Theme> {
        match self {
            Self::Image(img) => img.extract(config),
        }
    }

    fn hash<H: Hasher>(&self, state: &mut H) -> Result<()> {
        match self {
            Self::Image(img) => img.hash(state),
        }
    }

    fn cache_path(&self, paths: &Paths) -> Result<PathBuf> {
        let hash = {
            let mut hasher = DefaultHasher::default();
            self.hash(&mut hasher).wrap_err("Failed to generate caching ID for extraction")?;
            hasher.finish()
        };

        Ok(paths.cache.join(format!("{:16x}", hash)))
    }
}

impl crate::Command for Opt {
    fn run(&self, paths: &Paths, config: &Config) -> Result<Option<Theme>> {
        let cache_path = self.extractor.cache_path(&paths).wrap_err("Failed to find extraction cache location")?;

        let theme = if self.cache && cache_path.exists() {
            info!("Cache hit; using cached theme...");

            serde_json::from_reader(
                File::open(&cache_path).wrap_err("Failed reading cached theme file")?,
            )
            .wrap_err("Failed deserializing cached theme file")?
        } else {
            if self.cache {
                info!("Cache missed; extracting theme...");
            } else {
                info!("Extracting theme...");
            }

            self.extractor
                .extract(&config.extraction)
                .wrap_err("Failed to extract theme")?
        };

        trace!("Caching extracted theme...");
        File::create(&cache_path)
            .map(|file| {
                serde_json::to_writer(file, &theme)
                    .unwrap_or_else(|_| warn!("Failed to write theme to cache file"))
            })
            .unwrap_or_else(|_| error!("Failed to create theme cache file"));

        Ok(Some(theme))
    }
}


