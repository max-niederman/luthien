mod img;
mod toml;

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

pub trait Extractor {
    fn extract(&self, config: &ExtractionConfig) -> Result<Theme>;
    fn hash<H: Hasher>(&self, config: &ExtractionConfig, state: &mut H) -> Result<HashResult>;
}

#[impl_enum::with_methods {
  fn extract(&self, config: &ExtractionConfig) -> Result<Theme> {}
  fn hash<H: Hasher>(&self, config: &ExtractionConfig, state: &mut H) -> Result<HashResult> {}
}]
#[derive(Debug, PartialEq, Clone, StructOpt)]
enum Extractors {
    /// Extract common colors from an image
    #[structopt(aliases = &["img", "i"])]
    Image(img::Opt),

    /// Manually create a theme.
    #[structopt(aliases = &["manual", "man"])]
    Toml(toml::Opt),
}

impl Extractors {
    fn cache_path(&self, paths: &Paths, config: &Config) -> Result<Option<PathBuf>> {
        let hash = {
            let mut hasher = DefaultHasher::default();

            if !self
                .hash(&config.extraction, &mut hasher)
                .wrap_err("Failed to generate caching ID for extraction")?
                .finished()
            {
                return Ok(None);
            }

            hasher.finish()
        };

        Ok(Some(paths.cache.join(format!("{:16x}", hash))))
    }
}

impl crate::Command for Opt {
    fn run(&self, paths: &Paths, config: &Config) -> Result<Option<Theme>> {
        trace!("Finding extraction cache location...");
        let cache_path = self
            .extractor
            .cache_path(paths, config)
            .wrap_err("Failed to find extraction cache location")?;

        info!("Extracting theme...");
        // NOTE: This is necessary because let chains are not yet stable. In the future, `if let Some(cache_path) = cache_path && ...` should be possible.
        let theme = if cache_path.is_some() && self.cache && cache_path.as_ref().unwrap().exists() {
            let cache_path = cache_path.as_ref().unwrap();

            info!("Cache hit; using cached theme...");

            serde_json::from_reader(
                File::open(cache_path).wrap_err("Failed reading cached theme file")?,
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

        if let Some(cache_path) = cache_path {
            trace!("Caching extracted theme...");
            File::create(cache_path)
                .map(|file| {
                    serde_json::to_writer(file, &theme)
                        .unwrap_or_else(|_| warn!("Failed to write theme to cache file"))
                })
                .unwrap_or_else(|_| error!("Failed to create theme cache file"));
        }

        Ok(Some(theme))
    }
}

pub enum HashResult {
    Finished,
    Inapplicable,
}

impl HashResult {
    pub fn finished(self) -> bool {
        match self {
            Self::Finished => true,
            Self::Inapplicable => false,
        }
    }
}
