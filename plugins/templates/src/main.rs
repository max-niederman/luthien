mod render;

use color_eyre::eyre::{Result, WrapErr};
use render::Renderer;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = luthien_plugin::get_input().wrap_err("Failed to parse plugin input")?;
    let renderer =
        Renderer::from_input(&input).wrap_err("Failed to instantiate template renderer")?;

    let ress = renderer.render(input.directories.output);

    for (name, res) in ress.into_iter() {
        if let Err(err) = res {
            eprintln!(
                "{:?}",
                err.wrap_err(format!("Failed processing template '{}'", name))
            )
        }
    }

    Ok(())
}
