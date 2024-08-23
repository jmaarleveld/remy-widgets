use arboard::Clipboard;
use super::cursor::{Cursor, CursorCoordinate, CursorToCharIndexMapper};
use super::actions::UserAction;



pub trait TypingBehaviour: Default {
    fn handle_user_action(&mut self, action: UserAction);
    fn get_visible_text(&mut self, width: usize) -> (String, Option<(usize, usize)>);
    
    fn get_cursor_position(&mut self, width: usize) -> usize;
    
    fn get_text(&self) -> &str;
}

pub struct DefaultTypingBehaviour<T: CursorToCharIndexMapper> {
    cursor: Cursor<T>,
    insert_enabled: bool,
    selection: Option<(CursorCoordinate, CursorCoordinate)>,
    clipboard: Clipboard
}

impl<T: CursorToCharIndexMapper> DefaultTypingBehaviour<T> {
    pub fn new(text: String, clipboard: Clipboard) -> Self {
        Self {
            cursor: Cursor::new(text),
            insert_enabled: false,
            selection: None,
            clipboard
        }
    }
    
    fn maybe_clear_selection(&mut self) {
        let _ = self.selection.take();
    }
    
    fn action_toggle_insert(&mut self) {
        self.maybe_clear_selection();
        self.insert_enabled = !self.insert_enabled;
    }
    
    fn action_typing(&mut self, c: char) {
        match self.selection.take() {
            None => {
                if self.insert_enabled {
                    self.cursor.replace_char_at_cursor(c);
                } else {
                    self.cursor.insert_char_at_cursor(c);
                }
                self.cursor.move_right();
            }
            Some((start, end)) => {
                self.cursor.set_position(start);
                self.cursor.delete_string_at_cursor(end);
                self.cursor.insert_char_at_cursor(c);
                if !self.insert_enabled {
                    self.cursor.move_right();
                }
            }
        }
    }
    
    fn action_backspace(&mut self) {
        self.action_remove(false);
    }
    
    fn action_delete(&mut self) {
        self.action_remove(true);
    }
    
    fn action_remove(&mut self, inplace: bool) {
        match self.selection.take() {
            None => {
                if inplace {
                    self.cursor.delete_char_at_cursor();
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if self.cursor.move_left() { 
                        self.cursor.delete_char_at_cursor();
                    }
                }
            }
            Some((start, end)) => {
                self.cursor.set_position(start);
                self.cursor.delete_string_at_cursor(end);
            }
        }
    }
    
    fn action_cursor_left(&mut self) {
        self.maybe_clear_selection();
        self.cursor.move_left();
    }
    
    fn action_cursor_right(&mut self) {
        self.maybe_clear_selection();
        self.cursor.move_right();
    }
    
    fn action_cursor_left_select(&mut self) {
        let (start, stop) = match self.selection {
            None => {
                let stop = self.cursor.get_position();
                self.cursor.move_left();
                (self.cursor.get_position(), stop)
            }
            Some((start, stop)) => {
                let previous_pos = self.cursor.get_position();
                self.cursor.move_left();
                if previous_pos == start {
                    (self.cursor.get_position(), stop)
                } else {
                    (start, self.cursor.get_position())
                }
            }
        };
        self.update_selection(start, stop);
    }
    
    fn action_cursor_right_select(&mut self) {
        let (start, stop) = match self.selection {
            None => {
                let start = self.cursor.get_position();
                self.cursor.move_right();
                (start, self.cursor.get_position())
            }
            Some((start, stop)) => {
                let previous_pos = self.cursor.get_position();
                self.cursor.move_right();
                if previous_pos == start {
                    (self.cursor.get_position(), stop)
                } else {
                    (start, self.cursor.get_position())
                }
            }
        };
        self.update_selection(start, stop);
    }
    
    fn action_cursor_to_start(&mut self) {
        self.maybe_clear_selection();
        self.cursor.move_to_start();
    }
    
    fn action_cursor_to_end(&mut self) {
        self.maybe_clear_selection();
        self.cursor.move_to_end();
    }
    
    fn action_cursor_to_start_select(&mut self) {
        let stop = match self.selection {
            None => self.cursor.get_position(),
            Some((_, stop)) => stop
        };
        self.cursor.move_to_start();
        self.update_selection(self.cursor.get_position(), stop);
    }
    
    fn action_cursor_to_end_select(&mut self) {
        let start = match self.selection {
            None => self.cursor.get_position(),
            Some((start, _)) => start
        };
        self.cursor.move_to_end();
        self.update_selection(start, self.cursor.get_position());
    }
    
    fn action_select_all(&mut self) {
        let pos = self.cursor.get_position();
        self.cursor.move_to_start();
        let start = self.cursor.get_position();
        self.cursor.move_to_end();
        let stop = self.cursor.get_position();
        self.cursor.set_position(pos);
        self.update_selection(start, stop);
    }
    
    fn update_selection(&mut self, 
                        start: CursorCoordinate,
                        stop: CursorCoordinate) {
        if start != stop {
            self.selection = Some((start, stop));
        } else {
            self.selection = None;
        }
    }
    
    fn action_cut(&mut self) {
        if self.selection.is_some() {
            self.action_copy();
            self.action_backspace();
        }
    }
    
    fn action_copy(&mut self) {
        if let Some((start, stop)) = self.selection {
            self.clipboard
                .set_text(self.cursor.get_substring(start, stop))
                .unwrap();
        }
    }
    
    fn action_paste(&mut self) {
        match self.selection {
            None => {
                let text = self.clipboard.get_text().unwrap();  
                let end = self.cursor.insert_string_at_cursor(text.as_str());
                self.cursor.set_position(end);
            }
            Some(_) => {
                self.action_backspace();
                self.action_paste();
            }
        }
    }
}

impl<T: CursorToCharIndexMapper> Default for DefaultTypingBehaviour<T> {
    fn default() -> Self {
        Self::new(String::new(), Clipboard::new().expect("Failed to create clipboard"))
    }   
}

impl<T: CursorToCharIndexMapper> TypingBehaviour for DefaultTypingBehaviour<T> {
    fn handle_user_action(&mut self, action: UserAction) {
        match action {
            UserAction::ToggleInsert => self.action_toggle_insert(),
            UserAction::Typing(c) => self.action_typing(c),
            UserAction::Remove => self.action_backspace(),
            UserAction::Delete => self.action_delete(),
            UserAction::Cut => self.action_cut(),
            UserAction::Paste => self.action_paste(),
            UserAction::Copy => self.action_copy(),
            UserAction::CursorLeft => self.action_cursor_left(),
            UserAction::CursorRight => self.action_cursor_right(),
            UserAction::CursorLeftSelect => self.action_cursor_left_select(),
            UserAction::CursorRightSelect => self.action_cursor_right_select(),
            UserAction::ToStart => self.action_cursor_to_start(),
            UserAction::ToStartSelect => self.action_cursor_to_start_select(),
            UserAction::ToEnd => self.action_cursor_to_end(),
            UserAction::ToEndSelect => self.action_cursor_to_end_select(),
            UserAction::SelectAll => self.action_select_all(),
            UserAction::Null => ()
        }
    }

    fn get_visible_text(&mut self, width: usize) -> (String, Option<(usize, usize)>) {
        let (text, _, selection) = self.cursor.get_visible_text(width, self.selection);
        (text, selection)
    }

    fn get_cursor_position(&mut self, width: usize) -> usize {
        let (_, cursor, _) = self.cursor.get_visible_text(width, self.selection);
        cursor 
    }

    fn get_text(&self) -> &str {
        self.cursor.text()
    }
}