use luthien_plugin::palette::Srgb;
use luthien_plugin::*;
use serde_json::json;
use std::path::PathBuf;

macro_rules! test_json {
    () => {
        json!({
          "pipe": "/tmp/luthien_plugin_stdio_12345",
          "directories": {
            "config": "/home/user/.config/luthien/plugins/test",
            "output": "/home/user/.local/share/luthien/outputs/test",
            "cache": "/home/user/.cache/luthien/plugins/test",
            "data": "/home/user/.local/share/luthien/plugins/test"
          },
          "name": "test",
          "options": {
            "foo": "string",
            "bar": 0.0,
            "baz": {}
          },
          "theme": {
            "wallpaper": "/home/user/pictures/wallpaper.jpg",
            "colors": {
              "palette": {
                "black": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "red": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "green": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "yellow": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "blue": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "purple": {
                  "red": 0.0,
                  "green": 0.0,
                  "blue": 0.0
                },
                "cyan": { "red": 0.0, "green": 0.0, "blue": 0.0 },
                "white": { "red": 0.0, "green": 0.0, "blue": 0.0 }
              },
              "accents": [
                { "red": 0.0, "green": 0.0, "blue": 0.0 },
                { "red": 0.0, "green": 0.0, "blue": 0.0 },
                { "red": 0.0, "green": 0.0, "blue": 0.0 },
                { "red": 0.0, "green": 0.0, "blue": 0.0 },
                { "red": 0.0, "green": 0.0, "blue": 0.0 },
                { "red": 0.0, "green": 0.0, "blue": 0.0 }
              ],
              "foreground": {
                "red": 0.0,
                "green": 0.0,
                "blue": 0.0
              },
              "background": { "red": 0.0, "green": 0.0, "blue": 0.0 }
            }
          }
        });
    };
}

macro_rules! test_correct {
    () => {
        Input {
            pipe_path: Some(PathBuf::from("/tmp/luthien_plugin_stdio_12345")),
            directories: Directories {
                config: PathBuf::from("/home/user/.config/luthien/plugins/test"),
                output: PathBuf::from("/home/user/.local/share/luthien/outputs/test"),
                cache: PathBuf::from("/home/user/.cache/luthien/plugins/test"),
                data: PathBuf::from("/home/user/.local/share/luthien/plugins/test"),
            },
            name: "test".into(),
            options: json!({
                "foo": "string",
                "bar": 0.0,
                "baz": {}
            }),
            theme: Theme {
                wallpaper: Some(PathBuf::from("/home/user/pictures/wallpaper.jpg")),
                colors: Colors {
                    palette: Palette::uniform(Srgb::new(0.0, 0.0, 0.0)),
                    accents: [Srgb::new(0.0, 0.0, 0.0); 6].into(),
                    foreground: Srgb::new(0.0, 0.0, 0.0),
                    background: Srgb::new(0.0, 0.0, 0.0),
                }
            }
        }
    };
}

#[test]
fn parse() {
    let bytes = serde_json::to_vec(&test_json!()).unwrap();
    let input: Input = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(input, test_correct!(),);
}
