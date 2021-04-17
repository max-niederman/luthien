use super::theme;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Paths {
    pub config: PathBuf,
    pub themes: PathBuf,
    pub cache: PathBuf,
}

impl Paths {
    pub fn get_theme(&self, locator: PathBuf) -> io::Result<theme::Theme> {
        let file =
            File::open(locator.clone()).or_else(|_| File::open(self.themes.join(locator)))?;
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
    fn nonexistent_theme() {
        let paths = Paths::default();
        paths
            .get_theme(PathBuf::from("/non/existent/path"))
            .unwrap_err();
    }
}
