use std::io::Result;
use std::ops::{Deref, DerefMut, Range, RangeFrom, RangeTo};
use std::path::{Path, PathBuf};

use super::buffer::Buffer;
use super::file::TextFileInfo;
use druid::Data;

#[derive(Debug, Clone, Default)]
pub struct EditStack {
    pub buffer: Buffer,
    undo_stack: Vec<Buffer>,
    redo_stack: Vec<Buffer>,
    pub file: TextFileInfo,
    pub filename: Option<PathBuf>,
    dirty: bool,
}

impl Data for EditStack {
    fn same(&self, other: &Self) -> bool {
        self.buffer.same(&other.buffer)
            && self.file == other.file
            && self.filename == other.filename
            && self.dirty == other.dirty
    }
}

const AUTO_INSERT_CHARS: &[&str] = &["{}", "\"\"", "<>", "()", "[]"];

impl EditStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn selected_text(&self) -> String {
        self.buffer.selected_text(self.file.linefeed)
    }

    pub fn main_cursor_selected_text(&self) -> String {
        self.buffer.main_cursor_selected_text()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = TextFileInfo::load(&path)?;
        let buffer = Buffer::from_rope(file.1, file.0.indentation.visible_len());
        Ok(Self {
            buffer,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file: file.0,
            filename: Some(path.as_ref().to_path_buf()),
            dirty: false,
        })
    }

    pub fn open<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let editor = EditStack::from_file(path)?;
        let _ = std::mem::replace(self, editor);
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.file.save_as(&self.buffer, &path)?;
        self.filename = Some(path.as_ref().to_path_buf());
        self.dirty = false;
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(buffer) = self.undo_stack.pop() {
            let b = std::mem::take(&mut self.buffer);
            self.redo_stack.push(b);
            self.buffer = buffer;
        }
        if self.undo_stack.is_empty() {
            self.dirty = false;
        }
    }

    pub fn redo(&mut self) {
        if let Some(buffer) = self.redo_stack.pop() {
            let b = std::mem::take(&mut self.buffer);
            self.undo_stack.push(b);
            self.buffer = buffer;
        }
    }

    fn push_edit(&mut self, buffer: Buffer) {
        let b = std::mem::take(&mut self.buffer);
        self.undo_stack.push(b);
        self.buffer = buffer;
        self.redo_stack.clear();
        self.dirty = true;
    }

    pub fn insert(&mut self, text: &str) {
        let mut buf = self.buffer.clone();

        if text.chars().count() == 1 {
            if let Some(c) = AUTO_INSERT_CHARS
                .iter()
                .find(|c| text.chars().next().unwrap() == c.chars().next().unwrap())
            {
                if buf.have_selection() {
                    let inner_text = buf.selected_text(self.file.linefeed);
                    buf.insert(c, false);
                    buf.backward(false, false);
                    buf.insert(&inner_text, true);
                } else {
                    buf.insert(c, false);
                    buf.backward(false, false);
                }
            } else {
                buf.insert(text, false);
            }
        } else {
            buf.insert(text, false);
        }

        self.push_edit(buf);
    }

    pub fn backspace(&mut self) {
        let mut buf = self.buffer.clone();

        // TODO check if old buf is same that new with the Data trait
        if buf.backspace() {
            self.push_edit(buf);
        }
    }

    pub fn delete(&mut self) {
        let mut buf = self.buffer.clone();

        if buf.delete() {
            self.push_edit(buf);
        }
    }

    pub fn tab(&mut self) {
        let mut buf = self.buffer.clone();
        buf.tab(self.file.indentation);
        self.push_edit(buf);
    }
}

impl Deref for EditStack {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for EditStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

#[derive(Debug, Clone)]
pub enum SelectionLineRange {
    Range(Range<usize>),
    RangeTo(RangeTo<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull,
}
