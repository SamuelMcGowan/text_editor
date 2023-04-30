use std::collections::HashMap;

use map_macro::hash_map;

use super::command::{InsertAction, NormalAction};
use crate::event::*;

pub struct Keybindings<Event> {
    map: HashMap<KeyEvent, Event>,
}

impl<Event: Copy> Keybindings<Event> {
    pub fn lookup(&self, key: &KeyEvent) -> Option<Event> {
        self.map.get(key).copied()
    }
}

pub fn insert_mode_keybindings() -> Keybindings<InsertAction> {
    Keybindings {
        map: hash_map! {
            KeyEvent::key(KeyCode::Delete) => InsertAction::Delete,
            KeyEvent::key(KeyCode::Backspace) => InsertAction::Backspace,

            KeyEvent::key(KeyCode::Up) => InsertAction::MoveUp,
            KeyEvent::key(KeyCode::Down) => InsertAction::MoveDown,
            KeyEvent::key(KeyCode::Left) => InsertAction::MoveLeft,
            KeyEvent::key(KeyCode::Right) => InsertAction::MoveRight,

            KeyEvent::key(KeyCode::Home) => InsertAction::MoveHome,
            KeyEvent::key(KeyCode::End) => InsertAction::MoveEnd,

            KeyEvent::key(KeyCode::Escape) => InsertAction::Escape,
        },
    }
}

pub fn normal_mode_keybindings() -> Keybindings<NormalAction> {
    Keybindings {
        map: hash_map! {
            KeyEvent::key(KeyCode::Char('c')) => NormalAction::CommandMode,
            KeyEvent::key(KeyCode::Char('i')) => NormalAction::InsertMode,

            KeyEvent::key(KeyCode::Up) => NormalAction::MoveUp,
            KeyEvent::key(KeyCode::Down) => NormalAction::MoveDown,
            KeyEvent::key(KeyCode::Left) => NormalAction::MoveLeft,
            KeyEvent::key(KeyCode::Right) => NormalAction::MoveRight,

            KeyEvent::key(KeyCode::Home) => NormalAction::MoveHome,
            KeyEvent::key(KeyCode::End) => NormalAction::MoveEnd,

            KeyEvent::key(KeyCode::Char('q')) => NormalAction::Quit,
        },
    }
}
