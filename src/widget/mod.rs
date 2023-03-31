pub mod editor;
pub mod root;

use log::trace;

use std::io;
use std::time::{Duration, Instant};

use crate::buffer::Buffer;
use crate::event::{Event, EventReader};
use crate::term::Term;

pub enum ControlFlow {
    Continue,
    Exit,
}

pub trait Widget {
    fn handle_event(&mut self, event: Event) -> ControlFlow;

    fn update(&mut self) -> ControlFlow {
        ControlFlow::Continue
    }

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
        let term = Term::new()?;
        let term_size = term.size()?;

        Ok(Self {
            root: Box::new(widget),
            root_buf: Buffer::new(term_size.0, term_size.0),

            term,
            events: EventReader::new(),

            refresh_rate,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut last_time = Instant::now();
        while let ControlFlow::Continue = self.tick()? {
            let now = Instant::now();
            let duration = now.duration_since(last_time);
            last_time = now;

            trace!("frame finished in {duration:?}");
        }
        Ok(())
    }

    /// Handle events for however long a frame is, then update
    /// and render the root widget.
    fn tick(&mut self) -> io::Result<ControlFlow> {
        let time = Instant::now();
        let deadline = time
            .checked_add(self.refresh_rate)
            .expect("deadline overflowed");

        if let ControlFlow::Exit = self.root.update() {
            return Ok(ControlFlow::Exit);
        }

        // Keep reading (and handling) events until the deadline is up.
        while let Some(event) = self.events.read_with_deadline(deadline)? {
            if let ControlFlow::Exit = self.root.handle_event(event) {
                return Ok(ControlFlow::Exit);
            }
        }

        let term_size = self.term.size()?;
        self.root_buf.resize_and_clear(term_size.0, term_size.1);

        self.root.render(&mut self.root_buf);
        self.term.render_buffer(&self.root_buf)?;

        Ok(ControlFlow::Continue)
    }
}

#[derive(Default)]
pub struct InputPrinter {
    ticks: usize,
    event: Option<Event>,
}

impl Widget for InputPrinter {
    fn handle_event(&mut self, event: Event) -> ControlFlow {
        self.event = Some(event);
        ControlFlow::Continue
    }

    fn update(&mut self) -> ControlFlow {
        self.ticks += 1;
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

        let s = format!("{}    {s}", self.ticks / 60);

        for (x, c) in s.chars().enumerate().take(buf.width()) {
            buf[[x, 0]].c = c;
        }
    }
}
