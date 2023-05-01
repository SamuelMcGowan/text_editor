use super::event::VSplitEvent;
use super::EditorState;
use crate::buffer::Buffer;
use crate::event::*;
use crate::ui::*;

pub struct VSplit {
    top: Box<dyn Widget<EditorState>>,
    bottom: Box<dyn Widget<EditorState>>,

    top_constraint: Option<usize>,
    bottom_constraint: Option<usize>,

    top_buffer: Buffer,
    bottom_buffer: Buffer,

    focus: Focus,
}

impl VSplit {
    pub fn new(
        top: impl Widget<EditorState> + 'static,
        bottom: impl Widget<EditorState> + 'static,
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

impl Widget<EditorState> for VSplit {
    fn handle_event(
        &mut self,
        state: &mut EditorState,
        event: Event,
    ) -> Result<ControlFlow, Event> {
        match state.key_maps.vsplit(event) {
            Ok(event) => {
                match event {
                    VSplitEvent::FocusUp => {
                        self.focus = Focus::Top;
                    }
                    VSplitEvent::FocusDown => {
                        self.focus = Focus::Bottom;
                    }
                }
                Ok(ControlFlow::Continue)
            }
            Err(event) => match self.focus {
                Focus::Top => self.top.handle_event(state, event),
                Focus::Bottom => self.bottom.handle_event(state, event),
            },
        }
    }

    fn update(&mut self, state: &mut EditorState) -> ControlFlow {
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
