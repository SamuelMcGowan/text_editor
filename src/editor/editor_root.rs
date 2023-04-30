use super::command::EditorCommand;
use super::pane::Pane;
use super::text_field::TextField;
use super::EditorState;
use crate::buffer::Buffer;
use crate::event::Event;
use crate::ui::*;

pub struct EditorRoot {
    main: Box<dyn Widget<EditorCommand, EditorState>>,
    main_buf: Buffer,

    cmd_line: TextField,
    cmd_line_buf: Buffer,

    cmd_focused: bool,
}

impl Default for EditorRoot {
    fn default() -> Self {
        Self {
            main: Box::<Pane>::default(),
            main_buf: Buffer::new(0, 0),

            cmd_line: TextField::default(),
            cmd_line_buf: Buffer::new(0, 0),

            cmd_focused: false,
        }
    }
}

impl Widget<Event, EditorState> for EditorRoot {
    fn handle_event(&mut self, state: &mut EditorState, event: crate::event::Event) -> ControlFlow {
        let Some(event) = EditorCommand::from_event(event) else {
            return ControlFlow::Continue;
        };

        match event {
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

            event => {
                if self.cmd_focused {
                    self.cmd_line.handle_event(state, event)
                } else {
                    self.main.handle_event(state, event)
                }
            }
        }
    }

    fn update(&mut self, state: &mut EditorState) -> ControlFlow {
        if self.cmd_focused {
            self.cmd_line.update(state)
        } else {
            self.main.update(state)
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
