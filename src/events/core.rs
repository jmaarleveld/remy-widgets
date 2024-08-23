#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GenericEvent {
    KeyPress{ctrl: bool, shift: bool, alt: bool, key: Key, kind: GenericKeyEventKind},
    Null
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GenericKeyEventKind {
    Press,
    Release
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    Character(char),
    Esc, 
    F(u8),
    Backspace,
    Enter,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    Tab,
    BackTab,
    CapsLock,
    NumLock,
    ScrollLock,
    Pause,
    PrintScreen,
    Menu
}