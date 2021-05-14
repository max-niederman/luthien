use luthien_plugin::{palette::Srgb, Palette, Theme};
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
        use Sequence::*;

        impl Sequence {
            fn sequence(&self, color: &Srgb) -> String {
                let hex = format!(
                    "#{:02x}{:02x}{:02x}",
                    (color.red * 0xFF as f32) as u8,
                    (color.green * 0xFF as f32) as u8,
                    (color.blue * 0xFF as f32) as u8
                );
                match self {
                    Self::Regular(idx) => format!("\x1b]4;{};{}\x1b\\", idx, hex),
                    Self::Special(idx) => format!("\x1b]{};{}\x1b\\", idx, hex),
                }
            }
        }

        let mut ret = Vec::with_capacity(8 * 12);

        let indexes = Palette {
            black: [Regular(0), Regular(8)],
            red: [Regular(1), Regular(9)],
            green: [Regular(2), Regular(10)],
            yellow: [Regular(3), Regular(11)],
            blue: [Regular(4), Regular(12)],
            purple: [Regular(5), Regular(13)],
            cyan: [Regular(6), Regular(14)],
            white: [Regular(7), Regular(15)],
        };

        self.colors
            .palette
            .as_ref()
            .zip(indexes)
            .map(|(col, codes)| {
                ret.extend(
                    codes
                        .iter()
                        .flat_map(|code| code.sequence(col).into_bytes()),
                )
            });

        ret.extend(
            [Special(11), Special(19), Special(232)]
                .iter()
                .flat_map(|code| code.sequence(&self.colors.background).into_bytes()),
        );
        ret.extend(
            [Special(10), Special(13), Special(17), Special(256)]
                .iter()
                .flat_map(|code| code.sequence(&self.colors.foreground).into_bytes()),
        );

        ret
    }
}

fn main() -> io::Result<()> {
    let input = luthien_plugin::get_input()
        .expect("Input was malformed. Try updating this plugin and/or Luthien.");

    #[cfg(not(target_family = "unix"))]
    panic!("This plugin only works on Unixish systems.");

    let sequence = input.theme.sequence();

    fs::write(input.directories.output.join("sequences"), &sequence)?;

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
