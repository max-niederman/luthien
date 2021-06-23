use crate::persist::{Config, Paths};
use crate::plugin::Plugin;
use crate::theme::Theme;
use color_eyre::eyre::{eyre, Result, WrapErr};
use log::{debug, error, info, trace, warn};
use std::fs::{self, File};
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
    fn run(&self, paths: &Paths, _config: &Config) -> Result<Option<Theme>> {
        info!("Reading theme from filesystem...");
        Ok(Some(paths.get_theme(&self.theme)?))
    }
}

pub fn apply(config: &Config, theme: Theme) -> Result<()> {
    fn get_pipe() -> ipipe::Result<ipipe::Pipe> {
        let pipe_path =
            std::env::temp_dir().join(format!("luthien_plugin_stdio_{}", std::process::id()));
        ipipe::Pipe::open(&pipe_path, ipipe::OnCleanup::Delete)
    }

    info!("Applying theme...");

    let output_dir = dirs::data_local_dir()
        .ok_or_else(|| eyre!("Failed to get local data directory"))?
        .join("luthien")
        .join("outputs");

    trace!("Writing theme to application output directory...");
    serde_json::to_writer_pretty(
        &mut File::create(output_dir.join("theme.json"))
            .wrap_err("Failed to create theme output file")?,
        &theme,
    )
    .wrap_err("Failed to write theme to application output")?;

    if let Some(path) = &theme.wallpaper {
        trace!("Copying wallpaper to application output directory...");
        fs::copy(path, output_dir.join("wallpaper"))
            .wrap_err("Failed to copy wallpaper to application output")?;
    } else {
        let path = output_dir.join("wallpaper");
        if path.exists() {
            trace!("Removing old wallpaper from application output");
            fs::remove_file(path)
                .wrap_err("Failed to delete old wallpaper from application output")?;
        }
    }

    trace!("Spawning plugin IO pipes...");
    thread::spawn(|| io::copy(&mut io::stdin(), &mut get_pipe()?));
    thread::spawn(|| io::copy(&mut get_pipe()?, &mut io::stdout()));

    info!("Running plugins...");
    for pl in config.plugins.iter() {
        trace!("Running plugin {}...", pl.name());
        match pl.run(
            theme.clone(),
            get_pipe()
                .map(|p| p.path().to_path_buf())
                .map_err(|_| warn!("Failed to get a named pipe for the plugin"))
                .ok(),
        ) {
            Ok(status) => {
                if status.success() {
                    info!("Plugin {} exited successfully", pl.name());
                } else {
                    error!("Plugin {} exited with a non-zero error code", pl.name());
                }
            }
            Err(e) => {
                // TODO: Add descriptive error message
                error!("Failed to run plugin {}.", pl.name());
                debug!("{}", e);
            }
        }
    }

    drop(get_pipe().wrap_err("Failed to create a named pipe for plugin IO")?);
    Ok(())
}
