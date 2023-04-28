use std::io::{self, Write};

use self::ansi_builder::AnsiBuilder;
use crate::buffer::Buffer;

mod ansi_builder;
mod sys;

pub struct Term {
    raw_term: sys::RawTerm,
    raw_stdout: sys::RawStdout,
}

impl Term {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            raw_term: sys::RawTerm::new()?,
            raw_stdout: sys::RawStdout::default(),
        })
    }

    pub fn size(&self) -> io::Result<(usize, usize)> {
        self.raw_term.get_size()
    }

    pub fn render_buffer(&mut self, buffer: &Buffer) -> io::Result<()> {
        let mut ansi_buffer = AnsiBuilder::default();

        ansi_buffer.clear_screen();

        for y in 0..buffer.height() {
            for x in 0..buffer.width() {
                let cell = buffer[[x, y]];

                ansi_buffer.write_style(cell.style);
                ansi_buffer.write_char(cell.c);
            }

            if buffer.height() == 0 || y < buffer.height() - 1 {
                ansi_buffer.write_newline();
            }
        }

        if let Some((x, y)) = buffer.cursor() {
            ansi_buffer.set_cursor_position(x, y);
            ansi_buffer.show_cursor(true);
        }

        let ansi = ansi_buffer.finish();

        // Perform one write to stdout each frame.
        // No buffering performed, so no flushing required.
        write!(self.raw_stdout, "{ansi}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Term;

    #[test]
    fn get_term_size() {
        let term = Term::new().unwrap();
        let _ = term.size().unwrap();
    }
}
