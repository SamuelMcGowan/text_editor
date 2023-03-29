use crate::event::{Event, KeyCode, KeyEvent, Modifiers};

use super::{ControlFlow, Widget};

pub struct Root(Box<dyn Widget>);

impl Root {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self(Box::new(widget))
    }
}

impl Widget for Root {
    fn handle_event(&mut self, event: Event) -> ControlFlow {
        match &event {
            Event::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) => ControlFlow::Exit,
            _ => self.0.handle_event(event),
        }
    }

    fn update(&mut self) -> ControlFlow {
        self.0.update()
    }

    fn render(&self, buf: &mut crate::buffer::Buffer) {
        self.0.render(buf)
    }
}
