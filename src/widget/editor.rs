use ropey::Rope;

use super::{ControlFlow, Widget};
use crate::buffer::Buffer;
use crate::event::{Event, EventKind, KeyCode, KeyEvent};

pub struct Editor {
    rope: Rope,
    cursor_pos: usize,
    cursor_ghost_pos: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            rope: Rope::new(),
            cursor_pos: 0,
            cursor_ghost_pos: 0,
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
                    self.move_cursor(1);
                }
                KeyCode::Return => {
                    self.rope.insert_char(self.cursor_pos, '\n');
                    self.move_cursor(1);
                }

                KeyCode::Delete => {
                    let _ = self
                        .rope
                        .try_remove(self.cursor_pos..(self.cursor_pos.saturating_add(1)));
                }
                KeyCode::Backspace => {
                    let new_pos = self.cursor_pos.saturating_sub(1);
                    let _ = self.rope.try_remove(new_pos..self.cursor_pos);
                    self.move_cursor(-1);
                }

                KeyCode::Left => self.move_cursor(-1),
                KeyCode::Right => self.move_cursor(1),

                KeyCode::Up => self.move_cursor_vertical(-1),
                KeyCode::Down => self.move_cursor_vertical(1),

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

        let cursor = self.pos_to_xy(self.cursor_pos).unwrap();
        if cursor.x < buf.width() && cursor.y < buf.height() {
            buf.set_cursor(Some((cursor.x, cursor.y)));
        }
    }
}

impl Editor {
    fn pos_to_xy(&self, pos: usize) -> Option<Pos> {
        let y = self.rope.char_to_line(pos);
        let line_start = self.rope.line_to_char(y);
        let x = pos - line_start;

        Some(Pos { x, y })
    }

    fn line_len(&self, line_y: usize) -> Option<usize> {
        let line_len = self.rope.get_line(line_y)?.len_chars();

        // There is always at least one line.
        if line_y == self.rope.len_lines() - 1 {
            Some(line_len)
        } else {
            Some(line_len.saturating_sub(1))
        }
    }

    fn move_cursor(&mut self, offset: isize) {
        let new_pos = self
            .cursor_pos
            .saturating_add_signed(offset)
            .min(self.rope.len_chars());

        self.cursor_pos = new_pos;
        self.cursor_ghost_pos = new_pos;
    }

    fn move_cursor_vertical(&mut self, offset: isize) {
        let current_y = self.rope.char_to_line(self.cursor_pos);
        match current_y.checked_add_signed(offset) {
            None => {
                self.cursor_pos = 0;
                self.cursor_ghost_pos = 0;
            }
            Some(new_y) if new_y >= self.rope.len_lines() => {
                self.cursor_pos = self.rope.len_chars();
                self.cursor_ghost_pos = self.cursor_pos;
            }
            Some(new_y) => {
                let ghost_x = self.pos_to_xy(self.cursor_ghost_pos).unwrap().x;

                let new_line_start = self.rope.line_to_char(new_y);
                let new_line_len = self.line_len(new_y).unwrap();

                self.cursor_pos = new_line_start + ghost_x.min(new_line_len);
            }
        }
    }
}

struct Pos {
    x: usize,
    y: usize,
}
