use crossterm::style::Color::{Rgb, Cyan};
use termimad::{MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Cyan); // Light Blue
    skin.italic.set_bg(Rgb {
        r: 0,
        g: 0,
        b: 0, // Pale Yellow
    });
    skin.bullet = StyledChar::from_fg_char(Cyan, '⟡'); // Light Blue
    skin.set_headers_fg(Cyan); // Light Blue
    skin.quote_mark = StyledChar::from_fg_char(Cyan, '▐'); // Light Blue
    skin.quote_mark.set_fg(Rgb {
        r: 173,
        g: 216,
        b: 230, // Light Blue
    });
    skin.inline_code.set_fg(Rgb {
        r: 173,
        g: 216,
        b: 230, // Light Blue
    });
    skin.italic.set_fg(Rgb {
        r: 255,
        g: 255,
        b: 204, // Pale Yellow
    });

    skin
}