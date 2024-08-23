#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction {
    ToggleInsert,
    Typing(char),
    Remove,
    Delete,
    Cut,
    Paste,
    Copy,
    CursorLeft,
    CursorRight,
    CursorLeftSelect,
    CursorRightSelect,
    ToStart,
    ToStartSelect,
    ToEnd,
    ToEndSelect,
    SelectAll,
    Null
}
