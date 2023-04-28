use super::Widget;
use crate::buffer::Buffer;
use crate::command::CommandWriter;
use crate::event::{Event, EventKind, KeyCode, KeyEvent, Modifiers};

pub struct VSplit {
    top: Box<dyn Widget>,
    bottom: Box<dyn Widget>,

    top_constraint: Option<usize>,
    bottom_constraint: Option<usize>,

    top_buffer: Buffer,
    bottom_buffer: Buffer,

    focus: Focus,
}

impl VSplit {
    pub fn new(
        top: impl Widget + 'static,
        bottom: impl Widget + 'static,
        top_constraint: Option<usize>,
        bottom_constraint: Option<usize>,
    ) -> Self {
        Self {
            top: Box::new(top),
            bottom: Box::new(bottom),

            top_constraint,
            bottom_constraint,

            top_buffer: Buffer::new(0, 0),
            bottom_buffer: Buffer::new(0, 0),

            focus: Focus::Top,
        }
    }
}

impl Widget for VSplit {
    fn handle_event(&mut self, event: Event, cmds: &mut CommandWriter) {
        match event.kind {
            EventKind::Key(KeyEvent {
                key_code: KeyCode::Down,
                modifiers: Modifiers::SHIFT,
            }) => {
                self.focus = Focus::Bottom;
            }

            EventKind::Key(KeyEvent {
                key_code: KeyCode::Up,
                modifiers: Modifiers::SHIFT,
            }) => {
                self.focus = Focus::Top;
            }

            _ => match self.focus {
                Focus::Top => self.top.handle_event(event, cmds),
                Focus::Bottom => self.bottom.handle_event(event, cmds),
            },
        }
    }

    fn update(&mut self, cmds: &mut CommandWriter) {
        self.top.update(cmds);
        self.bottom.update(cmds)
    }

    fn render(&mut self, buf: &mut Buffer) {
        let (top_size, bottom_size) = match (self.top_constraint, self.bottom_constraint) {
            (None, None) | (Some(_), Some(_)) => {
                let size = buf.height() / 2;
                (size, size)
            }
            (Some(top_size), None) => {
                let top_size = top_size.min(buf.height());
                (top_size, buf.height() - top_size)
            }
            (None, Some(bottom_size)) => {
                let bottom_size = bottom_size.min(buf.height());
                (buf.height() - bottom_size, bottom_size)
            }
        };

        self.top_buffer.resize_and_clear(buf.width(), top_size);
        self.bottom_buffer
            .resize_and_clear(buf.width(), bottom_size);

        self.top.render(&mut self.top_buffer);
        self.bottom.render(&mut self.bottom_buffer);

        buf.blit(0, 0, &self.top_buffer, self.focus == Focus::Top);
        buf.blit(
            0,
            top_size,
            &self.bottom_buffer,
            self.focus == Focus::Bottom,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    Top,
    Bottom,
}
