pub enum NormalModeEvent {
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

pub enum InsertModeEvent {
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
