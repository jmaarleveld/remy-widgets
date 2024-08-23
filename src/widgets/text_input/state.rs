use crate::widgets::core::RemyWidgetState;
use crate::widgets::text_field::TextFieldState;
use crate::widgets::text_input::input::TextInputAction;
use crate::widgets::util::cursor::TypingBehaviour;

#[derive(Debug, Default, Clone)]
pub struct TextInputState<T: TypingBehaviour>(pub(super) TextFieldState<T>);

pub enum TextInputEvent {
    Submitted(String),
    Cancelled,
    Typing
}


impl<T: TypingBehaviour> TextInputState<T> {
    pub fn new(behaviour: T) -> Self {
        Self(TextFieldState::new(behaviour))
    }

    pub fn get_cursor_location(&mut self, width: usize) -> usize {
        self.0.get_cursor_location(width)
    }

    pub fn get_visible_text(&mut self, width: usize) -> (String, Option<(usize, usize)>) {
        self.0.get_visible_text(width)
    }

    fn text(&self) -> &str {
        self.0.text()
    }
}


impl<B: TypingBehaviour> RemyWidgetState for TextInputState<B> {
    type Command = TextInputAction;
    type EventOutput = TextInputEvent;

    fn handle_native_event(&mut self, event: Option<Self::Command>) -> Self::EventOutput {
        match event {
            Some(inner) => match inner {
                TextInputAction::Esc => TextInputEvent::Cancelled,
                TextInputAction::Enter => TextInputEvent::Submitted(self.0.text().to_string()),
                TextInputAction::Other(a) => {
                    let _ = self.0.handle_native_event(Some(a));
                    TextInputEvent::Typing
                }
                TextInputAction::Null => TextInputEvent::Typing,
            }
            None => TextInputEvent::Typing
        }
    }
}
