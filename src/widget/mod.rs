use std::io;
use std::time::Duration;

use crate::buffer::{Buffer, Cell};
use crate::event::{Event, EventReader};
use crate::term::Term;

pub enum ControlFlow {
    Continue,
    Exit,
}

pub trait Widget {
    fn update(&mut self) -> ControlFlow {
        ControlFlow::Continue
    }

    fn handle_event(&mut self, event: Event) -> ControlFlow;

    fn render(&self, buf: &mut Buffer);
}

pub struct App {
    root: Box<dyn Widget>,
    root_buf: Buffer,

    term: Term,
    events: EventReader,

    refresh_rate: Duration,
}

impl App {
    pub fn new(widget: impl Widget + 'static, refresh_rate: Duration) -> io::Result<Self> {
        Ok(Self {
            root: Box::new(widget),
            root_buf: Buffer::empty(),

            term: Term::new()?,
            events: EventReader::new(),

            refresh_rate,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        while let ControlFlow::Continue = self.update()? {
            std::thread::sleep(self.refresh_rate);
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<ControlFlow> {
        if let ControlFlow::Exit = self.root.update() {
            return Ok(ControlFlow::Exit);
        }

        while let Some(event) = self.events.read_event()? {
            if let ControlFlow::Exit = self.root.handle_event(event) {
                return Ok(ControlFlow::Exit);
            }
        }

        // FIXME: this is pretty inefficient
        let term_size = self.term.size()?;
        self.root_buf = Buffer::filled(term_size.0, term_size.1, Cell::default());

        self.root.render(&mut self.root_buf);
        self.term.render_buffer(&self.root_buf)?;

        Ok(ControlFlow::Continue)
    }
}

#[derive(Default)]
pub struct InputPrinter {
    event: Option<Event>,
}

impl Widget for InputPrinter {
    fn handle_event(&mut self, event: Event) -> ControlFlow {
        self.event = Some(event);
        ControlFlow::Continue
    }

    fn render(&self, buf: &mut Buffer) {
        if buf.height() == 0 {
            return;
        }

        let s = match &self.event {
            Some(event) => format!("{event:?}"),
            None => "--".to_string(),
        };

        for (x, c) in s.chars().enumerate().take(buf.width()) {
            buf[[x, 0]].c = c;
        }
    }
}
