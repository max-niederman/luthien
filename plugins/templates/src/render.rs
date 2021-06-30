use color_eyre::eyre::{Result, Report};
use luthien_plugin::Input;
use std::{fs::{self, File}, path::PathBuf};
use tera::Tera;

pub struct Renderer {
    tera: Tera,
    ctx: tera::Context,
}

impl Renderer {
    pub fn from_input(input: &Input) -> Result<Self> {
        Ok(Self {
            tera: {
                let mut tr = Tera::default();
                tr.add_template_files(
                    fs::read_dir(&input.directories.config)?
                        .filter_map(Result::ok)
                        .map(|e| (e.path(), e.file_name().into_string().ok()))
                        .collect::<Vec<(PathBuf, Option<String>)>>(),
                )?;
                tr
            },
            ctx: tera::Context::default(),
        })
    }

    pub fn render(&self, out_dir: PathBuf) -> Vec<Result<()>> {
        self.tera.get_template_names().map(|name| {
            self.tera.render_to(
                name,
                &self.ctx,
                File::create(out_dir.join(name))?
            ).map_err(Report::new)
        }).collect()
    }
}
