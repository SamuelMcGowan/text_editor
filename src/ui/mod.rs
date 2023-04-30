pub mod command;
pub mod widgets;

use std::io;
use std::time::{Duration, Instant};

use log::trace;

pub use self::command::*;
use crate::buffer::Buffer;
use crate::event::EventReader;
use crate::term::Term;

#[must_use]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    #[default]
    Continue,

    Exit,
}

pub trait Widget {
    type Command: Command;

    fn handle_command(
        &mut self,
        cmd: Self::Command,
        cmd_queue: &mut CmdQueue<Self::Command>,
    ) -> ControlFlow;

    fn update(&mut self, cmd_queue: &mut CmdQueue<Self::Command>) -> ControlFlow;

    fn render(&mut self, buf: &mut Buffer);
}

pub struct App<C: Command> {
    root: Box<dyn Widget<Command = C>>,
    root_buf: Buffer,

    term: Term,
    events: EventReader,

    cmd_queue: CmdQueue<C>,

    refresh_rate: Duration,
}

impl<C: Command> App<C> {
    pub fn new(
        widget: impl Widget<Command = C> + 'static,
        refresh_rate: Duration,
    ) -> io::Result<Self> {
        let term = Term::new()?;
        let term_size = term.size()?;

        Ok(Self {
            root: Box::new(widget),
            root_buf: Buffer::new(term_size.0, term_size.0),

            term,
            events: EventReader::new(),

            cmd_queue: CmdQueue::default(),

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

            if let ControlFlow::Exit = self.root.update(&mut self.cmd_queue) {
                break;
            }

            self.read_events(deadline)?;
            if let ControlFlow::Exit = self.handle_commands() {
                break;
            }

            self.render()?;

            let duration = time.duration_since(last_time);
            last_time = time;

            trace!("frame finished in {duration:?}");
        }

        Ok(())
    }

    fn read_events(&mut self, deadline: Instant) -> io::Result<()> {
        while let Some(event) = self.events.read_with_deadline(deadline)? {
            self.cmd_queue.write(C::new_event(event));
        }

        Ok(())
    }

    fn handle_commands(&mut self) -> ControlFlow {
        while let Some(cmd) = self.cmd_queue.read() {
            if let ControlFlow::Exit = self.root.handle_command(cmd, &mut self.cmd_queue) {
                return ControlFlow::Exit;
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
