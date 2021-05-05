use luthien_plugin::palette::{self, white_point};
use luthien_plugin::Palette;
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
    palette: Palette<Color>,
    background: Color,
    foreground: Color,
}

#[derive(Debug, Clone, Serialize)]
pub struct Data {
    wallpaper: Option<PathBuf>,
    colors: Colors,
}

impl From<luthien_plugin::Theme> for Data {
    fn from(raw: luthien_plugin::Theme) -> Self {
        Self {
            wallpaper: raw.wallpaper,
            colors: Colors {
                // TODO: Add light mode option
                foreground: raw.colors.black.into(),
                background: raw.colors.white.into(),
                palette: raw.colors.map(From::from),
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
