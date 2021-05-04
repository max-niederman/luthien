use palette::{IntoColor, Srgb};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl<T: Default> Default for Palette<T> {
    fn default() -> Self {
        Self {
            black: Default::default(),
            red: Default::default(),
            green: Default::default(),
            yellow: Default::default(),
            blue: Default::default(),
            purple: Default::default(),
            cyan: Default::default(),
            white: Default::default(),
        }
    }
}

impl<T> Palette<T> {
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

    pub fn map<F, R>(self, mut f: F) -> Palette<R>
    where
        F: FnMut(T) -> R,
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

impl<T> fmt::Display for Palette<T>
where
    T: IntoColor + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn color_block<T: IntoColor>(col: T) -> impl fmt::Display {
            use colored::*;

            let rgb = col.into_rgb::<palette::encoding::Srgb>().into_components();
            " ".on_truecolor(
                (rgb.0 * 0xFF as f32) as u8,
                (rgb.1 * 0xFF as f32) as u8,
                (rgb.2 * 0xFF as f32) as u8,
            )
        }

        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            color_block(self.black.clone()),
            color_block(self.red.clone()),
            color_block(self.green.clone()),
            color_block(self.yellow.clone()),
            color_block(self.blue.clone()),
            color_block(self.purple.clone()),
            color_block(self.cyan.clone()),
            color_block(self.white.clone()),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Theme {
    pub background: PathBuf,
    pub colors: Palette<Srgb>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Background Image: {}",
            self.background.to_str().ok_or(fmt::Error)?
        )?;
        write!(f, "Color Palette: {}", self.colors)
    }
}

#[cfg(test)]
mod tests {
    use super::{Palette, Theme};
    use palette::Srgb;
    use std::path::PathBuf;

    fn test_theme() -> Theme {
        Theme {
            background: PathBuf::from("test.jpg"),
            colors: Palette {
                black: Srgb::new(0.0, 0.0, 0.0),
                red: Srgb::new(1.0, 0.0, 0.0),
                green: Srgb::new(0.0, 1.0, 0.0),
                yellow: Srgb::new(1.0, 1.0, 0.0),
                blue: Srgb::new(0.0, 0.0, 1.0),
                purple: Srgb::new(1.0, 0.0, 1.0),
                cyan: Srgb::new(0.0, 1.0, 1.0),
                white: Srgb::new(1.0, 1.0, 1.0),
            },
        }
    }

    #[test]
    fn serialization() {
        let serialized = serde_json::to_string(&test_theme()).expect("Failed to serialize.");
        let deserialized = serde_json::from_str(&serialized).expect("Failed to deserialize.");

        assert_eq!(test_theme(), deserialized,)
    }

    #[test]
    fn display_colors() {
        assert_eq!(
            format!("{}", test_theme().colors),
            "\u{1b}[48;2;0;0;0m \u{1b}[0m\u{1b}[48;2;255;0;0m \u{1b}[0m\u{1b}[48;2;0;255;0m \u{1b}[0m\u{1b}[48;2;255;255;0m \u{1b}[0m\u{1b}[48;2;0;0;255m \u{1b}[0m\u{1b}[48;2;255;0;255m \u{1b}[0m\u{1b}[48;2;0;255;255m \u{1b}[0m\u{1b}[48;2;255;255;255m \u{1b}[0m"
        )
    }

    #[test]
    fn display_theme() {
        assert_eq!(
            format!("{}", test_theme()),
            format!(
                "Background Image: {}\nColor Palette: {}",
                test_theme().background.to_str().unwrap(),
                test_theme().colors
            )
        )
    }
}
