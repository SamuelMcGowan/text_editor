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
pub enum InsertModeEvent<'a> {
    InsertChar(char),
    InsertString(&'a str),

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
