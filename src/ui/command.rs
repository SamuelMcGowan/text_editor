use std::collections::VecDeque;

use crate::event::Event;

pub trait Command: Sized {
    fn from_event(event: Event) -> Option<Self>;
}

// TODO: move to input printer
pub struct SimpleCommand(pub Event);

impl Command for SimpleCommand {
    fn from_event(event: Event) -> Option<Self> {
        Some(Self(event))
    }
}

pub struct CmdQueue<C: Command>(VecDeque<C>);

impl<C: Command> Default for CmdQueue<C> {
    fn default() -> Self {
        Self(VecDeque::default())
    }
}

impl<C: Command> CmdQueue<C> {
    pub fn read(&mut self) -> Option<C> {
        self.0.pop_front()
    }

    pub fn write(&mut self, cmd: C) {
        self.0.push_back(cmd);
    }
}
