use luthien_plugin::{Palette, Theme};
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

        const INDEXES: Palette<&[Sequence]> = {
            use Sequence::*;
            Palette {
                black: &[
                    Regular(0),
                    Regular(8),
                    Special(11),
                    Special(19),
                    Special(232),
                ],
                red: &[Regular(1), Regular(9)],
                green: &[Regular(2), Regular(10)],
                yellow: &[Regular(3), Regular(11)],
                blue: &[Regular(4), Regular(12)],
                purple: &[Regular(5), Regular(13)],
                cyan: &[Regular(6), Regular(14)],
                white: &[
                    Regular(7),
                    Regular(15),
                    Special(10),
                    Special(13),
                    Special(17),
                    Special(256),
                ],
            }
        };

        self.colors.clone().zip(INDEXES).map(|(col, codes)| {
            ret.extend(codes.iter().flat_map(|code| {
                code.sequence(&format!(
                    "#{:02x}{:02x}{:02x}",
                    (col.red * 255.0) as u8,
                    (col.green * 255.0) as u8,
                    (col.blue * 255.0) as u8
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
