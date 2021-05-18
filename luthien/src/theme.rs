use palette::{IntoColor, Srgb};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

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
        fn color_block<T: IntoColor + Clone>(col: T) -> impl fmt::Display {
            use colored::*;

            let col = col
                .into_rgb::<palette::encoding::Srgb>()
                .into_encoding::<palette::encoding::Srgb>();
            "  ".on_truecolor(
                (col.red * 0xFF as f32) as u8,
                (col.green * 0xFF as f32) as u8,
                (col.blue * 0xFF as f32) as u8,
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
pub struct Colors<Color = Srgb> {
    pub palette: Palette<Color>,
    pub accents: Vec<Color>,
    pub foreground: Color,
    pub background: Color,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Theme {
    pub wallpaper: Option<PathBuf>,
    pub colors: Colors,
}

// TODO: Implement [`Display`] for [`Colors`] and use here.
impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(bg) = &self.wallpaper {
            writeln!(f, "Wallpaper: {}", bg.to_str().ok_or(fmt::Error)?)?;
        }
        write!(f, "Color Palette: {}", self.colors.palette)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;
    use std::path::PathBuf;

    macro_rules! test_theme {
        () => {{
            let palette = Palette {
                black: Srgb::new(0.0, 0.0, 0.0),
                red: Srgb::new(1.0, 0.0, 0.0),
                green: Srgb::new(0.0, 1.0, 0.0),
                yellow: Srgb::new(1.0, 1.0, 0.0),
                blue: Srgb::new(0.0, 0.0, 1.0),
                purple: Srgb::new(1.0, 0.0, 1.0),
                cyan: Srgb::new(0.0, 1.0, 1.0),
                white: Srgb::new(1.0, 1.0, 1.0),
            };
            Theme {
                wallpaper: Some(PathBuf::from("test.jpg")),
                colors: Colors {
                    accents: vec![
                        palette.red.clone(),
                        palette.green.clone(),
                        palette.yellow.clone(),
                        palette.blue.clone(),
                        palette.purple.clone(),
                        palette.cyan.clone(),
                    ],
                    foreground: palette.white.clone(),
                    background: palette.black.clone(),

                    palette,
                },
            }
        }};
    }

    #[test]
    fn serialization() {
        let serialized = serde_json::to_string(&test_theme!()).expect("Failed to serialize.");
        let deserialized = serde_json::from_str(&serialized).expect("Failed to deserialize.");

        assert_eq!(test_theme!(), deserialized);
    }

    #[test]
    fn display() {
        format!("{}", test_theme!().colors.palette);
        format!("{}", test_theme!());
    }
}
