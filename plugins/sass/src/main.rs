use askama::Template;
use std::fs;
use std::io;

#[derive(Template)]
#[template(path = "luthien.sass", escape = "none")]
struct SassTemplate {
    wallpaper: Option<String>,
    colors: luthien_plugin::Colors,
}

impl From<luthien_plugin::Theme> for SassTemplate {
    fn from(theme: luthien_plugin::Theme) -> Self {
        Self {
            wallpaper: theme
                .wallpaper
                .map(|p| p.to_str().map(String::from))
                .flatten(),
            colors: theme.colors,
        }
    }
}

trait AsHexCode {
    fn hex(&self) -> String;
}

impl AsHexCode for luthien_plugin::palette::Srgb {
    fn hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.red * 0xFF as f32) as u8,
            (self.green * 0xFF as f32) as u8,
            (self.blue * 0xFF as f32) as u8
        )
    }
}

fn main() -> io::Result<()> {
    let input = luthien_plugin::get_input()
        .expect("Input was malformed. Try updating this plugin and/or Luthien.");

    fs::write(
        input.directories.output.join("luthien.scss"),
        SassTemplate::from(input.theme)
            .render()
            .expect("Couldn't render Sass template file."),
    )?;

    let sass_options = grass::Options::default().load_path(&input.directories.output);

    let mut count = (0, 0);
    for entry in fs::read_dir(input.directories.config)? {
        let entry = entry?;

        match grass::from_path(entry.path().to_str().unwrap(), &sass_options) {
            Ok(compiled) => {
                let outfile = input
                    .directories
                    .output
                    .join(entry.path().with_extension("css").file_name().unwrap());
                fs::write(outfile, compiled)?;
                count.0 += 1;
            }
            Err(e) => {
                eprintln!(
                    "Failed to compile stylesheet {}:",
                    entry.file_name().to_string_lossy()
                );
                eprintln!("{}", e);
            }
        }
        count.1 += 1;
    }

    eprintln!("Successfully compiled {}/{} stylesheets.", count.0, count.1);
    Ok(())
}
