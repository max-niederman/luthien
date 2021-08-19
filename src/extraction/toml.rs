use super::{Extractor, HashResult};
use crate::{
    persist::ExtractionConfig,
    theme::{Colors, Theme},
};
use color_eyre::eyre::{Result, WrapErr};
use palette::Srgb;
use serde::Deserialize;
use std::hash::Hasher;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

/// Generate a standard theme file from an easier-to-write TOML format.
///
/// This exists largely because it's difficult and tedious to convert colors
/// from hexadecimal codes to sRGB floating-point values as they are read by
/// Luthien. Instead, you can write string hex-code values and convert them
/// using this extractor.
#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Opt {
    /// Path to a TOML theme file. Defaults to stdin.
    path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Source {
    wallpaper: Option<PathBuf>,
    colors: Colors<String>,
}

impl From<Source> for Theme {
    fn from(source: Source) -> Self {
        Self {
            wallpaper: source.wallpaper,
            colors: source.colors.map(|hex| {
                let hex = hex.trim_start_matches('#');
                assert_eq!(hex.len(), 6);
                Srgb::new(
                    u16::from_str_radix(&hex[0..2], 16).expect("Failed to parse hex code") as f32
                        / 255.0,
                    u16::from_str_radix(&hex[2..4], 16).expect("Failed to parse hex code") as f32
                        / 255.0,
                    u16::from_str_radix(&hex[4..6], 16).expect("Failed to parse hex code") as f32
                        / 255.0,
                )
            }),
        }
    }
}

impl Extractor for Opt {
    fn extract(&self, _: &ExtractionConfig) -> Result<Theme> {
        let bytes: Vec<u8> = match &self.path {
            Some(path) => std::fs::read(path).wrap_err("Failed to open input file.")?,
            None => {
                let mut buf = Vec::new();
                io::stdin()
                    .read_to_end(&mut buf)
                    .wrap_err("Failed to read from standard input.")?;
                buf
            }
        };
        let source: Source = toml::from_slice(&bytes).wrap_err("Source was invalid.")?;
        Ok(source.into())
    }

    fn hash<H: Hasher>(&self, _: &ExtractionConfig, _: &mut H) -> Result<HashResult> {
        Ok(HashResult::Inapplicable)
    }
}
