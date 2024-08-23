use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

use ratatui::widgets::TableState;

use crate::widgets::core::RemyWidgetState;
use crate::widgets::explorer::input::FileExplorerCommand;
use crate::widgets::text_input::{TextInputEvent, TextInputState};
use crate::widgets::util::cursor::TypingBehaviour;

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// File Information Structs
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInformation {
    pub(super) file_type: FileType,
    pub(super) name: String,
}

impl Ord for FileInformation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.file_type.cmp(&other.file_type) {
            Ordering::Equal => self.name.cmp(&other.name),
            o => o
        }
    }
}

impl PartialOrd for FileInformation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(super) enum FileType {
    File,
    Directory,
    FileSymlink,
    DirectorySymlink
}

impl FileType {
    pub(super) fn is_symlink(&self) -> bool {
        matches!(self, FileType::FileSymlink | FileType::DirectorySymlink)
    }

    pub(super) fn is_dir(&self) -> bool {
        matches!(self, FileType::Directory | FileType::DirectorySymlink)
    }

    pub(super) fn is_file(&self) -> bool {
        matches!(self, FileType::File | FileType::FileSymlink)
    }
}

impl Ord for FileType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (x, y) if x == y => Ordering::Equal,
            (FileType::File, _) => Ordering::Less,
            (_, FileType::File) => Ordering::Greater,
            (FileType::FileSymlink, _) => Ordering::Less,
            (_, FileType::FileSymlink) => Ordering::Greater,
            (FileType::Directory, _) => Ordering::Less,
            (_, FileType::Directory) => Ordering::Greater,
            (FileType::DirectorySymlink, _) => Ordering::Less
        }
    }
}

impl PartialOrd for FileType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Dialog type
//////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FileDialogType {
    AskFilename,
    AskDirectory,
    AskSaveAsFilename
}

impl FileDialogType {
    pub(super) fn show_filenames(&self) -> bool {
        match self {
            FileDialogType::AskFilename => true,
            FileDialogType::AskDirectory => false,
            FileDialogType::AskSaveAsFilename => true
        }
    }

    pub(super) fn allow_selecting_directory(&self) -> bool {
        match self {
            FileDialogType::AskFilename => false,
            FileDialogType::AskDirectory => true,
            FileDialogType::AskSaveAsFilename => false
        }
    }

    pub(super) fn allow_selecting_files(&self) -> bool {
        match self {
            FileDialogType::AskFilename => true,
            FileDialogType::AskDirectory => false,
            FileDialogType::AskSaveAsFilename => true
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// Event
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileExplorerEvent {
    Selecting,
    Selected(PathBuf),
    Cancelled
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////
// File Explorer State
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileExplorerState<T: TypingBehaviour> {
    pub(super) table_state: TableState,
    pub(super) directory: PathBuf,
    pub(super) files: Vec<FileInformation>,
    pub(super) io_error: Option<String>,
    pub(super) filename_input_state: Option<TextInputState<T>>,
    pub(super) dialog_type: FileDialogType
}


impl<T: TypingBehaviour> FileExplorerState<T> {
    pub fn new(directory: PathBuf, dialog_type: FileDialogType) -> anyhow::Result<Self> {
        let (files, table_state, io_error) = match Self::fresh_state(&directory) {
            Ok(res) => (res.0, res.1, None),
            Err(e) => (Vec::new(), TableState::default(), Some(e.to_string()))
        };
        Ok(Self { table_state, directory, files, io_error, filename_input_state: None, dialog_type })
    }

    pub fn cwd(file_dialog_type: FileDialogType) -> anyhow::Result<Self> {
        let directory = std::env::current_dir()?;
        Self::new(directory, file_dialog_type)
    }

    pub(super) fn update_directory(&mut self, path: PathBuf) -> anyhow::Result<()> {
        match Self::fresh_state(&path) {
            Ok((files, table_state)) => {
                self.directory = path;
                self.files = files;
                self.table_state = table_state;
                //self.io_error = None;
                Ok(())
            }
            Err(e) => {
                self.io_error = Some(e.to_string());
                Ok(())
            }
        }
    }
    
    fn fresh_state(path: &Path) -> anyhow::Result<(Vec<FileInformation>, TableState)> {
        let mut files = Self::collect_files(path)?;
        files.sort();
        let table_state = TableState::new()
            .with_selected(0)
            .with_offset(0);
        Ok((files, table_state))
    }

    fn collect_files(path: &Path) -> anyhow::Result<Vec<FileInformation>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_type = if metadata.is_file() {
                FileType::File
            } else if metadata.is_dir() {
                FileType::Directory
            } else {
                assert!(metadata.is_symlink());
                let symlink_metadata = fs::metadata(entry.path())?;
                if symlink_metadata.is_file() {
                    FileType::FileSymlink
                } else {
                    assert!(symlink_metadata.is_dir());
                    FileType::DirectorySymlink
                }
            };
            let name = entry.file_name()
                .into_string()
                .expect("Failed to convert filename");
            files.push(FileInformation { file_type, name });
        }
        Ok(files)
    }

    // pub fn handle_events(&mut self) -> anyhow::Result<FileExplorerEvent> {
    //     
    // }

    fn move_arrow_up(&mut self) -> anyhow::Result<FileExplorerEvent> {
        let sel = self.table_state.selected().unwrap();
        if sel > 0 {
            self.table_state.select(Some(sel - 1));
        }
        Ok(FileExplorerEvent::Selecting)
    }

    fn move_arrow_down(&mut self) -> anyhow::Result<FileExplorerEvent> {
        let sel =  self.table_state.selected().unwrap();
        if sel + 1 < self.files.len() {
            self.table_state.select(Some(sel + 1));
        }
        Ok(FileExplorerEvent::Selecting)
    }

    fn move_directory_up(&mut self) -> anyhow::Result<FileExplorerEvent> {
        if let Some(parent) = self.directory.parent() {
            let old = self.directory.file_name()
                .expect("Failed to get filename")
                .to_os_string()
                .into_string()
                .expect("Failed to convert filename");
            self.update_directory(parent.to_path_buf())?;
            let index = self.files.iter().enumerate()
                .find(|(_, info)| info.name == old)
                .expect("Failed to find file index");
            self.table_state.select(Some(index.0));
        }
        Ok(FileExplorerEvent::Selecting)
    }

    fn move_directory_down(&mut self) -> anyhow::Result<FileExplorerEvent> {
        let sel =  self.table_state.selected().unwrap();
        let info = self.files.get(sel).unwrap();
        if info.file_type == FileType::Directory {
            let path = self.directory.join(info.name.clone());
            self.update_directory(path)?;
        }
        Ok(FileExplorerEvent::Selecting)
    }

    fn get_selected_file(&mut self) -> anyhow::Result<FileExplorerEvent> {
        let sel =  self.table_state.selected().unwrap();
        let info = self.files.get(sel).unwrap();
        if info.file_type.is_dir() && !self.dialog_type.allow_selecting_directory() {
            return Ok(FileExplorerEvent::Selecting)
        }
        if info.file_type.is_file() && !self.dialog_type.allow_selecting_files() {
            return Ok(FileExplorerEvent::Selecting)
        }
        let path = self.directory.join(info.name.clone());
        Ok(FileExplorerEvent::Selected(path))
    }
}


impl<T: TypingBehaviour> RemyWidgetState for FileExplorerState<T> {
    type Command = FileExplorerCommand;
    type EventOutput = anyhow::Result<FileExplorerEvent>;

    fn handle_native_event(&mut self, event: Option<Self::Command>) -> Self::EventOutput {
        if let Some(command) = event {
            let _ = self.io_error.take();      // Reset error on key action
            match command {
                FileExplorerCommand::ArrowUp => self.move_arrow_up(),
                FileExplorerCommand::ArrowDown => self.move_arrow_down(),
                FileExplorerCommand::DirectoryUp => self.move_directory_up(),
                FileExplorerCommand::DirectoryDown => self.move_directory_down(),
                FileExplorerCommand::Confirm => self.get_selected_file(),
                FileExplorerCommand::EnterFileName => {
                    // Setting the state will also update the ui 
                    self.filename_input_state = Some(TextInputState::default());
                    Ok(FileExplorerEvent::Selecting)
                },
                FileExplorerCommand::Exit => Ok(FileExplorerEvent::Cancelled),
                FileExplorerCommand::FileNameDialogInput(inner) => {
                    let state = self.filename_input_state
                        .as_mut()
                        .expect("No filename input state");
                    let event = state.handle_native_event(Some(inner));
                    match event {
                        TextInputEvent::Submitted(filename) => {
                            let full_path = self.directory.join(filename);
                            Ok(FileExplorerEvent::Selected(full_path))
                        }
                        TextInputEvent::Cancelled => {
                            self.filename_input_state = None;
                            Ok(FileExplorerEvent::Selecting)
                        }
                        TextInputEvent::Typing => Ok(FileExplorerEvent::Selecting)
                    }
                    
                }
            }
        } else {
            Ok(FileExplorerEvent::Selecting)
        }
    }
}
