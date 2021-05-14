use luthien_plugin::{palette::Srgb, Colors, Theme};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Color {
    hex: String,
    hex_stripped: String,
    red: f32,
    green: f32,
    blue: f32,
}

impl From<Srgb> for Color {
    fn from(rgb: Srgb) -> Self {
        let hex = format!(
            "{:02x}{:02x}{:02x}",
            (rgb.red * 0xFF as f32) as u8,
            (rgb.green * 0xFF as f32) as u8,
            (rgb.blue * 0xFF as f32) as u8
        );
        Self {
            hex: format!("#{}", hex),
            hex_stripped: hex,
            red: rgb.red,
            green: rgb.green,
            blue: rgb.blue,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Data {
    wallpaper: Option<PathBuf>,
    colors: Colors<Color>,
}

impl From<Theme> for Data {
    fn from(theme: Theme) -> Self {
        Self {
            wallpaper: theme.wallpaper,
            colors: Colors {
                palette: theme.colors.palette.map(Into::into),
                accents: theme.colors.accents.into_iter().map(Into::into).collect(),
                foreground: theme.colors.foreground.into(),
                background: theme.colors.background.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_color() {
        use luthien_plugin::palette::Srgb;

        assert_eq!(
            Color::from(Srgb::new(0.0, 0.0, 0.0)),
            Color {
                hex: "#000000".into(),
                hex_stripped: "000000".into(),
                red: 0.0,
                green: 0.0,
                blue: 0.0,
            }
        );
        assert_eq!(
            Color::from(Srgb::new(1.0, 1.0, 1.0)),
            Color {
                hex: "#ffffff".into(),
                hex_stripped: "ffffff".into(),
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            }
        );
    }
}
