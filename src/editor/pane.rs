use ropey::Rope;

use super::command::EditorCommand;
use super::EditorState;
use crate::ui::*;

pub struct Pane {
    rope: Rope,
    cursor_pos: usize,
    cursor_ghost_pos: usize,
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            rope: Rope::new(),
            cursor_pos: 0,
            cursor_ghost_pos: 0,
        }
    }
}

impl Widget<EditorCommand, EditorState> for Pane {
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
        cmd: EditorCommand,
    ) -> ControlFlow {
        match cmd {
            EditorCommand::Return => {
                self.rope.insert_char(self.cursor_pos, '\n');
                self.move_cursor(1);
            }

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
            EditorCommand::MoveUp => self.move_cursor_vertical(-1),
            EditorCommand::MoveDown => self.move_cursor_vertical(1),

            EditorCommand::MoveHome => self.move_cursor_home(),
            EditorCommand::MoveEnd => self.move_cursor_end(),

            EditorCommand::Exit => return ControlFlow::Exit,

            _ => {}
        }

        ControlFlow::Continue
    }

    fn update(&mut self, _state: &mut AppState<EditorCommand, EditorState>) -> ControlFlow {
        ControlFlow::Continue
    }

    fn render(&mut self, buf: &mut crate::buffer::Buffer) {
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

impl Pane {
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

                self.cursor_pos = new_line_start.saturating_add(ghost_x.min(new_line_len));
            }
        }
    }

    fn move_cursor_home(&mut self) {
        let cursor_y = self.rope.char_to_line(self.cursor_pos);
        self.cursor_pos = self.rope.line_to_char(cursor_y);
        self.cursor_ghost_pos = self.cursor_pos;
    }

    fn move_cursor_end(&mut self) {
        let cursor_y = self.rope.char_to_line(self.cursor_pos);

        let line_start = self.rope.line_to_char(cursor_y);
        let line_len = self.line_len(cursor_y).unwrap();

        self.cursor_pos = line_start.saturating_add(line_len);
        self.cursor_ghost_pos = self.cursor_pos;
    }
}

struct Pos {
    x: usize,
    y: usize,
}