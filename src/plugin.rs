use crate::persist::PluginConfig;
use crate::theme::Theme;
use log::trace;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

/// Data provided to the plugin process through its stdin
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInput {
    theme: Theme,
    options: serde_json::Value,
    pipe: Option<PathBuf>,
}

pub trait Plugin {
    fn run<P>(&self, theme: Theme, stdio_pipe: Option<P>) -> io::Result<ExitStatus>
    where
        P: AsRef<OsStr>;

    fn name(&self) -> &OsStr;
}

impl Plugin for PluginConfig {
    fn run<P>(&self, theme: Theme, stdin_pipe: Option<P>) -> io::Result<ExitStatus>
    where
        P: AsRef<OsStr>,
    {
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
            &PluginInput {
                options: self.options.clone(),
                pipe: stdin_pipe.as_ref().map(PathBuf::from),
                theme,
            },
        )?;

        trace!("Waiting for plugin to finish executing.");
        child.wait()
    }

    fn name(&self) -> &OsStr {
        self.executable.file_stem().or_else(|| self.executable.file_name()).unwrap_or_else(|| self.executable.as_ref())
    }
}
