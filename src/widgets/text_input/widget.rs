use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::widgets::StatefulWidgetRef;
use crate::widgets::core::StatefulRemyWidget;
use crate::widgets::text_field::TextField;
use crate::widgets::text_input::state::TextInputState;
use crate::widgets::util::cursor::TypingBehaviour;

pub struct TextInput<T: TypingBehaviour>(TextField<T>);


impl<T: TypingBehaviour> TextInput<T> {
    pub fn new() -> Self {
        Self(TextField::new())
    }

    pub fn with_style(self, style: Style) -> Self {
        Self(self.0.with_style(style))
    }

    pub fn with_selection_style(self, style: Style) -> Self {
        Self(self.0.with_selection_style(style))
    }

    pub fn with_style_and_inverted_selection(self, s: Style) -> Self {
        Self(self.0.with_style_and_inverted_selection(s))
    }
}


impl<T: TypingBehaviour> StatefulWidgetRef for TextInput<T> {
    type State = TextInputState<T>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.0.render_ref(area, buf, &mut state.0)
    }
}

impl<T: TypingBehaviour> StatefulRemyWidget for TextInput<T> {
    type Input = TextInputState<T>;
}
