#[derive(Debug, Copy, Clone, Default)]
pub struct RowViewState {
    pub(super) selected: Option<usize>,
    pub(super) view_offset: usize
}

impl RowViewState {
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
    
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == Some(index)
    }
}
