mod render;

use color_eyre::eyre::{eyre, Result};
use render::Renderer;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = luthien_plugin::get_input().ok_or_else(|| eyre!("Plugin input was malformed"))?;
    let renderer = Renderer::from_input(&input)?;

    let ress = renderer.render(input.directories.output);

    for err in ress.into_iter().filter_map(Result::err) {
        eprintln!("{:?}", err)
    }

    Ok(())
}
