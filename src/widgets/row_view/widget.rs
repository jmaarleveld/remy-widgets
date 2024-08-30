//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Imports and Constants
//////////////////////////////////////////////////////////////////////////////////////////////////

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Style, Styled};
use ratatui::text::Text;
use ratatui::widgets::{StatefulWidgetRef, WidgetRef};

use crate::widgets::row_view::state::RowViewState;

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Enums
//////////////////////////////////////////////////////////////////////////////////////////////////

pub enum CellAlignment {
    Top, Bottom, Center
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Cell
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Cell<'a> {
    content: Text<'a>,
    align: CellAlignment,
    style: Style,
}

impl<'a> Default for Cell<'a> {
    fn default() -> Self {
        Self {
            content: Text::default(),
            align: CellAlignment::Top,
            style: Style::default(),
        }
    }
}

impl<'a> Cell<'a> {
    pub fn new<T: Into<Text<'a>>>(content: T) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }
    
    pub fn content<T: Into<Text<'a>>>(mut self, content: T) -> Self {
        self.content = content.into();
        self
    }

    pub fn align(mut self, align: CellAlignment) -> Self {
        self.align = align;
        self
    }
    
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    
    fn height(&self) -> usize {
        self.content.height()
    }
}

impl Cell<'_> {
    pub(super) fn render(&self,
                         area: Rect,
                         buf: &mut Buffer, 
                         max_height: Option<usize>) -> usize 
    {
        buf.set_style(area, self.style);
        let text_height = self.content.height();
        if let Some(height) = max_height {
            if text_height < height {
                // Pad according to alignment 
                let top_offset = match self.align {
                    CellAlignment::Top => 0,
                    CellAlignment::Bottom => height - text_height,
                    CellAlignment::Center => (height - text_height) / 2
                };
                let text_area = Rect {
                    x: area.x,
                    y: area.y + top_offset as u16,
                    width: area.width,
                    height: text_height as u16,
                };
                self.content.render_ref(text_area, buf);
            } else {
                // Pad top, truncate to fit 
                self.content.render_ref(area, buf);
            }
            height
        } else {
            // Either use all necessary space, or truncate to fit 
            self.content.render_ref(area, buf);
            usize::min(area.height as usize, text_height)   
        }
    }
}

impl<'a> Styled for Cell<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
} 

impl<'a, T: Into<Text<'a>>> From<T> for Cell<'a> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Row
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Row<'a> {
    cells: Vec<(Cell<'a>, Constraint)>,
    height: Option<u16>,
    // row defaults, overridden by cell-specific settings
    style: Style,
}

impl<'a> Row<'a> {
    pub fn new(cells: impl IntoIterator<Item=(Cell<'a>, Constraint)>) -> Self {
        Self {
            cells: cells.into_iter().collect(),
            height: Some(1),
            style: Style::default(),
        }
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }
    
    pub fn expand_to_fit(mut self) -> Self {
        self.height = None;
        self
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, A, B> FromIterator<(A, B)> for Row<'a> 
where 
    A: Into<Cell<'a>>,
    B: Into<Constraint>,
{
    fn from_iter<T: IntoIterator<Item=(A, B)>>(iter: T) -> Self {
        Self::new(iter.into_iter().map(|(a, b)| (a.into(), b.into())))
    }
}

impl Row<'_> {
    pub(super) fn render(&self, area: Rect, buf: &mut Buffer) -> usize {
        buf.set_style(area, self.style);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.cells.iter().map(|(_, c)| c).copied());
        if let Some(height) = self.height {
            let actual_area = if height < area.height {
                area 
            } else {
                Rect { x: area.x, y: area.y, width: area.width, height }
            };
            let rectangles = layout.split(actual_area);
            for (rect, (cell, _)) in rectangles.iter().zip(self.cells.iter()) {
                cell.render(*rect, buf, Some(actual_area.height as usize));
            }
            actual_area.height as usize
        } else {
            let rectangles = layout.split(area);
            let height = rectangles.iter()
                .copied()
                .zip(self.cells.iter())
                .map(|(rect, (cell, _))| cell.render(rect, buf, None))
                .max()
                .unwrap_or(0);
            height 
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Row View 
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RowView<'a> {
    rows: Vec<Row<'a>>,
    selected_style: Style
}

impl<'a> RowView<'a> {
    pub fn new<T: Into<Row<'a>>>(rows: impl IntoIterator<Item=T>) -> Self {
        Self {
            rows: rows.into_iter().map(|r| r.into()).collect(),
            selected_style: Style::default(),
        }
    }
    
    pub fn selected_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.selected_style = style.into();
        self
    }
}

impl<'a> StatefulWidgetRef for RowView<'a> {
    type State = RowViewState;

    fn render_ref(&self, 
                  mut area: Rect, 
                  buf: &mut Buffer, 
                  state: &mut Self::State) {
        if self.rows.is_empty() {
            state.select(None);
        } else if state.selected.is_some_and(|i| i >= self.rows.len()) {
            state.select(Some(self.rows.len() - 1));
        }
        let mut remaining_height = area.height as usize;
        let mut index = 0;
        while remaining_height > 0 && self.rows.len() < index + state.view_offset {
            let row = &self.rows[index + state.view_offset];
            let row_height = row.render(area, buf);
            if state.selected.is_some_and(|i| i == index + state.view_offset) {
                buf.set_style(area, self.selected_style);   // overwrite row style 
            }
            remaining_height -= row_height;
            area.y += row_height as u16;
            index += 1;
        }
    }
}

impl<'a> WidgetRef for RowView<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut state = RowViewState::default();
        <Self as StatefulWidgetRef>::render_ref(self, area, buf, &mut state);
    }
}
