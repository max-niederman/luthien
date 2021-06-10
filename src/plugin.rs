use crate::persist::PluginConfig;
use crate::theme::Theme;
use color_eyre::eyre::{eyre, Result, WrapErr};
use log::trace;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directories {
    config: PathBuf,
    output: PathBuf,
    cache: PathBuf,
    data: PathBuf,
}

impl Directories {
    fn new(name: &str) -> Option<Self> {
        Some(Self {
            config: dirs::config_dir()?
                .join("luthien")
                .join("plugins")
                .join(name),
            output: dirs::data_local_dir()?
                .join("luthien")
                .join("outputs")
                .join("plugins")
                .join(name),
            cache: dirs::cache_dir()?
                .join("luthien")
                .join("plugins")
                .join(name),
            data: dirs::data_dir()?.join("luthien").join("plugins").join(name),
        })
    }

    fn ensure_initialized(&self) -> Result<()> {
        fn create_dir(path: &Path) -> io::Result<()> {
            match fs::create_dir_all(path) {
                Ok(_) => Ok(()),
                Err(err) => match err.kind() {
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(err),
                },
            }
        }

        create_dir(&self.config).wrap_err("Failed initializing config directory")?;
        create_dir(&self.output).wrap_err("Failed initializing output directory")?;
        create_dir(&self.cache).wrap_err("Failed initializing cache directory")?;
        create_dir(&self.data).wrap_err("Failed initializing data directory")?;

        Ok(())
    }
}

/// Data provided to the plugin process through its stdin
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInput {
    pipe: Option<PathBuf>,
    directories: Directories,

    name: String,
    options: serde_json::Value,
    theme: Theme,
}

pub trait Plugin {
    fn run<P: AsRef<OsStr>>(&self, theme: Theme, stdio_pipe: Option<P>) -> Result<ExitStatus>;
    fn name(&self) -> String;
}

impl Plugin for PluginConfig {
    fn run<P: AsRef<OsStr>>(&self, theme: Theme, stdin_pipe: Option<P>) -> Result<ExitStatus> {
        trace!("Preparing plugin input.");
        let input = {
            let name = self.name();
            let directories = Directories::new(&name)
                .ok_or_else(|| eyre!("Failed to find plugin directories"))?;
            directories
                .ensure_initialized()
                .wrap_err("Failed to initialize plugin directories")?;

            PluginInput {
                pipe: stdin_pipe.as_ref().map(PathBuf::from),
                options: self.options.clone(),
                directories,
                name,
                theme,
            }
        };

        trace!("Spawning plugin process: {:?}", self.executable);
        let mut child = Command::new(&self.executable)
            .args(&self.args)
            .envs(&self.env)
            .stdin(Stdio::piped())
            .stdout(Stdio::null()) // NOTE: In the future this will be piped to allow plugins to provide more data when they error
            .spawn()?;

        trace!("Writing plugin input.");
        serde_json::to_writer(
            child
                .stdin
                .as_mut()
                .ok_or_else(|| eyre!("Failed to get stdin of plugin process"))?,
            &input,
        )?;

        trace!("Waiting for plugin to finish executing");
        Ok(child.wait()?)
    }

    fn name(&self) -> String {
        self.name.as_ref().cloned().unwrap_or_else(|| {
            self.executable
                .file_stem()
                .or_else(|| self.executable.file_stem())
                .unwrap_or_else(|| self.executable.as_ref())
                .to_string_lossy()
                .trim_start_matches("luthien-")
                .into()
        })
    }
}
