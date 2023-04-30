use crate::event::*;
use crate::ui::*;

pub struct Root<C: Command>(Box<dyn Widget<Command = C>>);

impl<C: Command> Root<C> {
    pub fn new(widget: impl Widget<Command = C> + 'static) -> Self {
        Self(Box::new(widget))
    }
}

impl<C: Command> Widget for Root<C> {
    type Command = C;

    fn handle_command(
        &mut self,
        cmd: Self::Command,
        cmd_queue: &mut CmdQueue<Self::Command>,
    ) -> ControlFlow {
        if let Some(event) = cmd.event() {
            if let EventKind::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) = &event.kind
            {
                return ControlFlow::Exit;
            }
        }

        self.0.handle_command(cmd, cmd_queue)
    }

    fn update(&mut self, cmd_queue: &mut CmdQueue<Self::Command>) -> ControlFlow {
        self.0.update(cmd_queue)
    }

    fn render(&mut self, buf: &mut crate::buffer::Buffer) {
        self.0.render(buf)
    }
}
