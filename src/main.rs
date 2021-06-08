use log::{info, trace};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

mod apply;
mod color;
mod extraction;
mod mod_arith;
mod persist;
mod plugin;
mod theme;

use persist::{Config, Paths};
use theme::Theme;

#[derive(Debug, PartialEq, Clone, StructOpt)]
#[structopt(name = "luthien")]
struct Opt {
    /// Override the config file.
    #[structopt(short, long)]
    config: Option<PathBuf>,

    /// Skip running plugins.
    #[structopt(short = "s", long = "skip", parse(from_flag = std::ops::Not::not))]
    apply: bool,

    /// Output file for the theme.
    #[structopt(short, long)]
    output: Option<PathBuf>,

    #[structopt(subcommand)]
    command: Commands,
}

#[derive(Debug, PartialEq, Clone, StructOpt)]
enum Commands {
    /// Apply an existing theme.
    #[structopt(aliases = &["app", "a"])]
    Apply(apply::Opt),

    /// Extract a theme from another format.
    ///
    /// Currently, themes can be extracted from images.
    #[structopt(aliases = &["ext", "e"])]
    Extract(extraction::Opt),
}

pub trait Command {
    type Err: std::error::Error;

    fn run(&self, paths: &Paths, config: &Config) -> Result<Theme, Self::Err>;
}

impl Commands {
  fn run(&self, paths: &Paths, config: &Config) -> Result<Theme, io::Error> {
    match self {
      Self::Apply(apply) => apply.run(paths, config),
      Self::Extract(extract) => extract.run(paths, config),
    }
  }
}

impl Opt {
    fn get_paths(&self) -> Paths {
        let mut paths = Paths::default();

        if let Some(path) = &self.config {
            paths.config = path.clone();
        }

        paths
    }
}

fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init()
}

fn main() -> io::Result<()> {
    init_logger();

    trace!("Parsing opts...");
    let opt = Opt::from_args();

    trace!("Loading configuration...");
    let paths = opt.get_paths();
    paths.ensure_initialized()?;
    let config = paths.get_config()?;


    trace!("Running command...");
    let theme = opt.command.run(&paths, &config)?;

    info!("Theme Preview:\n{}", theme);

    if let Some(out) = opt.output {
        trace!("Writing theme to output...");
        serde_json::to_writer_pretty(std::fs::File::create(out)?, &theme)?;
    }

    if opt.apply {
        info!("Applying theme...");
        apply::apply(&config, theme)?;
    }

    Ok(())
}


