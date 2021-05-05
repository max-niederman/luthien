mod data;
mod error;

use data::Data;
use error::Error;
use handlebars::Handlebars;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

struct Paths {
    templates: PathBuf,
    output: PathBuf,
}

impl From<&luthien_plugin::Input> for Paths {
    fn from(input: &luthien_plugin::Input) -> Self {
        Self {
            templates: input
                        .options
                        .get("template_dir")
                        .map(serde_json::Value::as_str)
                        .flatten()
                        .map(PathBuf::from)
                        .unwrap_or_else(
                            || dirs::config_dir()
                                        .expect("Couldn't find the luthien config directory. Please specify the template directory manually.")
                                        .join("luthien")
                                        .join("templates")
                        ),
            output: input
                        .options
                        .get("output_dir")
                        .map(serde_json::Value::as_str)
                        .flatten()
                        .map(PathBuf::from)
                        .unwrap_or_else(
                            || dirs::cache_dir()
                                        .expect("Couldn't find the user cache directory. Please specify the output directory manually.")
                                        .join("luthien")
                                        .join("templates")
                        ),
        }
    }
}

impl Paths {
    fn ensure_initialized(&self) -> error::Result<()> {
        fs::create_dir_all(&self.templates)?;
        fs::create_dir_all(&self.output)?;
        Ok(())
    }

    fn render_template(
        &self,
        name: &OsStr,
        hb: &Handlebars,
        ctx: &handlebars::Context,
    ) -> error::Result<()> {
        let template = fs::read_to_string(self.templates.join(name))?;
        let rendered = hb.render_template_with_context(&template, ctx)?;
        fs::write(self.output.join(name), rendered)?;
        Ok(())
    }

    fn render_all_templates(
        &self,
        hb: &Handlebars,
        ctx: &handlebars::Context,
    ) -> error::Result<(usize, usize)> {
        let mut count = (0, 0);

        for entry in fs::read_dir(&self.templates)? {
            let entry = entry?;
            match self.render_template(&entry.file_name(), hb, ctx) {
                Ok(_) => count.0 += 1,
                Err(Error::TemplateError(err)) => eprintln!(
                    "Template {} is invalid:\n{:?}",
                    entry.file_name().to_str().unwrap(),
                    err
                ),
                Err(err) => return Err(err),
            }
            count.1 += 1;
        }

        Ok(count)
    }
}

fn main() -> error::Result<()> {
    let input = luthien_plugin::get_input();
    let data = serde_json::to_value(Data::from(input.theme.clone()))
        .expect("Failed to transform plugin input to template data.");
    let paths = Paths::from(&input);

    paths.ensure_initialized()?;

    let template_count =
        paths.render_all_templates(&Handlebars::new(), &handlebars::Context::wraps(data)?)?;

    eprintln!(
        "Successfully rendered {}/{} templates.",
        template_count.0, template_count.1
    );
    Ok(())
}
