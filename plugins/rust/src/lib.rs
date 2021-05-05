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

use ipipe::{OnCleanup, Pipe};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

pub use ipipe;
pub use palette;

/// Colored palette of generic data.
///
/// Here, this is only used for [`palette::Srgb`], but it can also be used to further process the
/// given colors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// Represents the theme which is passed to the plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub wallpaper: Option<PathBuf>,
    pub colors: Palette<palette::Srgb>,
}

/// Represents the data givent to the plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub theme: Theme,
    pub options: serde_json::Value,
    pipe: Option<PathBuf>,
}

impl Input {
    /// Get an [`ipipe::Pipe`] which can be used to read input and print output to the parent
    /// Luthien process.
    pub fn io(&self) -> Option<ipipe::Result<Pipe>> {
        self.pipe
            .as_ref()
            .map(|path| Pipe::open(path, OnCleanup::NoDelete))
    }
}

/// Get the plugin's input.
///
/// ## Panics
/// Panics when [`serde_json`] is unable to deserialize the input, presumably because it was
/// incorrect.
pub fn get_input() -> Input {
    serde_json::from_reader(&mut io::stdin()).unwrap()
}
