use crate::persist::PluginConfig;
use crate::theme::Theme;
use log::trace;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, ErrorKind};
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

    fn ensure_initialized(&self) -> io::Result<()> {
        fn create_dir(path: &Path) -> io::Result<()> {
            match fs::create_dir_all(path) {
                Ok(_) => Ok(()),
                Err(err) => match err.kind() {
                    ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(err),
                },
            }
        }

        create_dir(&self.config)?;
        create_dir(&self.output)?;
        create_dir(&self.cache)?;
        create_dir(&self.data)?;

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
    fn run<P>(&self, theme: Theme, stdio_pipe: Option<P>) -> io::Result<ExitStatus>
    where
        P: AsRef<OsStr>;

    fn name(&self) -> String;
}

impl Plugin for PluginConfig {
    fn run<P>(&self, theme: Theme, stdin_pipe: Option<P>) -> io::Result<ExitStatus>
    where
        P: AsRef<OsStr>,
    {
        trace!("Preparing plugin input.");
        let input = {
            let name = self.name();
            let directories =
                Directories::new(&name).ok_or_else(|| io::Error::from(io::ErrorKind::Other))?;
            directories.ensure_initialized()?;

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
            .stdout(Stdio::null()) // In the future this will be piped to allow plugins to provide more data when they error.
            .spawn()?;

        trace!("Writing plugin input.");
        serde_json::to_writer(
            child
                .stdin
                .as_mut()
                .ok_or_else(|| io::Error::from(io::ErrorKind::BrokenPipe))?,
            &input,
        )?;

        trace!("Waiting for plugin to finish executing.");
        child.wait()
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
