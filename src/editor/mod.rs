mod editor_root;

mod event;
mod keymaps;
mod pane;
mod text_field;
mod vsplit;

pub use editor_root::EditorRoot;

use self::keymaps::*;

#[derive(Default)]
pub struct EditorState {
    pub key_maps: KeyMaps,
}
