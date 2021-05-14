mod data;
mod error;
mod hooks;

use data::Data;
use error::Error;
use handlebars::Handlebars;
use std::ffi::OsStr;
use std::fs;

trait Environment {
    fn render_template(
        &self,
        name: &OsStr,
        hb: &Handlebars,
        ctx: &handlebars::Context,
    ) -> error::Result<()>;
    fn render_all_templates(
        &self,
        hb: &Handlebars,
        ctx: &handlebars::Context,
    ) -> error::Result<(usize, usize)>;
}

impl Environment for luthien_plugin::Directories {
    fn render_template(
        &self,
        name: &OsStr,
        hb: &Handlebars,
        ctx: &handlebars::Context,
    ) -> error::Result<()> {
        let template = fs::read_to_string(self.config.join(name))?;
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

        for entry in fs::read_dir(&self.config)? {
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
    let input = luthien_plugin::get_input()
        .expect("Input was malformed. Try updating this plugin and/or Luthien.");
    let env = input.directories;
    let data = serde_json::to_value(Data::from(input.theme.clone()))
        .expect("Failed to transform plugin input to template data.");

    let template_count =
        env.render_all_templates(&Handlebars::new(), &handlebars::Context::wraps(data)?)?;

    eprintln!(
        "Successfully rendered {}/{} templates.",
        template_count.0, template_count.1
    );

    Ok(())
}
