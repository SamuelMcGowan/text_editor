use super::command::EditorCommand;
use super::text_field::TextField;
use super::Editor;
use crate::buffer::Buffer;
use crate::ui::*;

pub struct EditorRoot {
    main: Box<dyn Widget<Command = EditorCommand>>,
    main_buf: Buffer,

    cmd_line: TextField,
    cmd_line_buf: Buffer,

    cmd_focused: bool,
}

impl Default for EditorRoot {
    fn default() -> Self {
        Self {
            main: Box::<Editor>::default(),
            main_buf: Buffer::new(0, 0),

            cmd_line: TextField::default(),
            cmd_line_buf: Buffer::new(0, 0),

            cmd_focused: false,
        }
    }
}

impl Widget for EditorRoot {
    type Command = EditorCommand;

    fn handle_command(
        &mut self,
        cmd: Self::Command,
        cmd_queue: &mut CmdQueue<Self::Command>,
    ) -> ControlFlow {
        match cmd {
            EditorCommand::Exit => ControlFlow::Exit,

            EditorCommand::EnterCommand => {
                self.cmd_focused = true;
                ControlFlow::Continue
            }

            EditorCommand::Escape if self.cmd_focused => {
                self.cmd_line.clear();
                self.cmd_focused = false;
                ControlFlow::Continue
            }

            cmd => {
                if self.cmd_focused {
                    self.cmd_line.handle_command(cmd, cmd_queue)
                } else {
                    self.main.handle_command(cmd, cmd_queue)
                }
            }
        }
    }

    fn update(&mut self, cmd_queue: &mut CmdQueue<Self::Command>) -> ControlFlow {
        if self.cmd_focused {
            self.cmd_line.update(cmd_queue)
        } else {
            self.main.update(cmd_queue)
        }
    }

    fn render(&mut self, buf: &mut Buffer) {
        if buf.height() < 2 {
            return;
        }

        self.main_buf
            .resize_and_clear(buf.width(), buf.height() - 1);
        self.main.render(&mut self.main_buf);
        buf.blit(0, 0, &self.main_buf, !self.cmd_focused);

        if self.cmd_focused {
            let cmd_line_y = buf.height() - 1;

            self.cmd_line_buf
                .resize_and_clear(buf.width().saturating_sub(1), 1);
            self.cmd_line.render(&mut self.cmd_line_buf);

            buf[[0, cmd_line_y]].c = ':';
            buf.blit(1, cmd_line_y, &self.cmd_line_buf, true);
        }
    }
}
