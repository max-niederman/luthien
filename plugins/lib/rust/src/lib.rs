//! # Luthien Plugins in Rust
//!
//! `luthien-plugin` is a Rust library for writing Luthien plugins in Rust. It is not to be
//! confused with `luthien` itself.
//!
//! ## Input Deserialization
//! `luthien-plugin` provides plugin input data structures which can be deserialized with Serde, as well
//! as a utility function to get the input from stdin.
//!
//! ## Luthien IO
//! Luthien provides a named pipe which copies to and from its stdout and stdin respectively.
//! `luthien-plugin` can automatically get this pipe for you.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod theme;

#[cfg(feature = "io")]
pub mod io;

pub use theme::{Colors, Palette, Theme};

pub use serde_json;

#[cfg(feature = "palette")]
pub use palette;
#[cfg(not(feature = "palette"))]
pub mod palette {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Srgb<T = f32> {
        pub red: T,
        pub green: T,
        pub blue: T,
    }

    impl<T> Srgb<T> {
        pub const fn new(red: T, green: T, blue: T) -> Self {
            Self { red, green, blue }
        }
    }
}

/// The directories in which the plugin should store and output data.
///
/// These directories are guarunteed to exist and are exclusive to each plugin.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Directories {
    /// For user configuration of the plugin.
    pub config: PathBuf,
    /// For plugin outputs.
    pub output: PathBuf,
    /// For cached reproducible data.
    pub cache: PathBuf,
    /// For miscellaneous plugin data.
    pub data: PathBuf,
}

/// All data passed to the plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "pipe")]
    pub pipe_path: Option<PathBuf>,
    /// Directories which can be used to store data between runs.
    pub directories: Directories,

    /// Unique human-readable name.
    pub name: String,
    /// User-provided options.
    pub options: serde_json::Value,
    /// The provided theme.
    pub theme: Theme,
}

/// Get the plugin's input or [`None`] if it couldn't be deserialized.
pub fn get_input() -> Option<Input> {
    serde_json::from_reader(&mut std::io::stdin()).ok()
}
