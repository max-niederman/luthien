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

#[cfg(feature = "io")]
pub mod io;

pub use serde_json;

#[cfg(feature = "palette")]
pub use palette;

#[cfg(not(feature = "palette"))]
pub mod palette {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Srgb<T = f32> {
        pub red: T,
        pub green: T,
        pub blue: T,
    }
}

/// Colored palette of generic data.
///
/// Here, this is only used for [`palette::Srgb`], but it can also be used to further process the
/// given colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Palette<T> {
    pub black: T,
    pub red: T,
    pub green: T,
    pub yellow: T,
    pub blue: T,
    pub purple: T,
    pub cyan: T,
    pub white: T,
}

impl<T> Palette<T> {
    /// Returns a [`Palette`] where every element is one value.
    pub fn uniform(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            black: v.clone(),
            red: v.clone(),
            green: v.clone(),
            yellow: v.clone(),
            blue: v.clone(),
            purple: v.clone(),
            cyan: v.clone(),
            white: v,
        }
    }

    /// Zip this [`Palette`] with another, returning a [`Palette`] of tuples.
    pub fn zip<U>(self, other: Palette<U>) -> Palette<(T, U)> {
        Palette {
            black: (self.black, other.black),
            red: (self.red, other.red),
            green: (self.green, other.green),
            yellow: (self.yellow, other.yellow),
            blue: (self.blue, other.blue),
            purple: (self.purple, other.purple),
            cyan: (self.cyan, other.cyan),
            white: (self.white, other.white),
        }
    }

    pub fn map<F, U>(self, mut f: F) -> Palette<U>
    where
        F: FnMut(T) -> U,
    {
        Palette {
            black: f(self.black),
            red: f(self.red),
            green: f(self.green),
            yellow: f(self.yellow),
            blue: f(self.blue),
            purple: f(self.purple),
            cyan: f(self.cyan),
            white: f(self.white),
        }
    }
}

/// The color "mode." This is used to distinguish whether the user prefers dark or light themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    Dark,
    Light,
}

/// The [`Theme`]'s colors.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Colors {
    pub mode: ColorMode,
    #[serde(flatten)]
    pub palette: Palette<palette::Srgb>,
}

/// A theme passed to the plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub wallpaper: Option<PathBuf>,
    pub colors: Colors,
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
    pipe: Option<PathBuf>,
    /// Directories which can be used to store data between runs.
    pub directories: Directories,

    /// Unique human-readable name.
    pub name: String,
    /// User-provided options.
    pub options: serde_json::Value,
    /// The provided theme.
    pub theme: Theme,
}

/// Get the plugin's input.
///
/// ## Panics
/// Panics when [`serde_json`] is unable to deserialize the input.
pub fn get_input() -> Input {
    serde_json::from_reader(&mut std::io::stdin()).unwrap()
}
