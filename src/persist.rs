use crate::{color, theme};
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Paths {
    pub config: PathBuf,
    pub themes: PathBuf,
    pub cache: PathBuf,
}

impl Paths {
    pub fn set_config(&mut self, path: PathBuf) -> &mut Self {
        self.config = path;
        self
    }

    pub fn ensure_initialized(&self) -> io::Result<()> {
        fs::create_dir_all(&self.themes)?;
        fs::create_dir_all(&self.cache)?;

        if !self.config.exists() {
            self.config.parent().map(fs::create_dir_all);
            fs::write(&self.config, [])?;
        }

        Ok(())
    }

    pub fn get_config(&self) -> io::Result<Config> {
        match fs::read(&self.config) {
            Ok(raw) => {
                toml::from_slice(&raw).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    warn!("Config file not found. Using default.");
                    Ok(Config::default())
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn get_theme(&self, locator: impl AsRef<Path>) -> io::Result<theme::Theme> {
        let file = File::open(&locator).or_else(|_| File::open(self.themes.join(locator)))?;
        let reader = io::BufReader::new(file);

        serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Default for Paths {
    fn default() -> Self {
        let config_root = dirs::config_dir()
            .expect("Couldn't find config directory.")
            .join("luthien");

        Self {
            config: config_root.join("config.toml"),
            themes: config_root.join("themes"),
            cache: dirs::cache_dir()
                .expect("Couldn't find cache directory.")
                .join("luthien"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RegionConfig {
    hue: (f32, f32),
    saturation: (f32, f32),
    lightness: (f32, f32),
}

impl Hash for RegionConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash_comp = |comp: f32| state.write(&comp.to_le_bytes());

        hash_comp(self.hue.0);
        hash_comp(self.hue.1);
        hash_comp(self.saturation.0);
        hash_comp(self.saturation.1);
        hash_comp(self.lightness.0);
        hash_comp(self.lightness.1);
    }
}

impl From<RegionConfig> for color::Region<f32> {
    fn from(config: RegionConfig) -> Self {
        Self::new(
            config.hue.0..=config.hue.1,
            config.saturation.0..=config.saturation.1,
            config.lightness.0..=config.lightness.1,
        )
    }
}

impl Default for theme::Palette<RegionConfig> {
    fn default() -> Self {
        const PRIMARY_SAT: (f32, f32) = (0.5, 1.0);
        const PRIMARY_LIGHTNESS: (f32, f32) = (0.1, 0.9);
        Self {
            black: RegionConfig {
                hue: (0.0, 360.0),
                saturation: (0.0, 1.0),
                lightness: (0.0, PRIMARY_LIGHTNESS.0),
            },
            red: RegionConfig {
                hue: (345.0, 15.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            green: RegionConfig {
                hue: (90.0, 150.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            yellow: RegionConfig {
                hue: (45.0, 75.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            blue: RegionConfig {
                hue: (210.0, 255.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            purple: RegionConfig {
                hue: (270.0, 300.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            cyan: RegionConfig {
                hue: (165.0, 195.0),
                saturation: PRIMARY_SAT,
                lightness: PRIMARY_LIGHTNESS,
            },
            white: RegionConfig {
                hue: (0.0, 360.0),
                saturation: (0.0, 0.5),
                lightness: (PRIMARY_LIGHTNESS.1, 1.0),
            },
        }
    }
}

#[derive(Deserialize)]
pub struct PluginConfigRaw {
    pub executable: PathBuf,
    pub name: Option<String>,

    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub options: serde_json::Value,
}

impl From<PluginConfigRaw> for PluginConfig {
    fn from(raw: PluginConfigRaw) -> Self {
        Self {
            executable: {
                if raw.executable.iter().next() == Some("~".as_ref()) {
                    PathBuf::from_iter(
                        std::iter::once(
                            dirs::home_dir()
                                .ok_or_else(std::env::current_dir)
                                .expect("Couldn't expand tilde (~) in plugin executable path."),
                        )
                        .chain(raw.executable.iter().skip(1).map(PathBuf::from)),
                    )
                } else if raw.executable.is_relative() {
                    dirs::config_dir()
                        .map(|p| p.join("luthien"))
                        .or_else(dirs::home_dir)
                        .unwrap_or_else(PathBuf::new)
                        .join(raw.executable)
                } else {
                    raw.executable
                }
            },

            name: raw.name,
            args: raw.args,
            env: raw.env,
            options: raw.options,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "PluginConfigRaw")]
pub struct PluginConfig {
    pub executable: PathBuf,
    pub name: Option<String>,

    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub options: serde_json::Value,
}

#[serde(default)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractionConfig {
    pub target: theme::Palette<RegionConfig>,
}

#[serde(default)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
    pub extraction: ExtractionConfig,
}

#[cfg(test)]
mod tests {
    use super::Paths;
    use std::path::PathBuf;

    #[test]
    fn get_config() {
        let paths = Paths::default();
        paths.get_config().unwrap();
    }

    #[test]
    fn get_nonexistent_theme() {
        let paths = Paths::default();
        let theme = paths.get_theme(PathBuf::from("/non/existent/path"));
        debug_assert!(theme.is_err(), "Theme was Ok:\n{:?}", theme);
    }
}
