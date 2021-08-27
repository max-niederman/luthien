use color_eyre::eyre::{Result, WrapErr};
use log::{info, trace};
use std::path::PathBuf;
use structopt::{clap, StructOpt};

mod apply;
mod color;
mod extraction;
mod mod_arith;
mod modify;
mod persist;
mod plugin;
mod theme;

use persist::{Config, Paths};
use theme::Theme;

#[derive(Debug, PartialEq, Clone, StructOpt)]
#[structopt(author, about)]
struct Opt {
    /// Override the config file
    #[structopt(short, long)]
    config: Option<PathBuf>,

    /// Skip applying the theme
    #[structopt(short = "s", long = "skip", parse(from_flag = std::ops::Not::not))]
    apply_step: bool,

    /// Output file for the theme
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

    /// Modify one or more properties of an existing theme.
    ///
    /// Currently, only some properties can be changed using this command.
    /// Further modification must be done manually.
    #[structopt(aliases = &["mod", "m"])]
    Modify(modify::Opt),

    /// Extract a theme from another format.
    ///
    /// Currently, themes can be extracted from images.
    #[structopt(aliases = &["ext", "e"])]
    Extract(extraction::Opt),

    /// Generate shell completions and print to stdout
    Completions {
        #[structopt(possible_values = &clap::Shell::variants())]
        shell: String,
    },
}

pub trait Command {
    fn run(self, paths: &Paths, config: &Config) -> Result<Option<Theme>>;
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

fn main() -> Result<()> {
    color_eyre::install()?;
    init_logger();

    trace!("Parsing opts...");
    let opt = Opt::from_args();

    trace!("Loading configuration...");
    let paths = opt.get_paths();
    paths
        .ensure_initialized()
        .wrap_err("Failed to initialize config, data, and/or cache directories")?;
    let config = paths
        .get_config()
        .wrap_err("Failed to load configuration")?;

    trace!("Running command...");
    let res = match opt.command {
        Commands::Apply(cmd) => cmd.run(&paths, &config)?,
        Commands::Modify(cmd) => cmd.run(&paths, &config)?,
        Commands::Extract(cmd) => cmd.run(&paths, &config)?,

        Commands::Completions { shell } => {
            info!("Generating completions...");
            Opt::clap().gen_completions_to(
                "luthien",
                shell.parse().unwrap(),
                &mut std::io::stdout().lock(),
            );

            None
        }
    };

    if let Some(theme) = res {
        info!("Theme Preview:\n{}", theme);

        if let Some(out) = opt.output {
            trace!("Writing theme to output...");
            serde_json::to_writer_pretty(
                std::fs::File::create(out).wrap_err("Failed to write to output file")?,
                &theme,
            )
            .wrap_err("Failed to serialize the theme")?;
        }

        if opt.apply_step {
            apply::apply(&config, theme).wrap_err("Failed to apply the theme")?;
        }
    }

    Ok(())
}
