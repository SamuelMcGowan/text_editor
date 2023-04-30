use crate::event::Event;
use crate::ui::*;

pub struct JustAnEvent(pub Event);

impl Command for JustAnEvent {
    fn from_event(event: Event) -> Option<Self> {
        Some(Self(event))
    }
}

#[derive(Default)]
pub struct InputPrinter {
    ticks: usize,
    event: Option<Event>,
}

impl Widget for InputPrinter {
    type Command = JustAnEvent;

    fn handle_command(
        &mut self,
        cmd: Self::Command,
        _cmd_queue: &mut CmdQueue<Self::Command>,
    ) -> ControlFlow {
        self.event = Some(cmd.0);
        ControlFlow::Continue
    }

    fn update(&mut self, _cmd_queue: &mut CmdQueue<Self::Command>) -> ControlFlow {
        self.ticks += 1;
        ControlFlow::Continue
    }

    fn render(&mut self, buf: &mut crate::buffer::Buffer) {
        if buf.height() == 0 {
            return;
        }

        let s = format!("Time: {}\nEvent: {:#?}", self.ticks / 60, self.event);

        for (i, line) in s.lines().enumerate().take(buf.height()) {
            for (x, c) in line.chars().enumerate().take(buf.width()) {
                buf[[x, i]].c = c;
            }
        }
    }
}
