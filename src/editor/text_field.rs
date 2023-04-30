use ropey::Rope;

use super::command::EditorCommand;
use super::EditorState;
use crate::buffer::Buffer;
use crate::ui::*;

#[derive(Default)]
pub struct TextField {
    rope: Rope,
    cursor_pos: usize,
}

impl Widget<EditorCommand, EditorState> for TextField {
    fn handle_event(
        &mut self,
        _state: &mut AppState<EditorCommand, EditorState>,
        _event: crate::event::Event,
    ) -> ControlFlow {
        ControlFlow::Continue
    }

    fn handle_command(
        &mut self,
        _state: &mut AppState<EditorCommand, EditorState>,
        command: EditorCommand,
    ) -> ControlFlow {
        // TODO: maybe create wrapper type around rope for text manipulation?
        match command {
            EditorCommand::InsertChar(c) => {
                self.rope.insert_char(self.cursor_pos, c);
                self.move_cursor(1);
            }

            EditorCommand::InsertString(s) => {
                self.rope.insert(self.cursor_pos, &s);
                // conversion could *technically* overflow
                self.move_cursor(s.chars().count() as isize);
            }

            EditorCommand::Delete => {
                let _ = self
                    .rope
                    .try_remove(self.cursor_pos..(self.cursor_pos.saturating_add(1)));
            }

            EditorCommand::Backspace => {
                let new_pos = self.cursor_pos.saturating_sub(1);
                let _ = self.rope.try_remove(new_pos..self.cursor_pos);
                self.move_cursor(-1);
            }

            EditorCommand::MoveLeft => self.move_cursor(-1),
            EditorCommand::MoveRight => self.move_cursor(1),

            EditorCommand::MoveHome => self.cursor_pos = 0,
            EditorCommand::MoveEnd => self.cursor_pos = self.rope.len_chars(),

            // TODO: move this into an editor root widget.
            EditorCommand::Exit => return ControlFlow::Exit,

            _ => {}
        }

        ControlFlow::Continue
    }

    fn update(&mut self, _state: &mut AppState<EditorCommand, EditorState>) -> ControlFlow {
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
