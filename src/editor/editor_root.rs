use super::event::{CommandModeEvent, EditorRootEvent};
use super::pane::Pane;
use super::text_field::TextField;
use super::EditorState;
use crate::buffer::Buffer;
use crate::event::*;
use crate::ui::*;

pub struct EditorRoot {
    main: Box<dyn Widget<EditorState>>,
    main_buf: Buffer,

    cmd_line: TextField,
    cmd_line_buf: Buffer,

    command_mode: bool,
}

impl Default for EditorRoot {
    fn default() -> Self {
        Self {
            main: Box::<Pane>::default(),
            main_buf: Buffer::new(0, 0),

            cmd_line: TextField::default(),
            cmd_line_buf: Buffer::new(0, 0),

            command_mode: false,
        }
    }
}

impl Widget<EditorState> for EditorRoot {
    fn handle_event(&mut self, state: &mut EditorState, event: Event) -> ControlFlow {
        if self.command_mode {
            match state.key_maps.command_mode(event) {
                Ok(CommandModeEvent::Escape) => {
                    self.cmd_line.clear();
                    self.command_mode = false;
                    ControlFlow::Continue
                }
                Err(event) => self.cmd_line.handle_event(state, event),
            }
        } else {
            match state.key_maps.editor_root(event) {
                Ok(event) => match event {
                    EditorRootEvent::CommandMode => {
                        self.command_mode = true;
                        ControlFlow::Continue
                    }
                    EditorRootEvent::Quit => ControlFlow::Exit,
                },
                Err(event) => self.main.handle_event(state, event),
            }
        }
    }

    fn update(&mut self, state: &mut EditorState) -> ControlFlow {
        if self.command_mode {
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
        buf.blit(0, 0, &self.main_buf, !self.command_mode);

        if self.command_mode {
            let cmd_line_y = buf.height() - 1;

            self.cmd_line_buf
                .resize_and_clear(buf.width().saturating_sub(1), 1);
            self.cmd_line.render(&mut self.cmd_line_buf);

            buf[[0, cmd_line_y]].c = ':';
            buf.blit(1, cmd_line_y, &self.cmd_line_buf, true);
        }
    }
}
