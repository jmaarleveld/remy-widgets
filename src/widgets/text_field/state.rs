use crate::widgets::core::RemyWidgetState;
use crate::widgets::util::cursor::{TypingBehaviour, UserAction};

#[derive(Debug, Default, Clone)]
pub struct TextFieldState<T: TypingBehaviour>(T);

impl<T: TypingBehaviour> TextFieldState<T> {
    pub fn new(behaviour: T) -> Self {
        Self(behaviour)
    }

    pub fn get_cursor_location(&mut self, width: usize) -> usize {
        self.0.get_cursor_position(width)
    }

    pub fn get_visible_text(&mut self, width: usize) -> (String, Option<(usize, usize)>) {
        self.0.get_visible_text(width)
    }

    pub fn text(&self) -> &str {
        self.0.get_text()
    }
}


impl<B: TypingBehaviour> RemyWidgetState for TextFieldState<B> {
    type Command = UserAction;
    type EventOutput = ();

    fn handle_native_event(&mut self, event: Option<Self::Command>) -> Self::EventOutput {
        if let Some(action) = event {
            self.0.handle_user_action(action);
        }
    }
}
