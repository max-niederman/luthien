use log::trace;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, StructOpt)]
#[structopt(name = "luthien")]
struct Opt {
    /// Image to use for theming.
    ///
    /// If colors are specified, the image will be recolored with those colors. Otherwise, the
    /// colors will be extracted from the image.
    #[structopt(short, long)]
    image: Option<PathBuf>,

    /// Base theme to use.
    ///
    /// This argument is required unless the image option is given, in which case the theme
    /// will be derived from the image.
    #[structopt(required_unless = "image")]
    theme: Option<PathBuf>,

    /// Skip applying the theme, instead printing the generated theme to stdout.
    #[structopt(short = "s", long = "skip", parse(from_flag = std::ops::Not::not))]
    apply: bool,
}

fn main() {
    pretty_env_logger::init();

    trace!("Parsing opts.");
    let opt = Opt::from_args();
    trace!("Parsed opts: {:?}", opt);

    todo!("generate and apply theme");
}

mod tests {
    use super::Opt;
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[test]
    fn opt_parsing() {
        // Should parse
        assert_eq!(
            Opt::from_iter(&["test", "theme.toml"]),
            Opt {
                image: None,
                theme: Some(PathBuf::from("theme.toml")),
                apply: true,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--image", "image.jpg"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: None,
                apply: true,
            }
        );
        assert_eq!(
            Opt::from_iter(&["test", "--skip", "--image", "image.jpg", "theme.toml"]),
            Opt {
                image: Some(PathBuf::from("image.jpg")),
                theme: Some(PathBuf::from("theme.toml")),
                apply: false,
            }
        );

        // Shouldn't parse
        Opt::from_iter_safe(&["test"]).unwrap_err();
        Opt::from_iter_safe(&["test", "--skip"]).unwrap_err();
    }
}
