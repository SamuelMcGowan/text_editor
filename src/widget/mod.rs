pub mod editor;
pub mod root;
pub mod vsplit;

use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

use log::trace;

use crate::buffer::Buffer;
use crate::command::{Command, CommandWriter};
use crate::event::{Event, EventReader};
use crate::term::Term;

#[must_use]
pub enum ControlFlow {
    Continue,
    Exit,
}

pub trait Widget {
    fn handle_event(&mut self, event: Event, cmds: &mut CommandWriter);

    #[allow(unused_variables)]
    fn update(&mut self, cmds: &mut CommandWriter) {}

    fn render(&mut self, buf: &mut Buffer);
}

pub struct App {
    root: Box<dyn Widget>,
    root_buf: Buffer,

    term: Term,
    events: EventReader,

    command_queue: VecDeque<Command>,

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

            command_queue: VecDeque::new(),

            refresh_rate,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut last_time = Instant::now();

        loop {
            let time = Instant::now();
            let deadline = time
                .checked_add(self.refresh_rate)
                .expect("deadline overflowed");

            if let ControlFlow::Exit = self.update() {
                break;
            }

            if let ControlFlow::Exit = self.handle_events(deadline)? {
                break;
            }

            self.render()?;

            let duration = time.duration_since(last_time);
            last_time = time;

            trace!("frame finished in {duration:?}");
        }

        Ok(())
    }

    fn update(&mut self) -> ControlFlow {
        let mut cmds = CommandWriter::new(&mut self.command_queue);
        self.root.update(&mut cmds);
        self.process_commands()
    }

    fn handle_events(&mut self, deadline: Instant) -> io::Result<ControlFlow> {
        let mut cmds = CommandWriter::new(&mut self.command_queue);

        // Keep reading (and handling) events until the deadline is up.
        while let Some(event) = self.events.read_with_deadline(deadline)? {
            self.root.handle_event(event, &mut cmds);
        }

        Ok(self.process_commands())
    }

    fn process_commands(&mut self) -> ControlFlow {
        #[allow(clippy::never_loop)]
        for command in self.command_queue.drain(..) {
            match command {
                Command::Exit => return ControlFlow::Exit,
            }
        }

        ControlFlow::Continue
    }

    fn render(&mut self) -> io::Result<()> {
        let term_size = self.term.size()?;
        self.root_buf.resize_and_clear(term_size.0, term_size.1);

        self.root.render(&mut self.root_buf);
        self.term.render_buffer(&self.root_buf)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct InputPrinter {
    ticks: usize,
    event: Option<Event>,
}

impl Widget for InputPrinter {
    fn handle_event(&mut self, event: Event, _cmds: &mut CommandWriter) {
        self.event = Some(event);
    }

    fn update(&mut self, _cmds: &mut CommandWriter) {
        self.ticks += 1;
    }

    fn render(&mut self, buf: &mut Buffer) {
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
