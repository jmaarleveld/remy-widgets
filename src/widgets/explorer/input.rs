use crate::events::{Event, Key};
use crate::widgets::core::RemyWidgetCommandConverter;
use crate::widgets::explorer::FileExplorerState;
use crate::widgets::text_input::{DefaultTextInputInputConverter, TextInputAction, TextInputState};
use crate::widgets::util::cursor::TypingBehaviour;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileExplorerCommand {
    ArrowUp,
    ArrowDown,
    DirectoryUp,
    DirectoryDown,
    Exit,
    Confirm,
    EnterFileName,
    FileNameDialogInput(TextInputAction)
}

pub struct DefaultFileExplorerInputConverter;

impl<T> RemyWidgetCommandConverter<FileExplorerState<T>> for DefaultFileExplorerInputConverter
where
    T: TypingBehaviour
{
    type Event = FileExplorerCommand;

    fn convert(event: Event, state: &FileExplorerState<T>) -> Option<Self::Event> {
        if let Some(s) = state.filename_input_state.as_ref() {
            let inner = <DefaultTextInputInputConverter as RemyWidgetCommandConverter<TextInputState<T>>>::convert(event, s);
            inner.map(FileExplorerCommand::FileNameDialogInput)
        } else {
            match event {
                Event::KeyPress { key, .. } => {
                    match key {
                        Key::ArrowUp => Some(FileExplorerCommand::ArrowUp),
                        Key::ArrowDown => Some(FileExplorerCommand::ArrowDown),
                        Key::ArrowLeft => Some(FileExplorerCommand::DirectoryUp),
                        Key::ArrowRight => Some(FileExplorerCommand::DirectoryDown),
                        Key::Enter => Some(FileExplorerCommand::Confirm),
                        Key::Esc => Some(FileExplorerCommand::Exit),
                        Key::Character('n') => Some(FileExplorerCommand::EnterFileName),
                        _ => None
                    }
                }
                Event::Null => None
            }
        }
    }
}
