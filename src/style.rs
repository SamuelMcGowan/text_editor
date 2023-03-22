use std::fmt::Write;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Red = 1,
    Greem = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,

    #[default]
    Default = 9,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    #[default]
    Normal,
    Bold,
    Dim,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Style {
    fg: Color,
    bg: Color,

    weight: Weight,
    underline: bool,
}

#[derive(Default, Debug, Clone)]
pub struct AnsiBuilder {
    s: String,
    style: Style,
}

impl AnsiBuilder {
    pub fn write_str(&mut self, s: &str) {
        // remove the control characters without having to
        // push each character separately
        for part in s.split(|c: char| c.is_control()) {
            self.s.push_str(part);
        }
    }

    pub fn write_raw(&mut self, s: &str) {
        self.s.push_str(s);
    }

    pub fn write_style(&mut self, style: Style) {
        macro_rules! sgr {
            ($($arg:tt)+) => {{
                write!(self.s, "\x1b[{}m", format_args!($($arg)*)).unwrap();
            }};
        }

        if style.fg != self.style.fg {
            sgr!("3{}", style.fg as u8);
        }

        if style.bg != self.style.bg {
            sgr!("4{}", style.bg as u8);
        }

        if style.weight != self.style.weight {
            match style.weight {
                Weight::Normal => sgr!("22"),
                Weight::Bold => sgr!("1"),
                Weight::Dim => sgr!("2"),
            }
        }

        if style.underline != self.style.underline {
            match style.underline {
                true => sgr!("4"),
                false => sgr!("24"),
            }
        }

        self.style = style;
    }

    pub fn finish(mut self) -> String {
        self.write_style(Style::default());
        self.s
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::style::{Color, Style, Weight};
//     use crate::term::RawTermGuard;

//     use super::AnsiBuilder;

//     #[test]
//     fn my_test() {
//         let mut ansi = AnsiBuilder::default();
//         ansi.write_raw("hello\r\n");

//         ansi.write_style(Style {
//             fg: Color::Magenta,
//             ..Default::default()
//         });
//         ansi.write_raw("world\r\n");

//         ansi.write_style(Style {
//             fg: Color::Blue,
//             weight: Weight::Bold,
//             ..Default::default()
//         });
//         ansi.write_raw("boo!\r\n");

//         let _term = RawTermGuard::new(libc::STDIN_FILENO);
//         print!("{}", ansi.finish());
//     }
// }
