use super::command::EditorCommand;
use super::EditorState;
use crate::buffer::Buffer;
use crate::ui::*;

pub struct VSplit {
    top: Box<dyn Widget<EditorCommand, EditorState>>,
    bottom: Box<dyn Widget<EditorCommand, EditorState>>,

    top_constraint: Option<usize>,
    bottom_constraint: Option<usize>,

    top_buffer: Buffer,
    bottom_buffer: Buffer,

    focus: Focus,
}

impl VSplit {
    pub fn new(
        top: impl Widget<EditorCommand, EditorState> + 'static,
        bottom: impl Widget<EditorCommand, EditorState> + 'static,
        top_constraint: Option<usize>,
        bottom_constraint: Option<usize>,
    ) -> Self {
        Self {
            top: Box::new(top),
            bottom: Box::new(bottom),

            top_constraint,
            bottom_constraint,

            top_buffer: Buffer::new(0, 0),
            bottom_buffer: Buffer::new(0, 0),

            focus: Focus::Top,
        }
    }
}

impl Widget<EditorCommand, EditorState> for VSplit {
    fn handle_event(
        &mut self,
        _state: &mut AppState<EditorCommand, EditorState>,
        _event: crate::event::Event,
    ) -> ControlFlow {
        ControlFlow::Continue
    }

    fn handle_command(
        &mut self,
        state: &mut AppState<EditorCommand, EditorState>,
        cmd: EditorCommand,
    ) -> ControlFlow {
        match cmd {
            EditorCommand::FocusUp => {
                self.focus = Focus::Top;
                ControlFlow::Continue
            }

            EditorCommand::FocusDown => {
                self.focus = Focus::Bottom;
                ControlFlow::Continue
            }

            cmd => match self.focus {
                Focus::Top => self.top.handle_command(state, cmd),
                Focus::Bottom => self.bottom.handle_command(state, cmd),
            },
        }
    }

    fn update(&mut self, state: &mut AppState<EditorCommand, EditorState>) -> ControlFlow {
        if let ControlFlow::Exit = self.top.update(state) {
            return ControlFlow::Exit;
        }
        self.bottom.update(state)
    }

    fn render(&mut self, buf: &mut Buffer) {
        let (top_size, bottom_size) = match (self.top_constraint, self.bottom_constraint) {
            (None, None) | (Some(_), Some(_)) => {
                let size = buf.height() / 2;
                (size, size)
            }
            (Some(top_size), None) => {
                let top_size = top_size.min(buf.height());
                (top_size, buf.height() - top_size)
            }
            (None, Some(bottom_size)) => {
                let bottom_size = bottom_size.min(buf.height());
                (buf.height() - bottom_size, bottom_size)
            }
        };

        self.top_buffer.resize_and_clear(buf.width(), top_size);
        self.bottom_buffer
            .resize_and_clear(buf.width(), bottom_size);

        self.top.render(&mut self.top_buffer);
        self.bottom.render(&mut self.bottom_buffer);

        buf.blit(0, 0, &self.top_buffer, self.focus == Focus::Top);
        buf.blit(
            0,
            top_size,
            &self.bottom_buffer,
            self.focus == Focus::Bottom,
        );
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    #[default]
    Top,

    Bottom,
}