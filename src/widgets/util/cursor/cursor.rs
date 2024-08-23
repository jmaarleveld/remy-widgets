use std::marker::PhantomData;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CursorCoordinate(usize);



pub struct Cursor<T: CursorToCharIndexMapper> {
    // Actual cursor 
    position: CursorCoordinate,
    
    // Window information
    left_window_pos: CursorCoordinate,
    
    // Text 
    text: String,
    _mapper: PhantomData<T>
}


impl<T: CursorToCharIndexMapper> Cursor<T> {
    
    pub fn new(text: String) -> Self {
        Self {
            position: CursorCoordinate(0),
            left_window_pos: CursorCoordinate(0),
            text,
            _mapper: PhantomData
        }
    }
    
    // Data Access 
    
    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn get_substring(&self,
                         start: CursorCoordinate,
                         stop: CursorCoordinate) -> &str {
        let start = T::map(start, &self.text);
        let stop = T::map(stop, &self.text);
        &self.text[start..stop]
    }
    
    pub fn get_visible_text(
        &mut self,
        width: usize, 
        selection: Option<(CursorCoordinate, CursorCoordinate)>) -> (String, usize, Option<(usize, usize)>) 
    {
        let window = T::get_text_window(
            &self.text, self.left_window_pos, self.position, width
        );
        self.left_window_pos = window.cursor_start;
        let start = T::map(window.cursor_start, &self.text);
        let stop = T::map(window.cursor_end, &self.text);
        let cursor = T::map(
            CursorCoordinate(window.cursor_pos.0 - window.cursor_start.0), 
            self.get_substring(window.cursor_start, window.cursor_end)
        );
        let mapped_selection = selection.map(
            |(start, stop)| {
                let mapped_start = if start.0 < window.cursor_start.0 {
                    0
                } else {
                    start.0 - window.cursor_start.0
                };
                let mapped_stop = if stop.0 > window.cursor_end.0 {
                    window.cursor_end.0
                } else {
                    stop.0 - window.cursor_start.0
                };
                (mapped_start, mapped_stop)
            }
        );
        (self.text[start..stop].to_string(), cursor, mapped_selection)
    }
    
    // Movement 
    
    pub fn move_left(&mut self) -> bool {
        let moved = self.position.0 > 0;
        self.position = CursorCoordinate(
            self.position.0.saturating_sub(1)
        );
        moved 
    }
    
    pub fn move_right(&mut self) {
        self.position = CursorCoordinate(
            self.position.0.saturating_add(1)
                .clamp(0, T::string_length(&self.text))
        );
    }
    
    pub fn move_to_start(&mut self) {
        self.position = CursorCoordinate(0);
    }
    
    pub fn move_to_end(&mut self) {
        self.position = CursorCoordinate(T::string_length(&self.text));
    }
    
    pub fn set_position(&mut self, position: CursorCoordinate) {
        if position.0 > T::string_length(&self.text) {
            panic!("Invalid cursor position");
        }
        self.position = position;
    }
    
    pub fn get_position(&self) -> CursorCoordinate {
        self.position
    }
    
    // Editing -- char
    
    pub fn insert_char_at_cursor(&mut self, c: char) {
        let index = T::map(self.position, &self.text);
        self.text.insert(index, c);
    }
    
    pub fn delete_char_at_cursor(&mut self) -> bool {
        if self.position.0 >=  T::string_length(&self.text) {
            return false;
        }
        let index = T::map(self.position, &self.text);
        self.text.remove(index);
        true 
    }
    
    pub fn replace_char_at_cursor(&mut self, c: char) -> Option<char> {
        if self.position.0 >= T::string_length(&self.text) {
            return None;
        }
        let index = T::map(self.position, &self.text);
        let old_char = self.text.chars()
            .nth(index)
            .expect("Index out of bounds");
        self.text.replace_range(index..index+1, c.to_string().as_str());
        Some(old_char)
    }
    
    // Editing -- String
    
    pub fn insert_string_at_cursor(&mut self, s: &str) -> CursorCoordinate {
        let size = T::string_length(s);
        let index = T::map(self.position, &self.text);
        self.text.insert_str(index, s);
        CursorCoordinate(index + size)
    }
    
    pub fn delete_string_at_cursor(&mut self, second_position: CursorCoordinate) {
        let cursor_index = T::map(self.position, &self.text);
        let mut end_index = T::map(second_position, &self.text);
        end_index = end_index.clamp(0, T::string_length(&self.text));
        let first_index = cursor_index.min(end_index);
        let second_index = cursor_index.max(end_index);
        self.text.drain(first_index..second_index);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CursorWindow {
    cursor_start: CursorCoordinate,
    cursor_end: CursorCoordinate,
    cursor_pos: CursorCoordinate
}


pub trait CursorToCharIndexMapper: Sized {
    fn map(cursor_index: CursorCoordinate, text: &str) -> usize;
    
    fn string_length(text: &str) -> usize;
    
    fn cursor_coordinates(text: &str, start: CursorCoordinate) -> impl Iterator<Item=CursorCoordinate>;
    
    fn cursor_coordinates_rev(text: &str, start: CursorCoordinate) -> impl Iterator<Item=CursorCoordinate>;

    fn get_text_window(text: &str, 
                       start: CursorCoordinate, 
                       cursor: CursorCoordinate,
                       max_width: usize) -> CursorWindow 
    {
        if text.width() < max_width {
            // Show entire string 
            CursorWindow {
                cursor_start: CursorCoordinate(0),
                cursor_end: CursorCoordinate(Self::string_length(text)),
                cursor_pos: cursor
            }
        } else if cursor.0 < start.0 {
            // Cursor moved left out of bounds; cursor becomes left bound 
            CursorWindow {
                cursor_start: cursor, 
                cursor_end: get_max_index_within_width::<Self>(start, text, max_width),
                cursor_pos: cursor
            }
        } else {
            let max_reach = get_max_index_within_width::<Self>(start, text, max_width);
            if cursor.0 < max_reach.0 {
                // Cursor is in bounds 
                CursorWindow {
                    cursor_start: start,
                    cursor_end: max_reach,
                    cursor_pos: cursor
                }
            } else {
                // Cursor moved right out of bounds; cursor becomes right bound 
                CursorWindow {
                    cursor_start: get_min_index_within_width::<Self>(cursor, text, max_width),
                    cursor_end: cursor,
                    cursor_pos: cursor
                }
            }
        }
    }
}

fn get_max_index_within_width<T>(start: CursorCoordinate,
                                 text: &str,
                                 width: usize) -> CursorCoordinate 
where T: CursorToCharIndexMapper
{
    let first = T::map(start, text);
    let mut prev = start;
    let mut reached_last_element = true;
    for coord in T::cursor_coordinates(text, start) {
        let second = T::map(coord, text);
        if text[first..second].width() > width {
            reached_last_element = false;
            break;
        }
        prev = coord;
    }
    // The cursor_coordinates iterator skips the last character 
    // because indexing is non-inclusive. 
    if reached_last_element && text[first..].width() <= width {
        prev = CursorCoordinate(T::string_length(text));
    }
    prev 
}

fn get_min_index_within_width<T>(stop: CursorCoordinate,
                                 text: &str,
                                 width: usize) -> CursorCoordinate
where T: CursorToCharIndexMapper
{
    let second = T::map(stop, text);
    let mut prev = stop;
    for coord in T::cursor_coordinates_rev(text, stop) {
        let first = T::map(coord, text);
        if text[first..second].width() > width {
            break;
        }
        prev = coord;
    }
    //assert_eq!(get_max_index_within_width::<T>(prev, text, width), stop);
    prev 
}


pub struct SimpleCursorToCharIndexMapper;


impl CursorToCharIndexMapper for SimpleCursorToCharIndexMapper {
    fn map(cursor_index: CursorCoordinate, _text: &str) -> usize {
        cursor_index.0
    }
    
    fn string_length(text: &str) -> usize {
        text.chars().count()
    }

    fn cursor_coordinates(text: &str, start: CursorCoordinate) -> impl Iterator<Item=CursorCoordinate> {
        let index = Self::map(start, text);
        text[index..]
            .char_indices()
            .map(|(i, _)| CursorCoordinate(i + index))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn cursor_coordinates_rev(text: &str, start: CursorCoordinate) -> impl Iterator<Item=CursorCoordinate> {
        let index = Self::map(start, text);
        text[..index]
            .char_indices()
            .rev()
            .map(|(i, _)| CursorCoordinate(i))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
