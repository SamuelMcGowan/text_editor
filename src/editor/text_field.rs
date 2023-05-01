use ropey::Rope;

use super::event::InsertModeEvent;
use super::EditorState;
use crate::buffer::Buffer;
use crate::event::*;
use crate::ui::*;

#[derive(Default)]
pub struct TextField {
    rope: Rope,
    cursor_pos: usize,
}

impl Widget<EditorState> for TextField {
    fn handle_event(&mut self, state: &mut EditorState, event: &Event) -> Option<ControlFlow> {
        state.key_maps.insert_mode(event).and_then(|event| {
            match event {
                InsertModeEvent::InsertChar(c) => {
                    self.rope.insert_char(self.cursor_pos, c);
                    self.move_cursor(1);
                }

                InsertModeEvent::InsertString(s) => {
                    self.rope.insert(self.cursor_pos, &s);
                    // conversion could *technically* overflow
                    self.move_cursor(s.chars().count() as isize);
                }

                InsertModeEvent::Delete => {
                    let _ = self
                        .rope
                        .try_remove(self.cursor_pos..(self.cursor_pos.saturating_add(1)));
                }

                InsertModeEvent::Backspace => {
                    let new_pos = self.cursor_pos.saturating_sub(1);
                    let _ = self.rope.try_remove(new_pos..self.cursor_pos);
                    self.move_cursor(-1);
                }

                InsertModeEvent::MoveLeft => self.move_cursor(-1),
                InsertModeEvent::MoveRight => self.move_cursor(1),

                InsertModeEvent::MoveHome => self.cursor_pos = 0,
                InsertModeEvent::MoveEnd => self.cursor_pos = self.rope.len_chars(),

                InsertModeEvent::MoveUp | InsertModeEvent::MoveDown => return None,
                InsertModeEvent::Escape => return None,
            }

            Some(ControlFlow::Continue)
        })
    }

    fn update(&mut self, _state: &mut EditorState) -> ControlFlow {
        ControlFlow::Continue
    }

    fn render(&mut self, buf: &mut Buffer) {
        if buf.height() == 0 || buf.width() == 0 {
            return;
        }

        for (x, c) in self.rope.chars().enumerate().take(buf.width()) {
            buf[[x, 0]].c = c;
        }

        if self.cursor_pos < buf.width() {
            buf.set_cursor(Some((self.cursor_pos, 0)));
        }
    }
}

impl TextField {
    pub fn value(&self) -> String {
        self.rope.to_string()
    }

    pub fn clear(&mut self) {
        self.rope.remove(..);
        self.cursor_pos = 0;
    }

    fn move_cursor(&mut self, offset: isize) {
        let new_pos = self
            .cursor_pos
            .saturating_add_signed(offset)
            .min(self.rope.len_chars());
        self.cursor_pos = new_pos;
    }
}
