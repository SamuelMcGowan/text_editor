mod editor_root;

mod command;
mod keybindings;
mod pane;
mod text_field;
mod vsplit;

pub use editor_root::EditorRoot;

use self::command::EditorCommand;
use crate::ui::BoxedWidget;

pub type EditorBoxedWidget = BoxedWidget<EditorCommand, EditorState>;

pub struct EditorState;
