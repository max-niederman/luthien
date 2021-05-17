use crate::palette::Srgb;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Colored palette of generic data.
///
/// Here, this is only used for [`Srgb`], but it can also be used to further process the
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

    /// Convert a reference to a [`Palette<T>`] to a [`Palette<&T>`].
    pub fn as_ref(&self) -> Palette<&T> {
        Palette {
            black: &self.black,
            red: &self.red,
            green: &self.green,
            yellow: &self.yellow,
            blue: &self.blue,
            purple: &self.purple,
            cyan: &self.cyan,
            white: &self.white,
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

    /// Apply a function to each element of this [`Palette`], creating another palette with the
    /// return values.
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

/// The [`Theme`]'s colors.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Colors<Color = Srgb> {
    pub palette: Palette<Color>,
    pub accents: Vec<Color>,
    pub foreground: Color,
    pub background: Color,
}

/// A theme passed to the plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub wallpaper: Option<PathBuf>,
    pub colors: Colors,
}
