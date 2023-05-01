use crate::event::Event;
use crate::ui::*;

#[derive(Default)]
pub struct InputPrinter {
    ticks: usize,
    event: Option<Event>,
}

impl Widget<()> for InputPrinter {
    fn handle_event(&mut self, _state: &mut (), event: &Event) -> Option<ControlFlow> {
        self.event = Some(event.clone());
        Some(ControlFlow::Continue)
    }

    fn update(&mut self, _state: &mut ()) -> ControlFlow {
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
