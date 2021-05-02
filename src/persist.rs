use crate::{color, theme};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Paths {
    pub config: PathBuf,
    pub themes: PathBuf,
    pub cache: PathBuf,
}

impl Paths {
    pub fn get_config(&self) -> io::Result<Config> {
        match fs::read(&self.config) {
            Ok(raw) => {
                toml::from_slice(&raw).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    info!("Config file not found. Using default.");
                    Ok(Config::default())
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn get_theme(&self, locator: PathBuf) -> Option<io::Result<theme::Theme>> {
        let file = File::open(locator.clone())
            .or_else(|_| File::open(self.themes.join(locator)))
            .ok()?;
        let reader = io::BufReader::new(file);

        Some(
            serde_json::from_reader(reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        )
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
            cache: dirs::cache_dir().expect("Couldn't find cache directory."),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegionConfig {
    hue: (f32, f32),
    saturation: (f32, f32),
    value: (f32, f32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegionsConfig {
    black: RegionConfig,
}

impl From<RegionConfig> for color::Region<f32> {
    fn from(config: RegionConfig) -> Self {
        Self::new(
            config.hue.0..=config.hue.1,
            config.saturation.0..=config.saturation.1,
            config.value.0..=config.value.1,
        )
    }
}

impl Default for theme::Palette<RegionConfig> {
    fn default() -> Self {
        const PRIMARY_SAT: (f32, f32) = (0.1, 1.0);
        const PRIMARY_VALUE: (f32, f32) = (0.1, 1.0);
        Self {
            black: RegionConfig {
                hue: (0.0, 360.0),
                saturation: (0.0, 0.5),
                value: (0.0, PRIMARY_VALUE.0),
            },
            red: RegionConfig {
                hue: (345.0, 15.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            green: RegionConfig {
                hue: (90.0, 150.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            yellow: RegionConfig {
                hue: (45.0, 75.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            blue: RegionConfig {
                hue: (195.0, 255.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            purple: RegionConfig {
                hue: (270.0, 300.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            cyan: RegionConfig {
                hue: (165.0, 195.0),
                saturation: PRIMARY_SAT,
                value: PRIMARY_VALUE,
            },
            white: RegionConfig {
                hue: (0.0, 360.0),
                saturation: (0.0, PRIMARY_SAT.0),
                value: (0.5, 1.0),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub executable: String,

    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub options: serde_json::Value,
}

#[serde(default)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub colors: theme::Palette<RegionConfig>,
    pub plugins: Vec<PluginConfig>,
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
        debug_assert!(theme.is_none(), "Theme was Some:\n{:?}", theme);
    }
}
