#[derive(Debug, Clone)]
pub enum NormalModeEvent {
    InsertMode,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    MoveHome,
    MoveEnd,
}

#[derive(Debug, Clone)]
pub enum InsertModeEvent {
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

    Escape,
}

#[derive(Debug, Clone)]
pub enum CommandModeEvent {
    Escape,
}

#[derive(Debug, Clone)]
pub enum EditorRootEvent {
    CommandMode,
    Quit,
}

#[derive(Debug, Clone)]
pub enum VSplitEvent {
    FocusUp,
    FocusDown,
}
