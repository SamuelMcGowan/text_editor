pub mod widgets;

use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

use log::trace;

use crate::buffer::Buffer;
use crate::event::{Event, EventReader};
use crate::term::Term;

#[must_use]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    #[default]
    Continue,

    Exit,
}

pub trait Widget<Command, GlobalState> {
    fn handle_event(
        &mut self,
        state: &mut AppState<Command, GlobalState>,
        event: Event,
    ) -> ControlFlow;

    fn handle_command(
        &mut self,
        state: &mut AppState<Command, GlobalState>,
        cmd: Command,
    ) -> ControlFlow;

    fn update(&mut self, state: &mut AppState<Command, GlobalState>) -> ControlFlow;

    fn render(&mut self, buf: &mut Buffer);
}

pub struct App<Command, GlobalState> {
    root: Box<dyn Widget<Command, GlobalState>>,
    root_buf: Buffer,

    term: Term,
    events: EventReader,

    state: AppState<Command, GlobalState>,

    refresh_rate: Duration,
}

impl<Command, GlobalState> App<Command, GlobalState> {
    pub fn new(
        state: GlobalState,
        widget: impl Widget<Command, GlobalState> + 'static,
        refresh_rate: Duration,
    ) -> io::Result<Self> {
        let term = Term::new()?;
        let term_size = term.size()?;

        Ok(Self {
            root: Box::new(widget),
            root_buf: Buffer::new(term_size.0, term_size.0),

            term,
            events: EventReader::new(),

            state: AppState {
                state,
                commands: VecDeque::new(),
            },

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

            if let ControlFlow::Exit = self.root.update(&mut self.state) {
                break;
            }

            if let ControlFlow::Exit = self.handle_events(deadline)? {
                break;
            }

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

    fn handle_events(&mut self, deadline: Instant) -> io::Result<ControlFlow> {
        while let Some(event) = self.events.read_with_deadline(deadline)? {
            if let ControlFlow::Exit = self.root.handle_event(&mut self.state, event) {
                return Ok(ControlFlow::Exit);
            }
        }

        Ok(ControlFlow::Continue)
    }

    fn handle_commands(&mut self) -> ControlFlow {
        while let Some(cmd) = self.state.read_command() {
            if let ControlFlow::Exit = self.root.handle_command(&mut self.state, cmd) {
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

pub struct AppState<Command, GlobalState> {
    state: GlobalState,
    commands: VecDeque<Command>,
}

impl<Command, GlobalState> AppState<Command, GlobalState> {
    pub fn write_command(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    fn read_command(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }
}
