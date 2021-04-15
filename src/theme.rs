use palette::{IntoColor, Srgb};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colors<C = Srgb> {
    black: C,
    red: C,
    green: C,
    yellow: C,
    blue: C,
    purple: C,
    cyan: C,
    white: C,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            black: Srgb::new(0.0, 0.0, 0.0),
            red: Srgb::new(1.0, 0.0, 0.0),
            green: Srgb::new(0.0, 1.0, 0.0),
            yellow: Srgb::new(1.0, 1.0, 0.0),
            blue: Srgb::new(0.0, 0.0, 1.0),
            purple: Srgb::new(1.0, 0.0, 1.0),
            cyan: Srgb::new(0.0, 1.0, 1.0),
            white: Srgb::new(1.0, 1.0, 1.0),
        }
    }
}

impl<C> fmt::Display for Colors<C>
where
    C: IntoColor + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn color_block<C: IntoColor>(col: C) -> impl fmt::Display {
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
    background: PathBuf,
    colors: Colors,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Background Image: {}",
            self.background.to_str().ok_or(fmt::Error)?
        )?;
        writeln!(f, "Color Palette: {}", self.colors)
    }
}

#[cfg(test)]
mod tests {
    use super::{Colors, Theme};
    use std::path::PathBuf;

    fn test_theme() -> Theme {
        Theme {
            background: PathBuf::from("test.jpg"),
            colors: Colors::default(),
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
                "Background Image: {}\nColor Palette: {}\n",
                test_theme().background.to_str().unwrap(),
                test_theme().colors
            )
        )
    }
}
