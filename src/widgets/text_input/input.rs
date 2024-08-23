use crate::events::{Event, Key};
use crate::widgets::core::RemyWidgetCommandConverter;
use crate::widgets::text_field::{DefaultTextFieldInputConverter, TextFieldState};
use crate::widgets::util::cursor::{TypingBehaviour, UserAction};
use super::state::TextInputState;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputAction {
    Esc,
    Enter,
    Other(UserAction),
    Null
}

impl From<TextInputAction> for UserAction {
    fn from(value: TextInputAction) -> Self {
        match value {
            TextInputAction::Esc => UserAction::Null,
            TextInputAction::Enter => UserAction::Null,
            TextInputAction::Other(inner) => inner,
            TextInputAction::Null => UserAction::Null
        }
    }
}


pub struct DefaultTextInputInputConverter;


impl<T> RemyWidgetCommandConverter<TextInputState<T>> for DefaultTextInputInputConverter
where
    T: TypingBehaviour,
{
    type Event = TextInputAction;

    fn convert(event: Event, state: &TextInputState<T>) -> Option<Self::Event> {
        match event {
            Event::KeyPress { key, .. } => {
                match key {
                    Key::Esc => Some(TextInputAction::Esc),
                    Key::Enter => Some(TextInputAction::Enter),
                    _ => match <DefaultTextFieldInputConverter as RemyWidgetCommandConverter<TextFieldState<T>>>::convert(event, &state.0) {
                        None => None,
                        Some(a) => Some(TextInputAction::Other(a))
                    }
                }
            }
            Event::Null => None
        }
    }
}
