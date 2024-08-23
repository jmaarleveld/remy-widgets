use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use super::core::UserAction;


impl From<Event> for UserAction {
    fn from(value: Event) -> Self {
        if let Event::Key(inner) = value {
            if inner.kind == KeyEventKind::Press {
                return match inner.code {
                    KeyCode::Backspace => UserAction::Remove,
                    KeyCode::Delete => UserAction::Delete,
                    KeyCode::Insert => UserAction::ToggleInsert,
                    KeyCode::Left if inner.modifiers.contains(KeyModifiers::SHIFT) => UserAction::CursorLeftSelect,                    KeyCode::Left => UserAction::CursorLeft,
                    KeyCode::Right if inner.modifiers.contains(KeyModifiers::SHIFT) => UserAction::CursorRightSelect,
                    KeyCode::Right => UserAction::CursorRight,
                    KeyCode::Char('c') if inner.modifiers.contains(KeyModifiers::CONTROL) => UserAction::Copy,
                    KeyCode::Char('v') if inner.modifiers.contains(KeyModifiers::CONTROL) => UserAction::Paste,
                    KeyCode::Char('x') if inner.modifiers.contains(KeyModifiers::CONTROL) => UserAction::Cut,
                    KeyCode::Char('a') if inner.modifiers.contains(KeyModifiers::CONTROL) => UserAction::SelectAll,
                    KeyCode::Char(c) => UserAction::Typing(c),
                    KeyCode::Home if inner.modifiers.contains(KeyModifiers::SHIFT) => UserAction::ToStartSelect,
                    KeyCode::Home => UserAction::ToStart,
                    KeyCode::End if inner.modifiers.contains(KeyModifiers::SHIFT) => UserAction::ToEndSelect,
                    KeyCode::End => UserAction::ToEnd,
                    _ => UserAction::Null
                };
            }
        }
        UserAction::Null
    }
}
