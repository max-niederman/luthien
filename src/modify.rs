use crate::persist::{Config, Paths};
use crate::theme::Theme;
use color_eyre::eyre::Result;
use log::info;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Clone, StructOpt)]
pub struct Opt {
    /// Theme to be modified.
    pub theme: PathBuf,

    /// New wallpaper to be used.
    #[structopt(short, long)]
    pub wallpaper: Option<PathBuf>,

    /// Swap foreground and background colors.
    #[structopt(short, long)]
    pub swap: bool,
}

impl crate::Command for Opt {
    fn run(self, paths: &Paths, _config: &Config) -> Result<Option<Theme>> {
        info!("Reading theme from filesystem...");
        let mut theme = paths.get_theme(&self.theme)?;

        if self.wallpaper.is_some() {
            info!("Setting wallpaper...");
            theme.wallpaper = self.wallpaper;
        }

        if self.swap {
            info!("Swapping colors...");
            std::mem::swap(&mut theme.colors.foreground, &mut theme.colors.background);
        }

        Ok(Some(theme))
    }
}
