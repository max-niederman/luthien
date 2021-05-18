# Luthien Stylesheets

`luthien-sass` enables you to generate complex stylesheets with Luthien theme data. Stylesheets are written in Sass, so complex color adjustment and logic is also possible.

## Installation

Install using Cargo from Crates.io:

```bash
cargo install luthien-sass
```

## Configuration

Add a plugin to your Luthien config:

```toml
[[plugins]]
executable = "~/.cargo/bin/luthien-sass"
```

## Writing Stylesheets

Sass stylesheets in the plugin's config directory (should be something like `~/.config/luthien/plugins/sass`) will be compiled using [grass](https://github.com/connorskees/grass). It has near feature-parity with Dart Sass; see its documention for more details.

To access theme data, you can import a module called `luthien`. This module is also output in the output directory (~/.local/share/luthien/outputs/plugins/sass/ on most Unix-like systems). The module contains the following variables:

- `luthien.$wallpaper`: A `url()` to the wallpaper image.
- `luthien.$palette`: A map of color names to colors. The following keys are included:
  - `black`
  - `red`
  - `green`
  - `yellow`
  - `blue`
  - `purple`
  - `cyan`
  - `white`
- `luthien.$accents`: A list of accent colors in descending order of importance. It should have at least six elements.
- `luthien.$foreground`: The foreground color.
- `luthien.$background`: The background color.

## Compilation

After a stylesheet is compiled with [grass](https://github.com/connorskees/grass), the output is written to a file with the same name in the output directory, but with the `css` extension. For instance a stylesheet named `colors.scss` would be output as a file named `colors.css`.
