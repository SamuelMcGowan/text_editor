mod editor_root;

mod event;
mod keybindings;
mod pane;
mod text_field;
mod vsplit;

pub use editor_root::EditorRoot;

use self::event::*;
use self::keybindings::*;

pub struct EditorState {
    pub normal_mode_keybindings: Keybindings<NormalModeEvent>,
    pub insert_mode_keybindings: Keybindings<InsertModeEvent>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            normal_mode_keybindings: normal_mode_keybindings(),
            insert_mode_keybindings: insert_mode_keybindings(),
        }
    }
}
