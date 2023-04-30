use crate::event::*;
use crate::ui::Command;

pub enum NormalAction {
    CommandMode,
    InsertMode,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    MoveHome,
    MoveEnd,

    Quit,
}

pub enum InsertAction {
    Delete,
    Backspace,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    MoveHome,
    MoveEnd,

    Escape,
}

pub enum EditorCommand {
    EnterCommand,

    InsertChar(char),
    InsertString(String),
    Return,

    Delete,
    Backspace,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    MoveHome,
    MoveEnd,

    FocusUp,
    FocusDown,

    Escape,
    Exit,
}

impl Command for EditorCommand {
    fn from_event(event: Event) -> Option<Self> {
        match event.kind {
            EventKind::Key(KeyEvent {
                key_code: KeyCode::Char('C'),
                modifiers: Modifiers::CTRL,
            }) => Some(Self::EnterCommand),

            EventKind::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) => Some(Self::Exit),

            EventKind::Key(KeyEvent {
                key_code: KeyCode::Up,
                modifiers: Modifiers::SHIFT,
            }) => Some(Self::FocusUp),

            EventKind::Key(KeyEvent {
                key_code: KeyCode::Down,
                modifiers: Modifiers::SHIFT,
            }) => Some(Self::FocusDown),

            EventKind::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => Some(Self::InsertChar(c)),
                KeyCode::Return => Some(Self::Return),

                KeyCode::Delete => Some(Self::Delete),
                KeyCode::Backspace => Some(Self::Backspace),

                KeyCode::Up => Some(Self::MoveUp),
                KeyCode::Down => Some(Self::MoveDown),
                KeyCode::Left => Some(Self::MoveLeft),
                KeyCode::Right => Some(Self::MoveRight),

                KeyCode::Home => Some(Self::MoveHome),
                KeyCode::End => Some(Self::MoveEnd),

                KeyCode::Escape => Some(Self::Escape),

                _ => None,
            },

            EventKind::String(s) => Some(Self::InsertString(s)),

            _ => None,
        }
    }
}
