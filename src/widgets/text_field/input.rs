use crate::events::{Event, Key};
use crate::widgets::core::RemyWidgetCommandConverter;
use crate::widgets::text_field::state::TextFieldState;
use crate::widgets::util::cursor::{TypingBehaviour, UserAction};


pub struct DefaultTextFieldInputConverter;


impl<T> RemyWidgetCommandConverter<TextFieldState<T>> for DefaultTextFieldInputConverter
where T: TypingBehaviour
{
    type Event = UserAction;

    fn convert(event: Event, _state: &TextFieldState<T>) -> Option<Self::Event> {
        match event {
            Event::KeyPress { shift, ctrl, key, .. } => {
                match key {
                    Key::Backspace => Some(UserAction::Remove),
                    Key::Delete => Some(UserAction::Delete),
                    Key::Insert => Some(UserAction::ToggleInsert),
                    Key::ArrowLeft if shift => Some(UserAction::CursorLeftSelect),
                    Key::ArrowLeft => Some(UserAction::CursorLeft),
                    Key::ArrowRight if shift => Some(UserAction::CursorRightSelect),
                    Key::ArrowRight => Some(UserAction::CursorRight),
                    Key::Character('c') if ctrl => Some(UserAction::Copy),
                    Key::Character('v') if ctrl => Some(UserAction::Paste),
                    Key::Character('x') if ctrl => Some(UserAction::Cut),
                    Key::Character('a') if ctrl => Some(UserAction::SelectAll),
                    Key::Character(c) => Some(UserAction::Typing(c)),
                    Key::Home if shift => Some(UserAction::ToStartSelect),
                    Key::Home => Some(UserAction::ToStart),
                    Key::End if shift => Some(UserAction::ToEndSelect),
                    Key::End => Some(UserAction::ToEnd),
                    _ => None
                }
            }
            Event::Null => None
        }
    }
}
