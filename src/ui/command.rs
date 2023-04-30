use std::collections::VecDeque;

use crate::event::Event;

pub trait Command {
    fn new_event(event: Event) -> Self;
    fn event(&self) -> Option<&Event>;
}

pub struct SimpleCommand(pub Event);

impl Command for SimpleCommand {
    fn new_event(event: Event) -> Self {
        Self(event)
    }

    fn event(&self) -> Option<&Event> {
        Some(&self.0)
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
