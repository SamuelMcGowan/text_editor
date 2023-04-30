use crate::event::*;
use crate::ui::Command;

pub enum EditorCommand {
    InsertChar(char),
    InsertString(String),

    Delete,
    Backspace,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    MoveHome,
    MoveEnd,

    Exit,
}

impl Command for EditorCommand {
    fn from_event(event: Event) -> Option<Self> {
        match event.kind {
            EventKind::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) => Some(Self::Exit),

            EventKind::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => Some(Self::InsertChar(c)),
                KeyCode::Return => Some(Self::InsertChar('\n')),

                KeyCode::Delete => Some(Self::Delete),
                KeyCode::Backspace => Some(Self::Backspace),

                KeyCode::Up => Some(Self::MoveUp),
                KeyCode::Down => Some(Self::MoveDown),
                KeyCode::Left => Some(Self::MoveLeft),
                KeyCode::Right => Some(Self::MoveRight),

                KeyCode::Home => Some(Self::MoveHome),
                KeyCode::End => Some(Self::MoveEnd),

                _ => None,
            },

            _ => None,
        }
    }
}
