use luthien_plugin::{Palette, ColorMode, Theme};
use std::fs;
use std::io;

trait ControlSequence {
    fn sequence(&self) -> Vec<u8>;
}

impl ControlSequence for Theme {
    fn sequence(&self) -> Vec<u8> {
        enum Sequence {
            Regular(u16),
            Special(u16),
        }

        impl Sequence {
            fn sequence(&self, color: &str) -> String {
                match self {
                    Self::Regular(idx) => format!("\x1b]4;{};{}\x1b\\", idx, color),
                    Self::Special(idx) => format!("\x1b]{};{}\x1b\\", idx, color),
                }
            }
        }

        let mut ret = Vec::with_capacity(8 * 12);

        let indexes = {
            use Sequence::*;
            match self.colors.mode {
              ColorMode::Dark => Palette {
                    black: vec![
                        Regular(0),
                        Regular(8),
                        Special(11),
                        Special(19),
                        Special(232),
                    ],
                    red: vec![Regular(1), Regular(9)],
                    green: vec![Regular(2), Regular(10)],
                    yellow: vec![Regular(3), Regular(11)],
                    blue: vec![Regular(4), Regular(12)],
                    purple: vec![Regular(5), Regular(13)],
                    cyan: vec![Regular(6), Regular(14)],
                    white: vec![
                        Regular(7),
                        Regular(15),
                        Special(10),
                        Special(13),
                        Special(17),
                        Special(256),
                    ],
                },
            ColorMode::Light => Palette {
                    black: vec![
                        Regular(0),
                        Regular(8),
                        Special(10),
                        Special(13),
                        Special(17),
                        Special(256),
                    ],
                    red: vec![Regular(1), Regular(9)],
                    green: vec![Regular(2), Regular(10)],
                    yellow: vec![Regular(3), Regular(11)],
                    blue: vec![Regular(4), Regular(12)],
                    purple: vec![Regular(5), Regular(13)],
                    cyan: vec![Regular(6), Regular(14)],
                    white: vec![
                        Regular(7),
                        Regular(15),
                        Special(11),
                        Special(19),
                        Special(232),
                    ],
                }
            }
        };

        self.colors.palette.clone().zip(indexes).map(|(col, codes)| {
            ret.extend(codes.iter().flat_map(|code| {
                code.sequence(&format!(
                    "#{:02x}{:02x}{:02x}",
                    (col.red * 0xFF as f32) as u8,
                    (col.green * 0xFF as f32) as u8,
                    (col.blue * 0xFF as f32) as u8
                ))
                .into_bytes()
            }))
        });

        ret
    }
}

fn main() -> io::Result<()> {
    let input = luthien_plugin::get_input();

    #[cfg(not(target_family = "unix"))]
    panic!("This plugin only works on Unixish systems.");

    let sequence = input.theme.sequence();

    fs::write(
        input.directories.output.join("sequences"),
        &sequence,
    )?;

    let mut count = 0;
    for entry in fs::read_dir("/dev/pts")? {
        let entry = entry?;
        if entry.file_name() != "ptmx" {
            if fs::write(entry.path(), &sequence).is_ok() {
                count += 1;
            }
        }
    }

    eprintln!("Set colors in {} terminals.", count);
    Ok(())
}
