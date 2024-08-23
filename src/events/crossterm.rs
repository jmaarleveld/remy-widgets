use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crate::events::core::{GenericEvent, GenericKeyEventKind, Key};

impl From<Event> for GenericEvent {
    fn from(value: Event) -> Self {
        if let Event::Key(inner) = value {
            if inner.kind == KeyEventKind::Press || inner.kind == KeyEventKind::Release {
                let key = match inner.code {
                    KeyCode::Backspace => Key::Backspace,
                    KeyCode::Enter => Key::Enter,
                    KeyCode::Left => Key::ArrowLeft,
                    KeyCode::Right => Key::ArrowRight,
                    KeyCode::Up => Key::ArrowUp,
                    KeyCode::Down => Key::ArrowDown,
                    KeyCode::Home => Key::Home,
                    KeyCode::End => Key::End,
                    KeyCode::PageUp => Key::PageUp,
                    KeyCode::PageDown => Key::PageDown,
                    KeyCode::Tab => Key::Tab,
                    KeyCode::BackTab => Key::BackTab,
                    KeyCode::Delete => Key::Delete,
                    KeyCode::Insert => Key::Insert,
                    KeyCode::F(n) => Key::F(n),
                    KeyCode::Char(c) => Key::Character(c),
                    KeyCode::Null => {
                        return GenericEvent::Null;
                    }
                    KeyCode::Esc => Key::Esc,
                    KeyCode::CapsLock => Key::CapsLock,
                    KeyCode::ScrollLock => Key::ScrollLock,
                    KeyCode::NumLock => Key::NumLock,
                    KeyCode::PrintScreen => Key::PrintScreen,
                    KeyCode::Pause => Key::Pause,
                    KeyCode::Menu => Key::Menu,
                    KeyCode::KeypadBegin => {
                        return GenericEvent::Null;
                    }
                    KeyCode::Media(_) => {
                        return GenericEvent::Null;
                    }
                    KeyCode::Modifier(_) => {
                        return GenericEvent::Null;
                    }
                };
                return GenericEvent::KeyPress {
                    key,
                    ctrl: inner.modifiers.contains(KeyModifiers::CONTROL),
                    shift: inner.modifiers.contains(KeyModifiers::SHIFT),
                    alt: inner.modifiers.contains(KeyModifiers::ALT),
                    kind: if inner.kind == KeyEventKind::Press {
                        GenericKeyEventKind::Press
                    } else {
                        GenericKeyEventKind::Release
                    }
                }
            };
        }
        GenericEvent::Null
    }
}
