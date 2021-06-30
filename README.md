# Luthien

[![Crates.io](https://img.shields.io/crates/v/luthien)](https://crates.io/crates/luthien)
[![GitHub Stars](https://img.shields.io/github/stars/max-niederman/luthien)](https://github.com/max-niederman/luthien)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/max-niederman/luthien/Rust)](https://github.com/max-niederman/luthien/actions)
[![GitHub issues](https://img.shields.io/github/issues/max-niederman/luthien)](https://github.com/max-niederman/luthien/issues)
[![License](https://img.shields.io/crates/l/luthien)](./LICENSE.md)

Luthien is a WIP tool which generates color schemes and applying them to your system. It strives to produce beautiful color schemes and be highly extensible.

## Features

- Luthien produces "normal" color schemes suitable for use in any application, with generated colors of every hue. Many palette generation tools produce palettes which are mostly the same color; this is not the case with Luthien.
- Highly configurable plugin system which makes it easy to apply your themes to any application.
- Library of pre-made plugins for templating, terminals, and more.

## Installation

Luthien can be installed with Cargo via Crates.io:

```bash
cargo install luthien
```

## Usage

You can extract a theme from an image by using the `extract` subcommand with the `image` extractor like so:

```bash
luthien extract image path/to/image.jpg
```

You should get output that looks something like this:

```
 INFO  luthien::extraction > Cache hit; using cached theme...
 INFO  luthien             > Theme Preview:
...
 INFO  luthien             > Applying theme...
```

_NOTE: You may recognize this as output from Rust's `log` framework; and indeed, if you set `RUST_LOG=trace`, you'll get much more granular output._

Now, at this point, you might have noticed that nothing happened. This is because Luthien does nothing but generate themes on its own. If you want to get the theme, you can use the `--output` flag (or `-o` for short). If we did `luthien -o theme.json extract image path/to/image.jpg`, `theme.json` would look something like this:

```json
{
  "wallpaper": "path/to/image.jpg",
  "colors": {
    "palette": {
      "black": { "red": 0.0, "green": 0.0, "blue": 0.0 },
      "red": { "red": 1.0, "green": 0.0, "blue": 0.0 },
      "green": { "red": 0.0, "green": 1.0, "blue": 0.0 },
      "yellow": { "red": 1.0, "green": 1.0, "blue": 0.0 },
      "blue": { "red": 0.0, "green": 0.0, "blue": 1.0 },
      "purple": { "red": 1.0, "green": 0.0, "blue": 1.0 },
      "cyan": { "red": 0.0, "green": 1.0, "blue": 1.0 },
      "white": { "red": 1.0, "green": 1.0, "blue": 1.0 }
    },
    "accents": [
      { "red": 1.0, "green": 0.0, "blue": 0.0 },
      { "red": 0.0, "green": 1.0, "blue": 0.0 },
      { "red": 1.0, "green": 1.0, "blue": 0.0 },
      { "red": 0.0, "green": 0.0, "blue": 1.0 },
      { "red": 1.0, "green": 0.0, "blue": 1.0 },
      { "red": 0.0, "green": 1.0, "blue": 1.0 }
    ],
    "foreground": { "red": 1.0, "green": 1.0, "blue": 1.0 },
    "background": { "red": 0.0, "green": 0.0, "blue": 0.0 }
  }
}
```

We can see that the wallpaper is specified along with a set of colors. `colors.palette` contains the colors with their names; `colors.accents` is a list of colors in descending order of "importance" (the first accent should be featured more prominently than the last); and `colors.foreground` and `colors.background` are self-explanatory.

The output flag makes shell scripting relatively easy. You can do `-o /dev/stdout` and pipe the output through a JSON parser like [jq](https://stedolan.github.io/jq/) and into an application of your choosing.

### Plugins

Shell scripting is great, but most people will want to use the generated theme in multiple ways which stay about the same between each run. This is where plugins come in.

Plugins are added as a list of objects in the Luthien config file (on Unix-like systems, `~/.config/luthien/config.toml`). For example, we can add a plugins like this:

```toml
[[plugins]]
name = "echo"
executable = "/usr/bin/dd"
args = ["if=/dev/stdin", "of=/dev/stderr", "status=none"]
```

This will add a plugin named "echo," which executes `dd` and copies the plugin's input to stderr, which is inherited from the `luthien` process, so it'll be echoed back to the user. You can read more about writing and using plugins [here](https://github.com/max-niederman/luthien/wiki/Using-and-Developing-Plugins).

After a theme is generated, Luthien runs each plugin and passes the theme along with other data to them.
This enables the user to automate a huge amount of otherwise manual work when theming a system. For instance, you could write plugins to

- Theme your desktop environment/window manager.
- Update your terminal colors.
- Set your lighting strip's colors.

For ease of use, some first-party plugins are available:

- [`luthien-terminal`](./plugins/terminal): Generates terminal control sequences, sends them to open pseudoterminals, and saves them to a file to be read when new terminal are opened.
- [`luthien-templates`](./plugins/templates): Renders Handlebars templates with theme data.
- [`luthien-sass`](./plugins/templates): Exposes theme data to Sass modules and compiles them to CSS.
