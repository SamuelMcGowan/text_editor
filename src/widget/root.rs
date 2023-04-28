use super::Widget;
use crate::command::{Command, CommandWriter};
use crate::event::{Event, EventKind, KeyCode, KeyEvent, Modifiers};

pub struct Root(Box<dyn Widget>);

impl Root {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self(Box::new(widget))
    }
}

impl Widget for Root {
    fn handle_event(&mut self, event: Event, cmds: &mut CommandWriter) {
        match &event.kind {
            EventKind::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) => cmds.write(Command::Exit),

            _ => self.0.handle_event(event, cmds),
        }
    }

    fn update(&mut self, cmds: &mut CommandWriter) {
        self.0.update(cmds);
    }

    fn render(&mut self, buf: &mut crate::buffer::Buffer) {
        self.0.render(buf)
    }
}
