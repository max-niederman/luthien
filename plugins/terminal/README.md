# Luthien Terminal

`luthien-terminal` automatically sets the color schemes of open terminals from the Luthien theme. It also outputs a list of control sequences which can be used to set the colors of new terminals as well.

_Unfortunately, `luthien-terminal` only works on Unix-like systems_.

## Installation

Install using Cargo from Crates.io:

```bash
cargo install luthien-terminal
```

## Configuration

Add a plugin to your Luthien config:

```toml
[[plugins]]
executable = "~/.cargo/bin/luthien-terminal"
```

## Usage

Now when you run Luthien, the colorschemes of open terminals will be set automatically. However, if you open a new terminal, it won't have the correct colorscheme. This can be remedied by sending the correct control sequences to the terminal when it's opened.

Add the following to your `~/.bashrc` or similar:

```bash
cat ~/.local/share/luthien/outputs/plugins/terminal/sequences
```
