use std::io::Write;

use crate::buffer::Buffer;

use self::ansi_builder::AnsiBuilder;
use self::sys::RawTermGuard;

mod ansi_builder;
mod sys;

pub struct Term {
    _raw_term_guard: RawTermGuard,
}

impl Term {
    pub fn render_buffer(&mut self, buffer: &Buffer) {
        let mut ansi_buffer = AnsiBuilder::default();

        ansi_buffer.clear_screen();

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let cell = buffer[[x, y]];

                ansi_buffer.write_style(cell.style);
                ansi_buffer.write_char(cell.c);
            }

            ansi_buffer.write_newline();
        }

        if let Some((x, y)) = buffer.cursor() {
            ansi_buffer.set_cursor_position(x, y);
        }

        let ansi = ansi_buffer.finish();

        print!("{ansi}");
        std::io::stdout().flush().expect("couldn't flush stdout");
    }
}
