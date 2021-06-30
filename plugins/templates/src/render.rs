use color_eyre::eyre::{Report, Result, WrapErr};
use luthien_plugin::Input;
use std::{
    fs::{self, File},
    path::PathBuf,
};
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

                tr.register_filter("adjust", filters::adjust);
                tr.register_filter("hex", filters::hex);

                tr.add_template_files(
                    fs::read_dir(&input.directories.config)
                        .wrap_err("Failed to read template directory")?
                        .filter_map(Result::ok)
                        .map(|e| (e.path(), e.file_name().into_string().ok()))
                        .collect::<Vec<(PathBuf, Option<String>)>>(),
                )?;

                tr
            },
            ctx: tera::Context::from_serialize(&input.theme)
                .wrap_err("Failed to create template rendering context")?,
        })
    }

    pub fn render(&self, out_dir: PathBuf) -> Vec<(String, Result<()>)> {
        self.tera
            .get_template_names()
            .map(String::from)
            .zip(self.tera.get_template_names().map(|name| {
                self.tera
                    .render_to(
                        name,
                        &self.ctx,
                        File::create(out_dir.join(name))
                            .wrap_err("Failed to create output file")?,
                    )
                    .map_err(Report::new)
                    .wrap_err("Tera failed rendering the template")
            }))
            .collect()
    }
}

mod filters {
    use luthien_plugin::palette::{self, FromColor, Hsl, Srgb};
    use std::collections::HashMap;
    use tera::{Result, Value};

    type Args = HashMap<String, Value>;

    pub fn adjust(val: &tera::Value, args: &Args) -> Result<Value> {
        let mut col: Hsl = tera::from_value::<Srgb>(val.clone())?.into();

        macro_rules! adjustment {
            ($key:expr, $func:expr) => {{
                if let Some(arg) = args.get($key).and_then(Value::as_f64) {
                    col = $func(&col, arg as f32);
                }
            }};
        }

        adjustment!("lighten", palette::Shade::lighten);
        adjustment!("darken", palette::Shade::darken);
        adjustment!("saturate", palette::Saturate::saturate);
        adjustment!("desaturate", palette::Saturate::desaturate);
        adjustment!("with_hue", palette::Hue::with_hue);
        adjustment!("shift_hue", palette::Hue::shift_hue);

        Ok(tera::to_value(Srgb::from_hsl(col))?)
    }

    pub fn hex(val: &tera::Value, args: &Args) -> Result<Value> {
        let col: Srgb = tera::from_value(val.clone())?;
        let prefix: &str = args.get("prefix").and_then(Value::as_str).unwrap_or("#");

        Ok(format!(
            "{}{:02x}{:02x}{:02x}",
            prefix,
            (col.red * 0xFF as f32) as u8,
            (col.green * 0xFF as f32) as u8,
            (col.blue * 0xFF as f32) as u8
        )
        .into())
    }
}
