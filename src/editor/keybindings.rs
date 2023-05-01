use std::collections::HashMap;

use map_macro::hash_map;

use super::event::{InsertModeEvent, NormalModeEvent};
use crate::event::*;

pub struct Keybindings<Event> {
    map: HashMap<KeyEvent, Event>,
}

impl<Event: Copy> Keybindings<Event> {
    pub fn lookup(&self, key: &KeyEvent) -> Option<Event> {
        self.map.get(key).copied()
    }
}

pub fn insert_mode_keybindings() -> Keybindings<InsertModeEvent> {
    Keybindings {
        map: hash_map! {
            KeyEvent::key(KeyCode::Delete) => InsertModeEvent::Delete,
            KeyEvent::key(KeyCode::Backspace) => InsertModeEvent::Backspace,

            KeyEvent::key(KeyCode::Up) => InsertModeEvent::MoveUp,
            KeyEvent::key(KeyCode::Down) => InsertModeEvent::MoveDown,
            KeyEvent::key(KeyCode::Left) => InsertModeEvent::MoveLeft,
            KeyEvent::key(KeyCode::Right) => InsertModeEvent::MoveRight,

            KeyEvent::key(KeyCode::Home) => InsertModeEvent::MoveHome,
            KeyEvent::key(KeyCode::End) => InsertModeEvent::MoveEnd,

            KeyEvent::key(KeyCode::Escape) => InsertModeEvent::Escape,
        },
    }
}

pub fn normal_mode_keybindings() -> Keybindings<NormalModeEvent> {
    Keybindings {
        map: hash_map! {
            KeyEvent::key(KeyCode::Char('c')) => NormalModeEvent::CommandMode,
            KeyEvent::key(KeyCode::Char('i')) => NormalModeEvent::InsertMode,

            KeyEvent::key(KeyCode::Up) => NormalModeEvent::MoveUp,
            KeyEvent::key(KeyCode::Down) => NormalModeEvent::MoveDown,
            KeyEvent::key(KeyCode::Left) => NormalModeEvent::MoveLeft,
            KeyEvent::key(KeyCode::Right) => NormalModeEvent::MoveRight,

            KeyEvent::key(KeyCode::Home) => NormalModeEvent::MoveHome,
            KeyEvent::key(KeyCode::End) => NormalModeEvent::MoveEnd,

            KeyEvent::key(KeyCode::Char('q')) => NormalModeEvent::Quit,
        },
    }
}
