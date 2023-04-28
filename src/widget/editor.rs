use ropey::Rope;

use crate::buffer::Buffer;
use crate::event::{Event, KeyCode, KeyEvent, EventKind};

use super::{ControlFlow, Widget};

pub struct Editor {
    rope: Rope,
    cursor_pos: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            rope: Rope::new(),
            cursor_pos: 0,
        }
    }
}

impl Widget for Editor {
    fn handle_event(&mut self, event: Event) -> ControlFlow {
        match event.kind {
            EventKind::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => {
                    self.rope.insert_char(self.cursor_pos, c);
                    self.cursor_pos += 1;
                }
                KeyCode::Return => {
                    self.rope.insert_char(self.cursor_pos, '\n');
                    self.cursor_pos += 1;
                }
                _ => {}
            },
            _ => {}
        }

        ControlFlow::Continue
    }

    fn update(&mut self) -> ControlFlow {
        ControlFlow::Continue
    }

    fn render(&self, buf: &mut Buffer) {
        for (y, line) in (0..buf.height()).zip(self.rope.lines()) {
            for (x, c) in (0..buf.width()).zip(line.chars()) {
                buf[[x, y]].c = c;
            }
        }

        let (cursor_x, cursor_y) = if let Some(cursor_y) = self
            .rope
            .get_line(self.cursor_pos)
            .map(|line| line.len_chars())
        {
            let cursor_x = self.cursor_pos - cursor_y;
            (cursor_x, cursor_y)
        } else {
            let cursor_x = self
                .rope
                .lines()
                .last()
                .map(|line| line.len_chars())
                .unwrap_or_default();

            let cursor_y = self.rope.len_lines().saturating_sub(1);

            (cursor_x, cursor_y)
        };

        if cursor_x < buf.width() && cursor_y < buf.height() {
            buf.set_cursor(Some((cursor_x, cursor_y)));
        }
    }
}
