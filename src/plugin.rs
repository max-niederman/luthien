use crate::persist::PluginConfig;
use crate::theme::Theme;
use log::trace;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

/// Data provided to the plugin process through its stdin
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginInput {
    theme: Theme,
    options: serde_json::Value,
    pipe: Option<PathBuf>,
}

pub trait Plugin {
    fn run(&self, theme: Theme, stdio_pipe: Option<&Path>) -> io::Result<ExitStatus>;
}

impl Plugin for PluginConfig {
    fn run(&self, theme: Theme, stdin_pipe: Option<&Path>) -> io::Result<ExitStatus> {
        trace!("Spawning plugin process: {}", self.executable);
        let mut child = Command::new(&self.executable)
            .args(&self.args)
            .envs(&self.env)
            .stdin(Stdio::piped())
            .spawn()?;

        trace!("Writing plugin input.");
        serde_json::to_writer(
            child
                .stdin
                .as_mut()
                .ok_or(io::Error::from(io::ErrorKind::BrokenPipe))?,
            &PluginInput {
                options: self.options.clone(),
                pipe: stdin_pipe.map(PathBuf::from),
                theme,
            },
        )?;

        trace!("Waiting for plugin to finish executing.");
        child.wait()
    }
}
