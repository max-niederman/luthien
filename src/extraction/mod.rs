mod img;

use crate::persist::{Config, ExtractionConfig, Paths};
use crate::theme::Theme;
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
    Image(img::Opt),
}

pub trait Extractor {
    type Err: std::error::Error;

    fn extract(&self, config: &ExtractionConfig) -> Result<Theme, Self::Err>;
    fn hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Err>;
}

// TODO: Rewrite with macros
impl Extractors {
    fn extract(&self, config: &ExtractionConfig) -> Result<Theme, std::io::Error> {
        match self {
            Self::Image(img) => img.extract(config),
        }
    }

    fn hash<H: Hasher>(&self, state: &mut H) -> Result<(), std::io::Error> {
        match self {
            Self::Image(img) => img.hash(state),
        }
    }

    fn cache_path(&self, paths: &Paths) -> Result<PathBuf, std::io::Error> {
        let hash = {
            let mut hasher = DefaultHasher::default();
            self.hash(&mut hasher)?;
            hasher.finish()
        };

        Ok(paths.cache.join(format!("{:16x}", hash)))
    }
}

impl crate::Command for Opt {
    type Err = std::io::Error;

    fn run(&self, paths: &Paths, config: &Config) -> Result<Theme, Self::Err> {
        let cache_path = self.extractor.cache_path(&paths)?;

        let theme = if self.cache && cache_path.exists() {
            info!("Cache hit; using cached theme...");

            serde_json::from_reader(File::open(&cache_path)?)?
        } else {
            if self.cache {
                info!("Cache missed; extracting theme...");
            } else {
                info!("Extracting theme...");
            }

            self.extractor.extract(&config.extraction)?
        };

        trace!("Caching extracted theme...");
        File::create(&cache_path)
            .map(|file| {
                serde_json::to_writer(file, &theme)
                    .unwrap_or_else(|_| warn!("Failed to write theme to cache file."))
            })
            .unwrap_or_else(|_| error!("Failed to create theme cache file."));

        Ok(theme)
    }
}
