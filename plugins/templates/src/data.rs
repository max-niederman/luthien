use luthien_plugin::palette::{self, white_point};
use luthien_plugin::{Theme, Palette, ColorMode};
use serde::Serialize;
use std::path::PathBuf;

type WhitePoint = white_point::D65;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Color {
    hex: String,
    hex_stripped: String,
    red: f32,
    green: f32,
    blue: f32,
}

impl<C> From<C> for Color
where
    C: palette::IntoColor<WhitePoint, f32>,
{
    fn from(col: C) -> Self {
        let rgb = col.into_rgb::<palette::encoding::Srgb>();

        let hex = format!(
            "{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
            (rgb.blue * 255.0) as u8
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
pub struct Colors {
    mode: luthien_plugin::ColorMode,
    palette: Palette<Color>,
    background: Color,
    foreground: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct Data {
    wallpaper: Option<PathBuf>,
    colors: Colors,
}

impl From<Theme> for Data {
    fn from(raw: Theme) -> Self {
        let colors = match raw.colors.mode {
            ColorMode::Dark => Colors {
                mode: raw.colors.mode,
                foreground: raw.colors.palette.white.into(),
                background: raw.colors.palette.black.into(),
                palette: raw.colors.palette.map(From::from),
            },
            ColorMode::Light => Colors {
                mode: raw.colors.mode,
                foreground: raw.colors.palette.black.into(),
                background: raw.colors.palette.white.into(),
                palette: raw.colors.palette.map(From::from),
            },
        };

        Self {
            wallpaper: raw.wallpaper,
            colors,
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
