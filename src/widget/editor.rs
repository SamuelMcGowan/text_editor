use ropey::Rope;

use super::{ControlFlow, Widget};
use crate::buffer::Buffer;
use crate::event::{Event, EventKind, KeyCode, KeyEvent};

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

                KeyCode::Delete => {
                    let _ = self
                        .rope
                        .try_remove(self.cursor_pos..(self.cursor_pos.saturating_add(1)));
                }
                KeyCode::Backspace => {
                    let new_pos = self.cursor_pos.saturating_sub(1);
                    let _ = self.rope.try_remove(new_pos..self.cursor_pos);
                    self.cursor_pos = new_pos;
                }

                KeyCode::Left => {
                    self.cursor_pos = self.cursor_pos.saturating_sub(1);
                }
                KeyCode::Right => {
                    self.cursor_pos = self.cursor_pos.saturating_add(1).min(self.rope.len_chars());
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

        let (cursor_x, cursor_y) = self.cursor_xy();
        if cursor_x < buf.width() && cursor_y < buf.height() {
            buf.set_cursor(Some((cursor_x, cursor_y)));
        }
    }
}

impl Editor {
    fn cursor_xy(&self) -> (usize, usize) {
        let cursor_y = self.rope.char_to_line(self.cursor_pos);
        let cursor_x = self.cursor_pos - self.rope.line_to_char(cursor_y);

        (cursor_x, cursor_y)
    }
}
