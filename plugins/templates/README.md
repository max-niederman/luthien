# Luthien Templates

`luthien-templates` renders user-defined Handlebars templates with Luthien theme data.

## Installation

Install using Cargo from Crates.io:

```bash
cargo install luthien-templates
```

## Configuration

Add a plugin to your Luthien config:

```toml
[[plugins]]
executable = "~/.cargo/bin/luthien-templates"
```

## Writing Templates

[Tera](https://tera.netlify.app/) templates in the plugin's config directory (should be something like `~/.config/luthien/plugins/templates`) are rendered. Tera templates' syntax is quite similar to Jinja2's, if you're familiar with it.

### Template Data

The data accessible to the template looks like this (in some TypeScript-like pseudocode):

```typescript
// The RGB channels are sRGB floating-point values from 0 to 1
type Color = {
  red: float;
  green: float;
  blue: float;
};

type Data = {
  wallpaper: string;
  colors: {
    palette: {
      black: Color;
      red: Color;
      green: Color;
      yellow: Color;
      blue: Color;
      purple: Color;
      cyan: Color;
      white: Color;
    };
    accents: Color[];
    foreground: Color;
    background: Color;
  };
};
```

`colors.accents` is a list of accent colors in descending order of importance, so low-indexed colors should be used more often. There should be at least 6 accents.

### Filters

#### Color Encoding

Almost no application will take colors in raw sRGB channels, so the color data type on its own isn't very useful. To remedy this, we can use the `hex` filter to turn it into a hex code:

```
{{ colors.foreground | hex }}
```

would be rendered to something like this:

```
#ffffff
```

we can also pass an optional `prefix` argument to change or remove the `#` prefix.

#### Color Adjustment

The `adjust` filter adjusts colors using the following optional arguments:

- `lighten` and `darken`: floating-point shading amount from 0 to 1
- `saturate` and `desaturate`: floating-point saturation factor from 0 to 1
- `with_hue`: set the hue to a specific rotation in degrees
- `shift_hue`: shift the hue by a specified floating-point number of degrees

### Example

A shell script which sets variables using the template data:

```bash
wallpaper='{{ wallpaper }}'

background='{{ colors.background | hex }}'
foreground='{{ colors.foreground | hex }}'

black='{{ colors.palette.black | hex }}'
red='{{ colors.palette.red | hex }}'
green='{{ colors.palette.green | hex }}'
yellow='{{ colors.palette.yellow | hex }}'
blue='{{ colors.palette.blue | hex }}'
purple='{{ colors.palette.purple | hex }}'
cyan='{{ colors.palette.cyan | hex }}'
white='{{ colors.palette.white | hex }}'

accent0='{{ colors.accents.0 | hex }}'
accent1='{{ colors.accents.1 | hex }}'
accent2='{{ colors.accents.2 | hex }}'
accent3='{{ colors.accents.3 | hex }}'
accent4='{{ colors.accents.4 | hex }}'
accent5='{{ colors.accents.5 | hex }}'
```

## Outputs

The rendered template is then copied into a file in the output directory (should be something like `~/.local/share/luthien/outputs/plugins/templates` on Unix-like systems).
