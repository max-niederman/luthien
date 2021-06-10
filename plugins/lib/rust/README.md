# Luthien Plugins in Rust

This is a library for writing Luthien plugins in Rust.

## Usage

Add the library to your Cargo.toml:

```toml
[dependencies]
luthien-plugin = "0.1"
```

You can view the documentation [here](https://docs.rs/luthien-plugin) on docs.rs.

## Feature Flags

`luthien-plugin` has two feature flags:

- `io`: Read and write to the I/O pipe.
- `palette`: Deserialize input colors to `palette::Srgb` for adjustments/space transformation.

By default, `io` is enabled and `palette` is disabled.
