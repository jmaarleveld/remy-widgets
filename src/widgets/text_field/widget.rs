use std::marker::PhantomData;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{StatefulWidgetRef, Widget};
use crate::widgets::core::StatefulRemyWidget;
use crate::widgets::text_field::state::TextFieldState;
use crate::widgets::util::cursor::TypingBehaviour;

pub struct TextField<T: TypingBehaviour> {
    normal_style: Option<Style>,
    selection_style: Option<Style>,
    _behaviour: PhantomData<T>
}

impl<T: TypingBehaviour> TextField<T> {
    pub fn new() -> Self {
        Self {
            normal_style: None,
            selection_style: None,
            _behaviour: PhantomData
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.normal_style = Some(style);
        self
    }

    pub fn with_selection_style(mut self, style: Style) -> Self {
        self.selection_style = Some(style);
        self
    }

    pub fn with_style_and_inverted_selection(self, s: Style) -> Self {
        let fg = s.bg.unwrap_or_else(|| Style::default().bg.unwrap());
        let bg = s.fg.unwrap_or_else(|| Style::default().fg.unwrap());
        self.with_style(s)
            .with_selection_style(Style::default().fg(fg).bg(bg))
    }
}




impl<T: TypingBehaviour> StatefulWidgetRef for TextField<T> {
    type State = TextFieldState<T>;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let width = area.width as usize;
        let (text, selection) = state.get_visible_text(width);
        let formatted = if let Some((start, end)) = selection {
            let first = &text[0..start];
            let second = &text[start..end];
            let third = &text[end..];
            Line::from(vec![
                self.normal_style
                    .map(|s| Span::styled(first, s))
                    .unwrap_or(Span::raw(first)),
                self.selection_style
                    .map(|s| Span::styled(second, s))
                    .unwrap_or(Span::raw(second)),
                self.normal_style
                    .map(|s| Span::styled(third, s))
                    .unwrap_or(Span::raw(third))
            ])
        } else {
            match self.normal_style {
                None => Line::styled(text, Style::default()),
                Some(style) => Line::styled(text, style)
            }
        };
        formatted.render(area, buf);
    }
}

impl<T: TypingBehaviour> StatefulRemyWidget for TextField<T> {
    type Input = TextFieldState<T>;
}
