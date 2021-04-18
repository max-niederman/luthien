use super::theme;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugins: Vec::default(),
        }
    }
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
