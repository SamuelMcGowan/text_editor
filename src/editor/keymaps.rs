use std::collections::HashMap;

use super::event::{
    CommandModeEvent, EditorRootEvent, InsertModeEvent, NormalModeEvent, VSplitEvent,
};
use crate::event::*;

#[derive(Debug)]
struct KeyMap<E> {
    map: HashMap<KeyEvent, E>,
}

impl<E: Clone> KeyMap<E> {
    fn get(&self, event: &Event) -> Option<E> {
        match &event.kind {
            EventKind::Key(key_event) => self.map.get(key_event).cloned(),
            _ => None,
        }
    }
}

macro_rules! key_map {
    (
        $(
            $([$($modifier:ident)+])?
            $key:ident $(( $($arg:tt)* ))?
            => $e:expr
        ),*
        $(,)?
    ) => {{
        #[allow(unused_mut)]
        let mut map = HashMap::new();

        $(
            let key = KeyEvent {
                key_code: KeyCode::$key $(( $($arg)* ))?,
                modifiers: Modifiers::empty() $($( | Modifiers::$modifier )*)?,
            };

            map.insert(key, $e);
        )*

        KeyMap { map }
    }};
}

pub struct KeyMaps {
    normal_mode: KeyMap<NormalModeEvent>,
    insert_mode: KeyMap<InsertModeEvent<'static>>,
    command_mode: KeyMap<CommandModeEvent>,

    editor_root: KeyMap<EditorRootEvent>,
    vsplit: KeyMap<VSplitEvent>,
}

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            normal_mode: key_map! {
                Char('i') => NormalModeEvent::InsertMode,

                Up => NormalModeEvent::MoveUp,
                Down => NormalModeEvent::MoveDown,
                Left => NormalModeEvent::MoveLeft,
                Right => NormalModeEvent::MoveRight,

                Home => NormalModeEvent::MoveHome,
                End => NormalModeEvent::MoveEnd,
            },

            insert_mode: key_map! {
                Delete => InsertModeEvent::Delete,
                Backspace => InsertModeEvent::Backspace,

                Up => InsertModeEvent::MoveUp,
                Down => InsertModeEvent::MoveDown,
                Left => InsertModeEvent::MoveLeft,
                Right => InsertModeEvent::MoveRight,

                Home => InsertModeEvent::MoveHome,
                End => InsertModeEvent::MoveEnd,

                Escape => InsertModeEvent::Escape,
            },

            command_mode: key_map! {
                Escape => CommandModeEvent::Escape,
            },

            editor_root: key_map! {
                Char(':') => EditorRootEvent::CommandMode,
                Char('q') => EditorRootEvent::Quit,
            },

            vsplit: key_map! {
                [CTRL] Up => VSplitEvent::FocusUp,
                [CTRL] Down => VSplitEvent::FocusDown,
            },
        }
    }
}

impl KeyMaps {
    pub fn normal_mode(&self, event: &Event) -> Option<NormalModeEvent> {
        self.normal_mode.get(event)
    }

    pub fn insert_mode<'a>(&self, event: &'a Event) -> Option<InsertModeEvent<'a>> {
        self.insert_mode.get(event).or_else(|| match &event.kind {
            EventKind::Key(KeyEvent {
                key_code: KeyCode::Char(c),
                modifiers,
            }) if modifiers.is_empty() => Some(InsertModeEvent::InsertChar(*c)),

            EventKind::Key(KeyEvent {
                key_code: KeyCode::Return,
                modifiers,
            }) if modifiers.is_empty() => Some(InsertModeEvent::InsertChar('\n')),

            EventKind::String(s) => Some(InsertModeEvent::InsertString(s)),

            _ => None,
        })
    }

    pub fn command_mode(&self, event: &Event) -> Option<CommandModeEvent> {
        self.command_mode.get(event)
    }

    pub fn editor_root(&self, event: &Event) -> Option<EditorRootEvent> {
        self.editor_root.get(event)
    }

    pub fn vsplit(&self, event: &Event) -> Option<VSplitEvent> {
        self.vsplit.get(event)
    }
}
