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

Mustach templates in the plugin's config directory (should be something like `~/.config/luthien/plugins/templates`) will be rendered using the theme data.

### Template Data

The data accessible to the template looks like this (in some TypeScript-like pseudocode):

```typescript
type Color = {
  hex: string;
  hex_stripped: string;
  red: number;
  green: number;
  blue: number;
};

// The RGB channels are sRGB floating-point values from 0 to 1

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

## Outputs

The rendered template is then copied into a file in the output directory (should be something like `~/.local/share/luthien/outputs/plugins/templates` on Unix-like systems).
