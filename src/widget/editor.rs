use crate::buffer::Buffer;
use crate::event::{Event, KeyCode, KeyEvent};

use super::{ControlFlow, Widget};

pub struct Editor {
    lines: Vec<String>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
}

impl Widget for Editor {
    fn handle_event(&mut self, event: Event) -> ControlFlow {
        match event {
            Event::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => {
                    self.lines.last_mut().unwrap().push(c);
                }
                KeyCode::Return => {
                    self.lines.push(String::new());
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
        for (y, line) in (0..buf.height()).zip(&self.lines) {
            for (x, c) in (0..buf.width()).zip(line.chars()) {
                buf[[x, y]].c = c;
            }
        }

        let cursor_x = self.lines.last().unwrap().len();
        let cursor_y = self.lines.len() - 1;

        if cursor_x < buf.width() && cursor_y < buf.height() {
            buf.set_cursor(Some((cursor_x, cursor_y)));
        }
    }
}
