use std::io::Result;
use std::ops::{Range, RangeFrom, RangeTo};
use std::path::{Path, PathBuf};

use druid::Data;
use ropey::Rope;
use uuid::Uuid;

use crate::carret::Carret;
use crate::carret::Carrets;
use crate::file::{Indentation, LineFeed, TextFileInfo};
use crate::position::{self, Absolute, Line, Relative};
use crate::rope_utils;

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

impl EditStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len_lines(&self) -> usize {
        self.buffer.rope.len_lines()
    }

    pub fn move_main_cursor_to(&mut self, rel: usize, line: usize, expand_selection: bool) {
        use position::Position;
        let line = position::Line::from(line); // position::Point::new(col.into(), line.into(), &self.buffer.rope, self.file.indentation.visible_len()).absolute(&self.buffer.rope, self.file.indentation.visible_len());
        let abs = line.start(&self.buffer.rope) + position::Relative::from(rel);
        self.buffer.carrets[0].set_index(
            abs,
            !expand_selection,
            true,
            &self.buffer.rope,
            self.file.indentation.visible_len(),
        );
    }

    pub fn selected_text(&self) -> String {
        self.buffer.selected_text(self.file.linefeed)
    }

    pub fn select_all(&mut self) {
        self.revert_to_single_carrets();
        self.cancel_selection();
        self.buffer.carrets[0].set_index(0.into(),true,true,&self.buffer.rope,self.file.indentation.visible_len());
        self.buffer.carrets[0].set_index(self.buffer.rope.len_bytes().into(),false,true,&self.buffer.rope,self.file.indentation.visible_len());
    }

    pub fn cursor_display_info(&self) -> String {
        if self.buffer.carrets.len() == 1 {
            format!(
                "Ln {}, Col {}",
                self.buffer.carrets[0].line().index,
                self.buffer.carrets[0].col().index
            )
        } else {
            format!("{} selections", self.buffer.carrets.len())
        }
    }

    pub fn from_file<'a, P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = TextFileInfo::load(&path)?;
        let buffer = Buffer::from_rope(file.1);
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
        std::mem::replace(self, editor);
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.file.save_as(&self.buffer.rope, &path)?;
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

    pub fn selection_on_line<'a>(&'a self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
        ranges.clear();
        for r in self.buffer.carrets.iter().filter_map(move |c| {
            if !c.selection_is_empty() {
                let r = c.range();
                match (
                    self.buffer.rope.byte_to_line(r.start.into()),
                    self.buffer.rope.byte_to_line(r.end.into()),
                ) {
                    (s, e) if s == e && s == line_idx => Some(SelectionLineRange::Range(
                        self.byte_to_line_relative_index(r.start.into())
                            ..self.byte_to_line_relative_index(r.end.into()),
                    )),
                    (s, _) if s == line_idx => Some(SelectionLineRange::RangeFrom(
                        self.byte_to_line_relative_index(r.start.into())..,
                    )),
                    (_, e) if e == line_idx => Some(SelectionLineRange::RangeTo(
                        ..self.byte_to_line_relative_index(r.end.into()),
                    )),
                    (s, e) if line_idx < e && line_idx > s => Some(SelectionLineRange::RangeFull),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            ranges.push(r);
        }
    }

    pub fn byte_to_line_relative_index(&self, index: usize) -> usize {
        index - self.buffer.rope.line_to_byte(self.buffer.rope.byte_to_line(index))
    }

    /// Construct a string with tab replaced as space
    pub fn displayable_line(
        &self,
        line: Line,
        out: &mut String,
        indices: &mut Vec<Relative>,
        byte_to_rel: &mut Vec<Relative>,
    ) {
        line.displayable_string(
            &self.buffer.rope,
            self.file.indentation.visible_len(),
            out,
            indices,
            byte_to_rel,
        );
    }

    pub fn carrets_on_line<'a>(&'a self, line: Line) -> impl Iterator<Item = &'a Carret> {
        self.buffer.carrets.iter().filter(move |c| c.line() == line)
    }

    pub fn backward(&mut self, expand_selection: bool, word_boundary: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_backward(expand_selection, word_boundary, &self.buffer.rope, self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn forward(&mut self, expand_selection: bool, word_boundary: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_forward(expand_selection, word_boundary, &buf.rope, self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn up(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_up(expand_selection, &buf.rope, self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }
    pub fn down(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_down(expand_selection, &buf.rope, self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }
    pub fn duplicate_down(&mut self) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        buf.carrets.sort_unstable();

        if let Some(c) = buf.carrets.last().and_then(|c| c.duplicate_down(&buf.rope, tabsize)) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn duplicate_up(&mut self) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        buf.carrets.sort_unstable();

        if let Some(c) = buf.carrets.first().and_then(|c| c.duplicate_up(&buf.rope, tabsize)) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn cancel_selection(&mut self) {
        let mut buf = self.buffer.clone();
        for c in &mut buf.carrets.iter_mut() {
            c.cancel_selection();
        }
        self.buffer = buf
    }

    pub fn have_selection(&self) -> bool {
        self.buffer.carrets.iter().any(|c| !c.selection_is_empty())
    }

    pub fn revert_to_single_carrets(&mut self) {
        let mut buf = self.buffer.clone();
        buf.carrets.retain(|c| !c.is_clone);
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn end(&mut self, expand_selection: bool) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_end(expand_selection, &buf.rope, tabsize);
        }

        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn home(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();
        for s in &mut buf.carrets.iter_mut() {
            s.move_home(expand_selection, &buf.rope, tabsize);
        }

        buf.carrets.merge();
        self.buffer = buf
    }

    pub fn insert(&mut self, text: &str) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        for i in 0..buf.carrets.len() {
            let r = buf.carrets[i].range();
            buf.edit(&r, text, tabsize);
            buf.carrets[i].set_index(r.start + text.len(), true, true, &buf.rope, tabsize);
        }
        buf.carrets.merge();

        self.push_edit(buf);
    }

    pub fn backspace(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            if !buf.carrets[i].selection_is_empty() {
                // delete all the selection
                let r = buf.carrets[i].range();
                buf.edit(&r, "", tabsize);
                buf.carrets[i].set_index(r.start, true, true, &buf.rope, tabsize);

                did_nothing = false;
            } else if buf.carrets[i].index > 0.into() {
                // delete the preceding grapheme
                let r = rope_utils::prev_grapheme_boundary(&buf.rope.slice(..), buf.carrets[i].index).into()
                    ..buf.carrets[i].index;
                buf.edit(&r, "", tabsize);
                buf.carrets[i].set_index(r.start, true, true, &buf.rope, tabsize);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            buf.carrets.merge();
            self.push_edit(buf);
        }
    }

    pub fn delete(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();
        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            if !buf.carrets[i].selection_is_empty() {
                let r = buf.carrets[i].range();
                buf.edit(&r, "", tabsize);
                buf.carrets[i].set_index(r.start, true, true, &buf.rope, tabsize);

                did_nothing = false;
            } else if buf.carrets[i].index < buf.rope.len_bytes().into() {
                let r = buf.carrets[i].index
                    ..rope_utils::next_grapheme_boundary(&buf.rope.slice(..), buf.carrets[i].index).into();
                buf.edit(&r, "", tabsize);
                buf.carrets[i].set_index(r.start, true, true, &buf.rope, tabsize);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            buf.carrets.merge();
            self.push_edit(buf);
        }
    }

    pub fn tab(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        for i in 0..buf.carrets.len() {
            if let Some(line_range) = buf.carrets[i].selected_lines_range(&buf.rope) {
                // TODO: Find a better way to iterate over line of a selection
                for line_idx in line_range.start().index..line_range.end().index + 1 {
                    let line_start: position::Absolute = buf.rope.line_to_byte(line_idx).into();
                    let r = line_start..line_start;
                    let text = match self.file.indentation {
                        Indentation::Space(n) => " ".repeat(n).to_owned(),
                        Indentation::Tab(_) => "\t".to_owned(),
                    };
                    buf.edit(&r, &text, tabsize);
                }
            } else {
                let r = buf.carrets[i].range();
                let text = match self.file.indentation {
                    Indentation::Space(n) => {
                        let start: usize = buf.carrets[i].col().into();
                        let nb_space = n - start % n;
                        " ".repeat(nb_space).to_owned()
                    }
                    Indentation::Tab(_) => "\t".to_owned(),
                };
                buf.edit(&r, &text, tabsize);
                buf.carrets[i].set_index(r.start + Relative::from(text.len()), true, true, &buf.rope, tabsize);
            }
        }
        buf.carrets.merge();
        self.push_edit(buf);
    }

    // position handling
    pub fn point<P>(&self, position: P) -> position::Point
    where
        P: position::Position,
    {
        position.point(&self.buffer.rope, self.file.indentation.visible_len())
    }

    pub fn absolute<P>(&self, position: P) -> position::Absolute
    where
        P: position::Position,
    {
        position.absolute(&self.buffer.rope, self.file.indentation.visible_len())
    }

    pub fn char_to_absolute(&self, index: usize) -> Absolute {
        self.buffer.rope.char_to_byte(index).into()
    }

    fn next_word_boundary<P>(&self, position: P) -> position::Absolute
    where
        P: position::Position,
    {
        let mut i = self.absolute(position).index;
        loop {
            if self.buffer.rope.char(self.char_to_absolute(i).index) == ' ' {
                break;
            }
            let newi = rope_utils::next_grapheme_boundary(&self.buffer.rope.slice(..), i);
            if newi == i {
                break;
            }
            i = newi;
        }
        return i.into();
    }
    fn prev_word_boundary<P>(&self, position: P) -> position::Absolute
    where
        P: position::Position,
    {
        let mut i = self.absolute(position).index;
        loop {
            if self.buffer.rope.char(self.char_to_absolute(i).index) == ' ' {
                break;
            }
            let newi = rope_utils::prev_grapheme_boundary(&self.buffer.rope.slice(..), i);
            if newi == i {
                break;
            }
            i = newi;
        }
        return i.into();
    }


}

#[derive(Debug, Clone)]
pub enum SelectionLineRange {
    Range(Range<usize>),
    RangeTo(RangeTo<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub rope: Rope,
    pub carrets: Carrets,
    uuid: Uuid,
}

impl Data for Buffer {
    fn same(&self, other: &Self) -> bool {
        self.uuid == other.uuid && self.carrets == other.carrets
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            carrets: Carrets::new(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn from_rope(rope: Rope) -> Self {
        Self {
            rope: rope.clone(),
            carrets: Carrets::new(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn edit(&mut self, range: &Range<Absolute>, text: &str, tabsize: usize) {
        let insert_index = self.rope.byte_to_char(range.start.into());
        let end_index = self.rope.byte_to_char(range.end.into());
        let cr = insert_index..end_index;
        self.rope.remove(cr);
        self.rope.insert(insert_index, text);

        for i in 0..self.carrets.len() {
            self.carrets[i].update_after_delete(range.start, (range.end - range.start).into(), &self.rope, tabsize); // TODO verify this
            self.carrets[i].update_after_insert(range.start, text.len().into(), &self.rope, tabsize);
        }
        self.carrets.merge();
        self.uuid = Uuid::new_v4();
    }

    pub fn selected_text(&self, line_feed: LineFeed) -> String {
        let mut s = String::new();
        let multi = self.carrets.len() > 1;
        for c in self.carrets.iter() {
            for chuck in self
                .rope
                .slice(self.rope.byte_to_char(c.start().index)..self.rope.byte_to_char(c.end().index))
                .chunks()
            {
                s.push_str(chuck)
            }
            if multi {
                s.push_str(&line_feed.to_str())
            }
        }
        s
    }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.rope.to_string()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn rope_insert() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("hello world").to_string(), "hello world");
//     }
//     #[test]
//     fn rope_double_insert() {
//         let b = Buffer::new();
//         println!("{:?}", b.insert("hello"));
//         assert_eq!(b.insert("hello").insert(" world").to_string(), "hello world");
//     }
//     #[test]
//     fn rope_backspace() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("hello").backspace().unwrap().to_string(), "hell");
//     }
//     #[test]
//     fn rope_backspace2() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("").backspace().unwrap().to_string(), "");
//     }
//     #[test]
//     fn rope_right() {
//         let b = Buffer::new();
//         let mut b = b.insert("hello\n");
//         b.carrets[0].index = 0;
//         let b = b.forward(false);
//         assert_eq!(b.carrets[0].index, 1);

//         let b = b.forward(false).forward(false).forward(false);
//         assert_eq!(b.carrets[0].index, 4);
//         // move 3 forward, but the last move is ineffective because beyond the rope lenght
//         let b = b.forward(false).forward(false).forward(false);
//         assert_eq!(b.carrets[0].index, 6);
//     }
//     // #[test]
//     // fn rope_forward() {
//     //     let indexes = vec![1usize, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");
//     //     b.carrets[0].index = 0;

//     //     for i in &indexes {
//     //         b = b.forward(false);
//     //         assert_eq!(b.carrets[0].index, *i);
//     //     }
//     //     b.carrets[0].index = 0;

//     //     for i in &indexes {
//     //         b = b.forward(true);
//     //         assert_eq!(b.carrets[0].selection.byte_len, *i);
//     //         assert_eq!(
//     //             b.carrets[0].selection.direction,
//     //             SelectionDirection::Backward
//     //         );
//     //     }
//     // }
//     // #[test]
//     // fn rope_backward() {
//     //     let indexes = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1, 0, 0];
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");

//     //     for i in &indexes {
//     //         b = b.backward(false);
//     //         assert_eq!(b.carrets[0].index, *i);
//     //     }
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");
//     //     let len = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
//     //     for i in &len {
//     //         b = b.backward(true);
//     //         assert_eq!(b.carrets[0].selection.byte_len, *i);
//     //         assert_eq!(
//     //             b.carrets[0].selection.direction,
//     //             SelectionDirection::Forward
//     //         );
//     //     }
//     // }
// }
