use crate::persist::{Config, Paths};
use crate::plugin::Plugin;
use crate::theme::Theme;
use log::{debug, error, info, trace, warn};
use std::io;
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Clone, StructOpt)]
pub struct Opt {
    /// Theme to be applied.
    pub theme: PathBuf,
}

impl crate::Command for Opt {
    type Err = std::io::Error;

    fn run(&self, paths: &Paths, _config: &Config) -> Result<Theme, Self::Err> {
        info!("Reading theme from filesystem...");
        paths.get_theme(&self.theme)
    }
}

pub fn apply(config: &Config, theme: Theme) -> io::Result<()> {
    fn get_pipe() -> ipipe::Result<ipipe::Pipe> {
        let pipe_path =
            std::env::temp_dir().join(format!("luthien_plugin_stdio_{}", std::process::id()));
        ipipe::Pipe::open(&pipe_path, ipipe::OnCleanup::Delete)
    }

    trace!("Spawning plugin IO pipes...");
    thread::spawn(|| io::copy(&mut io::stdin(), &mut get_pipe()?));
    thread::spawn(|| io::copy(&mut get_pipe()?, &mut io::stdout()));

    for pl in config.plugins.iter() {
        trace!("Running plugin {}...", pl.name());
        match pl.run(
            theme.clone(),
            get_pipe()
                .map(|p| p.path().to_path_buf())
                .map_err(|_| warn!("Failed to get a named pipe for the plugin."))
                .ok(),
        ) {
            Ok(status) => {
                if status.success() {
                    info!("Plugin {} exited successfully.", pl.name());
                } else {
                    error!("Plugin {} exited with a non-zero error code.", pl.name());
                }
            }
            Err(e) => {
                // TODO: Add descriptive error message
                error!("Failed to run plugin {}.", pl.name());
                debug!("{}", e);
            }
        }
    }

    drop(get_pipe()?);
    Ok(())
}
