use std::collections::VecDeque;

pub enum Command {
    Exit,
}

pub struct CommandWriter<'a> {
    queue: &'a mut VecDeque<Command>,
}

impl<'a> CommandWriter<'a> {
    pub fn new(queue: &'a mut VecDeque<Command>) -> Self {
        Self { queue }
    }

    pub fn write(&mut self, cmd: Command) {
        self.queue.push_back(cmd);
    }
}
