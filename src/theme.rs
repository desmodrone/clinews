use crossterm::style::Color::{Rgb, Yellow, White};
use termimad::{MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(White);
    skin.italic.set_bg(Rgb {
        r: 28,
        g: 28,
        b: 28,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '✧');
    skin.set_headers_fg(Yellow);
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '┃');
    skin.quote_mark.set_fg(Rgb {
        r: 0,
        g: 180,
        b: 255,
    });
    skin.inline_code.set_fg(Rgb {
        r: 0,
        g: 180,
        b: 255,
    });
    skin.italic.set_fg(Rgb {
        r: 255,
        g: 255,
        b: 255,
    });
    skin
}